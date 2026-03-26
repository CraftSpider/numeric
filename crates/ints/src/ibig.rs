//! An implementation of a 'big integer', optimized for rapid cloning and minimal memory usage.
//!
//! Small values are stored inline, large values are stored in a ref-counted interner.

use crate::big_utils::{
    arr_size, MaybeInline, OutOfRangeError, Side, TaggedOffset, TaggedVal, INT_STORE,
};
use crate::UBig;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt::{Binary, Debug, Display, LowerHex, UpperHex, Write};
use core::{fmt, num, ops, ptr};
use numeric_bits::algos::{
    Add, Algo, AssignBitAlgo, BitAlgo, DivRem, Element, Mul, NewtonRaphson, Shl, Shr, Sub,
};
use numeric_bits::array::*;
use numeric_bits::bit_slice::BitSlice;
use numeric_traits::cast::{FromChecked, FromSaturating, FromStrRadix, FromTruncating};
use numeric_traits::class::{Integral, Numeric, Signed};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::Pow;
use numeric_utils::{static_assert, static_assert_traits};

/// A signed 'big' integer - an unbounded signed value, capable of representing any value up to
/// however many bytes the running computer can reasonably hold in memory.
pub struct IBig(TaggedOffset<2>);

static_assert!(size_of::<IBig>() == size_of::<usize>());
static_assert_traits!(IBig: Send + Sync);

impl IBig {
    #[inline]
    fn val(&self) -> MaybeInline<'_> {
        match self.0.offset() {
            TaggedVal::Inline(val) => MaybeInline::Inline(val),
            TaggedVal::Slice(val) => MaybeInline::Slice(val.get()),
        }
    }

    #[inline]
    fn with_slices<R>(left: &IBig, right: &IBig, f: impl FnOnce(&[usize], &[usize]) -> R) -> R {
        left.with_slice(|left| right.with_slice(|right| f(left, right)))
    }

    /// Create a new `IBig` with the default value of zero
    #[must_use]
    #[inline]
    pub const fn new() -> IBig {
        IBig::new_inline(0, false)
    }

    #[inline]
    const fn new_inline(val: usize, neg: bool) -> IBig {
        IBig(TaggedOffset::new(val, (val != 0 && neg) as usize))
    }

    fn new_intern<V>(val: V, neg: bool) -> IBig
    where
        V: Borrow<[usize]> + Into<Box<[usize]>>,
    {
        let val = INT_STORE.add::<_, [usize]>(val);
        IBig(TaggedOffset::new_ptr(ptr::from_ref(val), usize::from(neg)))
    }

    fn new_slice<V>(val: V, neg: bool) -> IBig
    where
        V: IntSlice<usize> + Borrow<[usize]> + Into<Box<[usize]>>,
    {
        let val = IntSlice::shrink(val);
        if val.len() == 1 && val[0] <= (usize::MAX >> 2) {
            IBig::new_inline(val[0], neg)
        } else {
            IBig::new_intern(val, neg)
        }
    }

    #[inline]
    pub(crate) fn with_slice<R>(&self, f: impl FnOnce(&[usize]) -> R) -> R {
        f(self.val().slice())
    }

    fn write_base<W: Write>(&self, base: usize, w: &mut W, chars: &[char]) -> fmt::Result {
        // This is the simplest way - mod base for digit, div base for next digit
        // It isn't super fast though, so there are probably optimization improvements
        let mut digits = Vec::new();
        let mut scratch = self.clone();

        while scratch > 0 {
            let digit = u8::from_checked(scratch.clone() % base)
                .expect("Mod base should always be less than 255");
            digits.push(digit);
            scratch /= base;
        }

        if digits.is_empty() {
            digits.push(0);
        }

        for d in digits.iter().rev() {
            w.write_char(chars[d as usize])?;
        }
        Ok(())
    }

    /// Check whether this value is stored inline
    #[must_use]
    #[inline]
    pub const fn is_inline(&self) -> bool {
        self.0.inline()
    }

    /// Check whether this value is stored in the global interner
    #[must_use]
    #[inline]
    pub const fn is_interned(&self) -> bool {
        !self.0.inline()
    }

    /// Generate an approximation of this value as a float
    ///
    /// If the value is large, this may return [`f64::INFINITY`] or [`f64::NEG_INFINITY`].
    pub fn approx_float(&self) -> f64 {
        const USIZE_MAX: f64 = usize::MAX as f64;
        self.with_slice(|vals| {
            vals.iter()
                .copied()
                .enumerate()
                .try_fold(0., |acc, (idx, val)| {
                    let val = val as f64;
                    let idx = i32::try_from(idx)?;
                    Ok::<_, num::TryFromIntError>(val.mul_add(USIZE_MAX.powi(idx), acc))
                })
                .unwrap_or(f64::INFINITY)
        })
    }
}

impl Debug for IBig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for IBig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        if self.is_negative() {
            write!(f, "-")?;
        }
        self.write_base(10, f, DIGITS)
    }
}

impl Binary for IBig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_negative() {
            write!(f, "-")?;
        }
        write!(f, "0b")?;
        self.with_slice(|slice| {
            for idx in (0..slice.bit_len()).rev() {
                write!(f, "{}", u8::from(slice.get_bit(idx).unwrap_or(false)))?;
            }
            Ok(())
        })
    }
}

impl UpperHex for IBig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DIGITS: &[char] = &[
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
        ];

        if self.is_negative() {
            write!(f, "-")?;
        }
        write!(f, "0x")?;
        self.write_base(16, f, DIGITS)
    }
}

impl LowerHex for IBig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DIGITS: &[char] = &[
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        if self.is_negative() {
            write!(f, "-")?;
        }
        write!(f, "0x")?;
        self.write_base(16, f, DIGITS)
    }
}

impl Clone for IBig {
    fn clone(&self) -> Self {
        let (val, _) = self.0.get();
        if let TaggedVal::Slice(val) = val {
            val.incr();
        }
        IBig(self.0)
    }
}

impl Drop for IBig {
    fn drop(&mut self) {
        let (val, _) = self.0.get();
        if let TaggedVal::Slice(val) = val {
            val.decr();
        }
    }
}

impl Default for IBig {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for IBig {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            true
        } else if self.0.tags() == other.0.tags() && !self.0.inline() && !other.0.inline() {
            Self::with_slices(self, other, |this, other| this == other)
        } else {
            false
        }
    }
}

impl Eq for IBig {}

impl PartialOrd for IBig {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for IBig {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            return Ordering::Equal;
        } else if self.is_negative() && other.is_positive() {
            return Ordering::Less;
        } else if self.is_positive() && other.is_negative() {
            return Ordering::Greater;
        }

        let out = Self::with_slices(self, other, |this, other| {
            if this.len() != other.len() {
                usize::cmp(&this.len(), &other.len())
            } else {
                this.iter()
                    .zip(other.iter())
                    .find_map(|(l, r)| match l.cmp(r) {
                        Ordering::Equal => None,
                        other => Some(other),
                    })
                    .unwrap_or(Ordering::Equal)
            }
        });

        if self.is_negative() {
            out.reverse()
        } else {
            out
        }
    }
}

macro_rules! impl_assign_for_int {
    ($ty:ty) => {
        impl_assign_for_int!($ty, +, AddAssign, add_assign);
        impl_assign_for_int!($ty, -, SubAssign, sub_assign);
        impl_assign_for_int!($ty, *, MulAssign, mul_assign);
        impl_assign_for_int!($ty, /, DivAssign, div_assign);
        impl_assign_for_int!($ty, %, RemAssign, rem_assign);
    };
    ($ty:ty, $op:tt, $trait:ident, $meth:ident) => {
        impl core::ops::$trait<$ty> for IBig {
            fn $meth(&mut self, other: $ty) {
                *self = &*self $op IBig::from(other);
            }
        }
    };
}

macro_rules! impl_ops_for_int {
    ($ty:ty) => {
        impl_ops_for_int!($ty, +, Add, add);
        impl_ops_for_int!($ty, -, Sub, sub);
        impl_ops_for_int!($ty, *, Mul, mul);
        impl_ops_for_int!($ty, /, Div, div);
        impl_ops_for_int!($ty, %, Rem, rem);

        impl_ops_for_int!($ty, <<, Shl, shl);
        impl_ops_for_int!($ty, >>, Shr, shr);
    };

    ($ty:ty, $op:tt, $trait:ident, $meth:ident) => {
        impl core::ops::$trait<$ty> for IBig {
            type Output = IBig;

            fn $meth(self, other: $ty) -> IBig {
                self $op IBig::from(other)
            }
        }
    };
}

macro_rules! impl_for_int {
    ($signed:ty, $unsigned:ty) => {
        // From/TryFrom

        impl From<$signed> for IBig {
            fn from(val: $signed) -> Self {
                let neg = val.is_negative();
                IBig::new_slice::<&[usize]>(
                    &int_to_arr::<$unsigned, usize, { arr_size::<$unsigned>() }>(
                        val.unsigned_abs(),
                    ),
                    neg,
                )
            }
        }

        impl From<$unsigned> for IBig {
            fn from(val: $unsigned) -> Self {
                IBig::new_slice::<&[usize]>(
                    &int_to_arr::<$unsigned, usize, { arr_size::<$unsigned>() }>(val),
                    false,
                )
            }
        }

        impl TryFrom<IBig> for $signed {
            type Error = OutOfRangeError;

            fn try_from(bi: IBig) -> Result<Self, Self::Error> {
                <$signed as TryFrom<_>>::try_from(&bi)
            }
        }

        impl TryFrom<IBig> for $unsigned {
            type Error = OutOfRangeError;

            fn try_from(bi: IBig) -> Result<Self, Self::Error> {
                <$unsigned as TryFrom<_>>::try_from(&bi)
            }
        }

        impl TryFrom<&IBig> for $signed {
            type Error = OutOfRangeError;

            fn try_from(bi: &IBig) -> Result<Self, Self::Error> {
                if bi > &IBig::from(Self::MAX) {
                    Err(OutOfRangeError::above())
                } else if bi < &IBig::from(Self::MIN) {
                    Err(OutOfRangeError::below())
                } else {
                    bi.with_slice(|s| arr_to_int(s))
                        .ok_or_else(|| OutOfRangeError::above())
                }
            }
        }

        impl TryFrom<&IBig> for $unsigned {
            type Error = OutOfRangeError;

            fn try_from(bi: &IBig) -> Result<Self, Self::Error> {
                if bi > &IBig::from(Self::MAX) {
                    Err(OutOfRangeError::above())
                } else if bi < &IBig::from(Self::MIN) {
                    Err(OutOfRangeError::below())
                } else {
                    bi.with_slice(|s| arr_to_int(s))
                        .ok_or_else(|| OutOfRangeError::above())
                }
            }
        }

        // Casts

        impl numeric_traits::cast::FromTruncating<IBig> for $unsigned {
            fn truncate_from(val: IBig) -> Self {
                val.with_slice(|s| arr_to_int(s))
                    .unwrap_or(<$unsigned>::MAX)
            }
        }

        impl numeric_traits::cast::FromTruncating<IBig> for $signed {
            fn truncate_from(val: IBig) -> Self {
                val.with_slice(|s| arr_to_int(s)).unwrap_or(<$signed>::MAX)
                    * if val.is_negative() { -1 } else { 1 }
            }
        }

        impl numeric_traits::cast::FromChecked<IBig> for $unsigned {
            fn from_checked(val: IBig) -> Option<Self> {
                val.try_into().ok()
            }
        }

        impl numeric_traits::cast::FromChecked<IBig> for $signed {
            fn from_checked(val: IBig) -> Option<Self> {
                val.try_into().ok()
            }
        }

        impl numeric_traits::cast::FromSaturating<IBig> for $unsigned {
            fn saturate_from(val: IBig) -> Self {
                match val.try_into() {
                    Ok(val) => val,
                    Err(OutOfRangeError(Side::Above)) => Self::MAX,
                    Err(OutOfRangeError(Side::Below)) => Self::MIN,
                }
            }
        }

        impl numeric_traits::cast::FromSaturating<IBig> for $signed {
            fn saturate_from(val: IBig) -> Self {
                match val.try_into() {
                    Ok(val) => val,
                    Err(OutOfRangeError(Side::Above)) => Self::MAX,
                    Err(OutOfRangeError(Side::Below)) => Self::MIN,
                }
            }
        }

        impl numeric_traits::cast::FromTruncating<$signed> for IBig {
            fn truncate_from(val: $signed) -> Self {
                IBig::from(val)
            }
        }

        impl numeric_traits::cast::FromTruncating<$unsigned> for IBig {
            fn truncate_from(val: $unsigned) -> Self {
                IBig::from(val)
            }
        }

        impl numeric_traits::cast::FromChecked<$signed> for IBig {
            fn from_checked(val: $signed) -> Option<Self> {
                Some(IBig::from(val))
            }
        }

        impl numeric_traits::cast::FromChecked<$unsigned> for IBig {
            fn from_checked(val: $unsigned) -> Option<Self> {
                Some(IBig::from(val))
            }
        }

        impl numeric_traits::cast::FromSaturating<$signed> for IBig {
            fn saturate_from(val: $signed) -> Self {
                IBig::from(val)
            }
        }

        impl numeric_traits::cast::FromSaturating<$unsigned> for IBig {
            fn saturate_from(val: $unsigned) -> Self {
                IBig::from(val)
            }
        }

        // Comparison

        impl PartialEq<$signed> for IBig {
            fn eq(&self, other: &$signed) -> bool {
                if self.is_negative() != other.is_negative() {
                    return false;
                }
                let other = other.abs();

                self.with_slice(|this| {
                    let arr = int_to_arr::<_, _, { arr_size::<$unsigned>() }>(other as $unsigned);
                    this == IntSlice::shrink(&arr as &[_])
                })
            }
        }

        impl PartialEq<$unsigned> for IBig {
            fn eq(&self, other: &$unsigned) -> bool {
                self.with_slice(|this| {
                    let arr = int_to_arr::<_, _, { arr_size::<$unsigned>() }>(*other);
                    this == IntSlice::shrink(&arr as &[_])
                })
            }
        }

        impl PartialOrd<$signed> for IBig {
            fn partial_cmp(&self, other: &$signed) -> Option<Ordering> {
                Some(IBig::cmp(self, &IBig::from(*other)))
            }
        }

        impl PartialOrd<$unsigned> for IBig {
            fn partial_cmp(&self, other: &$unsigned) -> Option<Ordering> {
                Some(IBig::cmp(self, &IBig::from(*other)))
            }
        }

        // Operations

        impl_ops_for_int!($signed);
        impl_ops_for_int!($unsigned);
        impl_assign_for_int!($signed);
        impl_assign_for_int!($unsigned);
    };
}

impl_for_int!(i8, u8);
impl_for_int!(i16, u16);
impl_for_int!(i32, u32);
impl_for_int!(i64, u64);
impl_for_int!(i128, u128);
impl_for_int!(isize, usize);

impl From<UBig> for IBig {
    fn from(value: UBig) -> Self {
        UBig::with_slice(&value, |slice| IBig::new_slice(slice, false))
    }
}

impl FromChecked<UBig> for IBig {
    fn from_checked(val: UBig) -> Option<Self> {
        Some(Self::from(val))
    }
}

impl FromSaturating<UBig> for IBig {
    fn saturate_from(val: UBig) -> Self {
        Self::from(val)
    }
}

impl FromTruncating<UBig> for IBig {
    fn truncate_from(val: UBig) -> Self {
        Self::from(val)
    }
}

impl_op!(add(self: IBig, rhs) => {
    let (out, neg) = IBig::with_slices(self, rhs, |this, other| {
        match (self.is_positive(), rhs.is_positive()) {
            (true, true) | (false, false) => {
                (<Element as Algo<Add>>::wrapping(this, other), self.is_negative())
            }
            (true, _) => {
                let (mut out, neg): (Vec<_>, _) = <Element as Algo<Sub>>::overflowing(this, other);
                if neg {
                    out.set_bit(0, !out.get_bit(0).unwrap_or(false));
                    <Element as AssignBitAlgo>::not(&mut out);
                }
                (out, neg)
            }
            (_, true) => {
                let (mut out, neg): (Vec<_>, _) = <Element as Algo<Sub>>::overflowing(this, other);
                if neg {
                    out.set_bit(0, !out.get_bit(0).unwrap_or(false));
                    <Element as AssignBitAlgo>::not(&mut out);
                }
                (out, !neg)
            }
        }
    });

    IBig::new_slice(out, neg)
});

impl_op!(mul(self: IBig, rhs) => {
    let out: Vec<_> = IBig::with_slices(self, rhs, |this, other| {
        <Element as Algo<Mul>>::wrapping(this, other)
    });

    IBig::new_slice(out, self.is_negative() != rhs.is_negative())
});

impl_op!(sub(self: IBig, rhs) => {
    let (out, neg) = IBig::with_slices(self, rhs, |this, other| {
        match (self.is_positive(), rhs.is_positive()) {
            (true, false) | (false, true) => {
                let out: Vec<_> = <Element as Algo<Add>>::wrapping(this, other);
                (out, self.is_negative())
            }
            (true, true) => {
                let (out, neg) = <Element as Algo<Sub>>::overflowing(this, other);
                (out, neg)
            }
            (false, false) => {
                let (out, neg) = <Element as Algo<Sub>>::overflowing(this, other);
                (out, !neg)
            }
        }
    });

    IBig::new_slice(out, neg)
});

impl_op!(div(self: IBig, rhs) => {
    let out: Vec<_> = IBig::with_slices(self, rhs, |this, other| {
        <NewtonRaphson as Algo<DivRem>>::wrapping(this, other).0
    });
    IBig::new_slice(out, self.is_negative() != rhs.is_negative())
});

impl_op!(rem(self: IBig, rhs) => {
    let out: Vec<_> = IBig::with_slices(self, rhs, |this, other| {
        <NewtonRaphson as Algo<DivRem>>::wrapping(this, other).1
    });
    IBig::new_slice(out, self.is_negative() != rhs.is_negative())
});

impl_op!(shl(self: IBig, rhs) => {
    let out: Vec<_> = IBig::with_slices(self, rhs, |this, _| {
        <Element as Algo<Shl>>::wrapping::<_, _, [_]>(this, usize::try_from(rhs).expect("Shifts larger than a usize are not yet supported"))
    });
    IBig::new_slice(out, self.is_negative())
});

impl_op!(shr(self: IBig, rhs) => {
    let out: Vec<_> = IBig::with_slices(self, rhs, |this, _| {
        <Element as Algo<Shr>>::wrapping::<_, _, [_]>(this, usize::try_from(rhs).expect("Shifts larger than a usize are not yet supported"))
    });
    IBig::new_slice(out, self.is_negative())
});

impl_op!(bitand(self: IBig, rhs) => {
    let out = IBig::with_slices(self, rhs, |this, other| {
        let len = usize::max(this.len(), other.len());
        let mut out = alloc::vec![0; len];
        <Element as BitAlgo>::and(this, other, &mut out);
        out
    });
    IBig::new_slice(out, self.is_negative())
});

impl_op!(bitor(self: IBig, rhs) => {
    let out = IBig::with_slices(self, rhs, |this, other| {
        let len = usize::max(this.len(), other.len());
        let mut out = alloc::vec![0; len];
        <Element as BitAlgo>::or(this, other, &mut out);
        out
    });
    IBig::new_slice(out, self.is_negative())
});

impl_op!(bitxor(self: IBig, rhs) => {
    let out = IBig::with_slices(self, rhs, |this, other| {
        let len = usize::max(this.len(), other.len());
        let mut out = alloc::vec![0; len];
        <Element as BitAlgo>::xor(this, other, &mut out);
        out
    });
    IBig::new_slice(out, self.is_negative())
});

impl ops::Not for IBig {
    type Output = IBig;

    fn not(self) -> Self::Output {
        let out = IBig::with_slice(&self, |slice| {
            let mut out = slice.to_vec();
            <Element as AssignBitAlgo>::not(&mut out);
            out
        });
        IBig::new_slice(out, self.is_negative())
    }
}

impl ops::Neg for IBig {
    type Output = IBig;

    fn neg(mut self) -> Self::Output {
        self.0 = self.0.invert_tag::<1>();
        self
    }
}

impl ops::Neg for &IBig {
    type Output = IBig;

    fn neg(self) -> Self::Output {
        let mut out = self.clone();
        out.0 = out.0.invert_tag::<1>();
        out
    }
}

impl_assign_op!(add(self: IBig, rhs) => { *self = &*self + rhs });
impl_assign_op!(sub(self: IBig, rhs) => { *self = &*self - rhs });
impl_assign_op!(mul(self: IBig, rhs) => { *self = &*self * rhs });
impl_assign_op!(div(self: IBig, rhs) => { *self = &*self / rhs });
impl_assign_op!(rem(self: IBig, rhs) => { *self = &*self % rhs });
impl_assign_op!(shl(self: IBig, rhs) => { *self = &*self << rhs });
impl_assign_op!(shr(self: IBig, rhs) => { *self = &*self >> rhs });
impl_assign_op!(bitand(self: IBig, rhs) => { *self = &*self & rhs });
impl_assign_op!(bitor(self: IBig, rhs) => { *self = &*self | rhs });
impl_assign_op!(bitxor(self: IBig, rhs) => { *self = &*self ^ rhs });

impl Zero for IBig {
    fn zero() -> Self {
        Self::new()
    }

    fn is_zero(&self) -> bool {
        self.0.get() == (TaggedVal::Inline(0), 0)
    }
}

impl One for IBig {
    fn one() -> Self {
        IBig::new_inline(1, false)
    }

    fn is_one(&self) -> bool {
        self.0.get() == (TaggedVal::Inline(1), 0)
    }
}

/// The error for when you try to create a `IBig` from a string and either the radix is invalid,
/// or the string contains invalid characters.
#[derive(Debug)]
pub enum FromStrError {
    /// Radix was outside the valid range for conversion
    InvalidRadix(u32),
    /// Character wasn't a valid digit for the provided radix
    InvalidChar(char),
}

struct RadixChars;

impl RadixChars {
    fn val_from_char(c: char, radix: u32) -> Result<u32, FromStrError> {
        static INSENS_CHARS: &[char] = &[
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
            'y', 'z',
        ];

        match radix {
            0..=36 => {
                let chars = &INSENS_CHARS[..(radix as usize)];
                chars
                    .iter()
                    .enumerate()
                    .find_map(|(idx, &c2)| {
                        if c2 == c.to_ascii_lowercase() {
                            Some(u32::try_from(idx).unwrap())
                        } else {
                            None
                        }
                    })
                    .ok_or(FromStrError::InvalidChar(c))
            }
            _ => Err(FromStrError::InvalidRadix(radix)),
        }
    }
}

impl FromStrRadix for IBig {
    type Error = FromStrError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::Error> {
        let mut out = IBig::zero();
        for digit in str.chars() {
            let new_val = RadixChars::val_from_char(digit, radix)?;
            out = (out * radix) + new_val;
        }
        Ok(out)
    }
}

impl Numeric for IBig {}

impl Integral for IBig {}

impl Signed for IBig {
    fn abs(self) -> Self {
        if self.is_negative() {
            -self
        } else {
            self
        }
    }

    // fn abs_sub(&self, other: &Self) -> Self {
    //     (self - other).abs()
    // }
    //
    // fn signum(&self) -> Self {
    //     if self.is_zero() {
    //         IBig::from(0)
    //     } else if self.is_negative() {
    //         IBig::from(-1)
    //     } else {
    //         IBig::from(1)
    //     }
    // }

    fn is_positive(&self) -> bool {
        self.0.tags() == 0
    }

    fn is_negative(&self) -> bool {
        self.0.tags() != 0
    }
}

impl Pow<IBig> for IBig {
    type Output = IBig;

    fn pow(self, rhs: IBig) -> Self::Output {
        if rhs == 0 {
            IBig::from(1)
        } else {
            let mut rhs = rhs;
            let mut out = self.clone();
            while rhs > 1 {
                out *= self.clone();
                rhs -= 1;
            }
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use approx::assert_ulps_eq;

    #[test]
    fn test_new() {
        let b0 = IBig::new_slice(&[0usize] as &[_], false);
        assert!(b0.is_inline());
        let b1 = IBig::new_slice(&[usize::MAX >> 2] as &[_], false);
        assert!(b1.is_inline());
        let b2 = IBig::new_slice(&[(usize::MAX >> 2) + 1] as &[_], false);
        assert!(b2.is_interned());
        let b3 = IBig::new_slice(&[0usize, 1] as &[_], false);
        assert!(b3.is_interned());
    }

    #[test]
    fn test_zero() {
        assert!(IBig::zero().is_inline());
        assert!(IBig::zero().with_slice(|s| s == [0]));
        assert!(IBig::zero().is_zero());
        assert!(!IBig::one().is_zero());
    }

    #[test]
    fn test_one() {
        assert!(IBig::one().is_inline());
        assert!(IBig::one().with_slice(|s| s == [1]));
        assert!(IBig::one().is_one());
        assert!(!IBig::one().is_zero());
    }

    #[test]
    fn test_no_neg_zero() {
        assert_eq!(IBig::new_slice(&[0usize] as &[_], true), IBig::from(0));
    }

    #[test]
    fn test_print() {
        assert_eq!(IBig::from(1).to_string(), "1");
        assert_eq!(IBig::from(10).to_string(), "10");
        assert_eq!(IBig::from(111).to_string(), "111");
        assert_eq!(
            IBig::from(18_446_744_073_709_551_616u128).to_string(),
            "18446744073709551616"
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!(IBig::from_str_radix("123", 10).unwrap(), IBig::from(123));
        assert_eq!(IBig::from_str_radix("FF", 16).unwrap(), IBig::from(255));
    }

    #[test]
    fn test_add() {
        assert_eq!(IBig::from(1) + IBig::from(1), IBig::from(2));
        assert_eq!(IBig::from(-10) + IBig::from(5), IBig::from(-5));
        assert_eq!(IBig::from(-10) + IBig::from(15), IBig::from(5));
        assert_eq!(IBig::from(5) + IBig::from(-10), IBig::from(-5));
        assert_eq!(IBig::from(15) + IBig::from(-10), IBig::from(5));
        assert_eq!(IBig::from(-1) + IBig::from(-1), IBig::from(-2));

        assert_eq!(IBig::from(-1) + IBig::from(1), IBig::from(0));
        assert_eq!(IBig::from(1) + IBig::from(-1), IBig::from(0));

        assert_eq!(
            IBig::from(usize::MAX) + IBig::from(usize::MAX),
            IBig::from((usize::MAX as u128) * 2)
        )
    }

    #[test]
    fn test_sub() {
        assert_eq!(IBig::from(1) - IBig::from(1), IBig::from(0));
        assert_eq!(IBig::from(2) - IBig::from(1), IBig::from(1));

        assert_eq!(IBig::from(-1) - IBig::from(1), IBig::from(-2));
        assert_eq!(IBig::from(-1) - IBig::from(-1), IBig::from(0));
    }

    #[test]
    fn test_mul() {
        assert_eq!(IBig::from(0) * IBig::from(1), IBig::from(0));
        assert_eq!(IBig::from(1) * IBig::from(1), IBig::from(1));
        assert_eq!(IBig::from(2) * IBig::from(1), IBig::from(2));
        assert_eq!(IBig::from(2) * IBig::from(2), IBig::from(4));

        assert_eq!(IBig::from(-1) * IBig::from(1), IBig::from(-1));
        assert_eq!(IBig::from(1) * IBig::from(-1), IBig::from(-1));
        assert_eq!(IBig::from(-1) * IBig::from(-1), IBig::from(1));
    }

    #[test]
    fn test_div() {
        assert_eq!(IBig::from(2) / IBig::from(2), IBig::from(1));
        assert_eq!(IBig::from(-2) / IBig::from(2), IBig::from(-1));
        assert_eq!(IBig::from(2) / IBig::from(-2), IBig::from(-1));
        assert_eq!(IBig::from(-2) / IBig::from(-2), IBig::from(1));
        assert_eq!(IBig::from(1) / IBig::from(3), IBig::from(0));
        assert_eq!(
            IBig::new_slice(&[0usize, 0, 1] as &[_], false)
                / IBig::new_slice(&[2usize] as &[_], false),
            IBig::new_slice(&[0usize, (usize::MAX / 2) + 1] as &[_], false),
        );
    }

    #[test]
    fn test_rem() {
        assert_eq!(IBig::from(1) % IBig::from(2), IBig::from(1));
        assert_eq!(IBig::from(2) % IBig::from(2), IBig::from(0));
        assert_eq!(IBig::from(3) % IBig::from(2), IBig::from(1));
        assert_eq!(IBig::from(4) % IBig::from(2), IBig::from(0));
        assert_eq!(IBig::from(usize::MAX) % IBig::from(10), IBig::from(5));
    }

    #[test]
    fn test_shl() {
        assert_eq!(IBig::from(1) << IBig::from(1), IBig::from(2));
        assert_eq!(IBig::from(2) << IBig::from(1), IBig::from(4));
        assert_eq!(IBig::from(3) << IBig::from(1), IBig::from(6));

        assert_eq!(
            IBig::from(usize::MAX) << IBig::from(1),
            IBig::from((usize::MAX as u128) * 2)
        );
    }

    #[test]
    fn test_shr() {
        assert_eq!(IBig::from(2) >> IBig::from(1), IBig::from(1));
        assert_eq!(IBig::from(6) >> IBig::from(1), IBig::from(3));

        assert_eq!(
            IBig::from((usize::MAX as u128) * 2) >> 1,
            IBig::from(usize::MAX)
        );
    }

    #[test]
    fn test_pow() {
        assert_eq!(IBig::from(1).pow(IBig::from(2)), IBig::from(1));
        assert_eq!(IBig::from(2).pow(IBig::from(2)), IBig::from(4));
    }

    #[test]
    fn test_neg_pos() {
        let neg_one = -IBig::one();
        assert!(neg_one.is_inline());
        assert!(neg_one.is_negative());
        assert!(neg_one.with_slice(|s| s == [1]));

        let neg = IBig::from(-0x0102_0304);
        assert!(neg.is_negative());
        let pos = -neg.clone();
        assert!(pos.is_positive());
    }

    #[test]
    fn test_abs() {
        let neg = IBig::from(-0x0102_0304);
        let pos = -neg.clone();

        assert_eq!(neg.abs(), pos);
        assert_eq!(pos.clone().abs(), pos);
    }

    #[test]
    fn test_eq() {
        let a = IBig::from(0);
        let b = IBig::from(1);

        let c = IBig::from(4) % IBig::from(2);

        assert_ne!(a, b);
        assert_eq!(a, c);

        assert_eq!(a, 0i32);
        assert_eq!(b, 1i32);

        assert_ne!(a, 1i32);
        assert_ne!(b, 0i32);

        let big_a = IBig::from(usize::MAX);
        let big_b = IBig::from(usize::MAX - 1);
        let big_c = &big_b + &big_a - &big_b;

        assert_ne!(big_a, big_b);
        assert_eq!(big_a, big_c);

        assert_eq!(big_a, usize::MAX);
        assert_eq!(big_b, usize::MAX - 1);

        assert_ne!(big_a, usize::MAX - 1);
        assert_ne!(big_b, usize::MAX);
    }

    #[test]
    fn test_cmp() {
        let a = IBig::from(0);
        let b = IBig::from(1);
        let c = IBig::from(-1);

        assert!(a < b);
        assert!(a > c);

        assert!(b > c);
        assert!(c < b);

        assert!(a < 1);
        assert!(a > -1);

        assert!(b > 0);

        assert!(c < 0);
    }

    #[test]
    fn test_approx_float() {
        let zero = IBig::zero();
        let one = IBig::one();
        let max_int = IBig::from(9_007_199_254_740_991u64);
        let max_u64 = IBig::from(u64::MAX);
        let pretty_big = max_u64.clone().pow(IBig::from(5));
        let very_big = max_u64.clone().pow(IBig::from(20));

        assert_eq!(zero.approx_float(), 0.0);
        assert_eq!(one.approx_float(), 1.0);
        assert_eq!(max_int.approx_float(), 9_007_199_254_740_991.0);
        assert_ulps_eq!(max_u64.approx_float(), 18_446_744_073_709_552_000.0);
        assert_ulps_eq!(
            pretty_big.approx_float(),
            2.135_987_035_920_91e96,
            max_ulps = 8
        );
        assert_eq!(very_big.approx_float(), f64::INFINITY);
    }
}
