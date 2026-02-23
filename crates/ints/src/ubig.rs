use crate::big_utils::{
    arr_size, MaybeInline, OutOfRangeError, Side, TaggedOffset, TaggedVal, INT_STORE,
};
use crate::IBig;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt::{Debug, Display, Write};
use core::{fmt, num, ops, ptr};
use numeric_bits::algos::{
    AddAlgo, AssignBitAlgo, BitAlgo, Bitwise, DivRemAlgo, Element, MulAlgo, ShlAlgo, ShrAlgo,
    SubAlgo,
};
use numeric_bits::array::{arr_to_int, int_to_arr, IntSlice};
use numeric_traits::cast::{FromChecked, FromSaturating, FromTruncating};
use numeric_traits::class::{Integral, Numeric, Signed, Unsigned};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::Pow;
use numeric_utils::{static_assert, static_assert_traits};

/// An unsigned 'big' integer - an unbounded unsigned value, capable of representing any value up to
/// however many bytes the running computer can reasonably hold in memory.
pub struct UBig(TaggedOffset<1>);

static_assert!(size_of::<UBig>() == size_of::<usize>());
static_assert_traits!(UBig: Send + Sync);

impl UBig {
    #[inline]
    fn val(&self) -> MaybeInline<'_> {
        match self.0.offset() {
            TaggedVal::Inline(val) => MaybeInline::Inline(val),
            TaggedVal::Slice(val) => MaybeInline::Slice(val.get()),
        }
    }

    #[inline]
    fn with_slices<R>(left: &UBig, right: &UBig, f: impl FnOnce(&[usize], &[usize]) -> R) -> R {
        left.with_slice(|left| right.with_slice(|right| f(left, right)))
    }

    /// Create a new `UBig` with the default value of zero
    #[must_use]
    #[inline]
    pub const fn new() -> UBig {
        UBig::new_inline(0)
    }

    #[inline]
    const fn new_inline(val: usize) -> UBig {
        UBig(TaggedOffset::new(val, 0))
    }

    fn new_intern<V>(val: V) -> UBig
    where
        V: Borrow<[usize]> + Into<Box<[usize]>>,
    {
        let val = INT_STORE.add::<_, [usize]>(val);
        UBig(TaggedOffset::new_ptr(ptr::from_ref(val), 0))
    }

    fn new_slice<V>(val: V) -> UBig
    where
        V: IntSlice<usize> + Borrow<[usize]> + Into<Box<[usize]>>,
    {
        let val = IntSlice::shrink(val);
        if val.len() == 1 && val[0] <= (usize::MAX >> 2) {
            UBig::new_inline(val[0])
        } else {
            UBig::new_intern(val)
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

        for &d in digits.iter().rev() {
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
    /// If the value is large, this may return [`f64::INFINITY`].
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

impl Debug for UBig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for UBig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        self.write_base(10, f, DIGITS)
    }
}

impl Clone for UBig {
    fn clone(&self) -> Self {
        let (val, _) = self.0.get();
        if let TaggedVal::Slice(val) = val {
            val.incr();
        }
        UBig(self.0)
    }
}

impl Drop for UBig {
    fn drop(&mut self) {
        let (val, _) = self.0.get();
        if let TaggedVal::Slice(val) = val {
            val.decr();
        }
    }
}

impl Default for UBig {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for UBig {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            true
        } else if !self.0.inline() && !other.0.inline() {
            Self::with_slices(self, other, |this, other| this == other)
        } else {
            false
        }
    }
}

impl Eq for UBig {}

impl PartialOrd for UBig {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for UBig {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            return Ordering::Equal;
        }

        Self::with_slices(self, other, |this, other| {
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
        })
    }
}

macro_rules! impl_assign_for_int {
    ($ty:ty) => {
        impl_assign_for_int!($ty, +, AddAssign, add_assign);
        impl_assign_for_int!($ty, -, SubAssign, sub_assign);
        impl_assign_for_int!($ty, *, MulAssign, mul_assign);
        impl_assign_for_int!($ty, /, DivAssign, div_assign);
        impl_assign_for_int!($ty, %, RemAssign, rem_assign);

        impl_assign_for_int!($ty, <<, ShlAssign, shl_assign);
        impl_assign_for_int!($ty, >>, ShrAssign, shr_assign);
    };
    ($ty:ty, $op:tt, $trait:ident, $meth:ident) => {
        impl core::ops::$trait<$ty> for UBig {
            fn $meth(&mut self, other: $ty) {
                *self = &*self $op UBig::from(other);
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
        impl core::ops::$trait<$ty> for UBig {
            type Output = UBig;

            fn $meth(self, other: $ty) -> UBig {
                self $op UBig::from(other)
            }
        }
    };
}

macro_rules! impl_for_int {
    ($signed:ty, $unsigned:ty) => {
        // From/TryFrom

        impl TryFrom<$signed> for UBig {
            type Error = OutOfRangeError;

            fn try_from(val: $signed) -> Result<Self, Self::Error> {
                if val.is_negative() {
                    Err(OutOfRangeError::below())
                } else {
                    Ok(UBig::new_slice::<&[usize]>(&int_to_arr::<
                        $unsigned,
                        usize,
                        { arr_size::<$unsigned>() },
                    >(
                        val.unsigned_abs()
                    )))
                }
            }
        }

        impl From<$unsigned> for UBig {
            fn from(val: $unsigned) -> Self {
                UBig::new_slice::<&[usize]>(&int_to_arr::<
                    $unsigned,
                    usize,
                    { arr_size::<$unsigned>() },
                >(val))
            }
        }

        impl TryFrom<UBig> for $signed {
            type Error = OutOfRangeError;

            fn try_from(bi: UBig) -> Result<Self, Self::Error> {
                <$signed as TryFrom<_>>::try_from(&bi)
            }
        }

        impl TryFrom<UBig> for $unsigned {
            type Error = OutOfRangeError;

            fn try_from(bi: UBig) -> Result<Self, Self::Error> {
                <$unsigned as TryFrom<_>>::try_from(&bi)
            }
        }

        impl TryFrom<&UBig> for $signed {
            type Error = OutOfRangeError;

            fn try_from(bi: &UBig) -> Result<Self, Self::Error> {
                if bi > &UBig::try_from(Self::MAX).unwrap() {
                    Err(OutOfRangeError::above())
                } else {
                    bi.with_slice(|s| arr_to_int(s))
                        .ok_or_else(|| OutOfRangeError::above())
                }
            }
        }

        impl TryFrom<&UBig> for $unsigned {
            type Error = OutOfRangeError;

            fn try_from(bi: &UBig) -> Result<Self, Self::Error> {
                if bi > &UBig::from(Self::MAX) {
                    Err(OutOfRangeError::above())
                } else {
                    bi.with_slice(|s| arr_to_int(s))
                        .ok_or_else(|| OutOfRangeError::above())
                }
            }
        }

        // Casts

        impl numeric_traits::cast::FromTruncating<UBig> for $unsigned {
            fn truncate_from(val: UBig) -> Self {
                val.with_slice(|s| arr_to_int(s))
                    .unwrap_or(<$unsigned>::MAX)
            }
        }

        impl numeric_traits::cast::FromTruncating<UBig> for $signed {
            fn truncate_from(val: UBig) -> Self {
                val.with_slice(|s| arr_to_int(s)).unwrap_or(<$signed>::MAX)
            }
        }

        impl numeric_traits::cast::FromChecked<UBig> for $unsigned {
            fn from_checked(val: UBig) -> Option<Self> {
                val.try_into().ok()
            }
        }

        impl numeric_traits::cast::FromChecked<UBig> for $signed {
            fn from_checked(val: UBig) -> Option<Self> {
                val.try_into().ok()
            }
        }

        impl numeric_traits::cast::FromSaturating<UBig> for $unsigned {
            fn saturate_from(val: UBig) -> Self {
                match val.try_into() {
                    Ok(val) => val,
                    Err(OutOfRangeError(Side::Above)) => Self::MAX,
                    Err(OutOfRangeError(Side::Below)) => Self::MIN,
                }
            }
        }

        impl numeric_traits::cast::FromSaturating<UBig> for $signed {
            fn saturate_from(val: UBig) -> Self {
                match val.try_into() {
                    Ok(val) => val,
                    Err(OutOfRangeError(Side::Above)) => Self::MAX,
                    Err(OutOfRangeError(Side::Below)) => Self::MIN,
                }
            }
        }

        impl numeric_traits::cast::FromTruncating<$signed> for UBig {
            fn truncate_from(val: $signed) -> Self {
                UBig::from(val as $unsigned)
            }
        }

        impl numeric_traits::cast::FromTruncating<$unsigned> for UBig {
            fn truncate_from(val: $unsigned) -> Self {
                UBig::from(val)
            }
        }

        impl numeric_traits::cast::FromChecked<$signed> for UBig {
            fn from_checked(val: $signed) -> Option<Self> {
                UBig::try_from(val).ok()
            }
        }

        impl numeric_traits::cast::FromChecked<$unsigned> for UBig {
            fn from_checked(val: $unsigned) -> Option<Self> {
                Some(UBig::from(val))
            }
        }

        impl numeric_traits::cast::FromSaturating<$signed> for UBig {
            fn saturate_from(val: $signed) -> Self {
                UBig::try_from(val).unwrap_or(UBig::zero())
            }
        }

        impl numeric_traits::cast::FromSaturating<$unsigned> for UBig {
            fn saturate_from(val: $unsigned) -> Self {
                UBig::from(val)
            }
        }

        // Comparison

        impl PartialEq<$signed> for UBig {
            fn eq(&self, other: &$signed) -> bool {
                if other.is_negative() {
                    return false;
                }
                self.with_slice(|this| {
                    let arr = int_to_arr::<_, _, { arr_size::<$unsigned>() }>(*other as $unsigned);
                    this == IntSlice::shrink(&arr as &[_])
                })
            }
        }

        impl PartialEq<$unsigned> for UBig {
            fn eq(&self, other: &$unsigned) -> bool {
                self.with_slice(|this| {
                    let arr = int_to_arr::<_, _, { arr_size::<$unsigned>() }>(*other);
                    this == IntSlice::shrink(&arr as &[_])
                })
            }
        }

        impl PartialOrd<$signed> for UBig {
            fn partial_cmp(&self, other: &$signed) -> Option<Ordering> {
                Some(match UBig::try_from(*other) {
                    Ok(val) => UBig::cmp(self, &val),
                    Err(_) => Ordering::Greater,
                })
            }
        }

        impl PartialOrd<$unsigned> for UBig {
            fn partial_cmp(&self, other: &$unsigned) -> Option<Ordering> {
                Some(UBig::cmp(self, &UBig::from(*other)))
            }
        }

        // Operations

        impl_ops_for_int!($unsigned);
        impl_assign_for_int!($unsigned);
    };
}

impl_for_int!(i8, u8);
impl_for_int!(i16, u16);
impl_for_int!(i32, u32);
impl_for_int!(i64, u64);
impl_for_int!(i128, u128);
impl_for_int!(isize, usize);

impl TryFrom<IBig> for UBig {
    type Error = OutOfRangeError;

    fn try_from(value: IBig) -> Result<Self, Self::Error> {
        if value.is_negative() {
            Err(OutOfRangeError::below())
        } else {
            Ok(IBig::with_slice(&value, |slice| UBig::new_slice(slice)))
        }
    }
}

impl FromChecked<IBig> for UBig {
    fn from_checked(val: IBig) -> Option<Self> {
        Self::try_from(val).ok()
    }
}

impl FromSaturating<IBig> for UBig {
    fn saturate_from(val: IBig) -> Self {
        Self::try_from(val).unwrap_or(UBig::zero())
    }
}

impl FromTruncating<IBig> for UBig {
    fn truncate_from(val: IBig) -> Self {
        IBig::with_slice(&val, |slice| UBig::new_slice(slice))
    }
}

impl_op!(add(self: UBig, rhs) => {
    let out = UBig::with_slices(self, rhs, |this, other| {
        <Element as AddAlgo>::long(this, other)
    });

    UBig::new_slice(out)
});

impl_op!(mul(self: UBig, rhs) => {
    let out = UBig::with_slices(self, rhs, |this, other| {
        <Element as MulAlgo>::long(this, other)
    });

    UBig::new_slice(out)
});

impl_op!(sub(self: UBig, rhs) => {
    let (out, nonzero) = UBig::with_slices(self, rhs, |this, other| {
        <Element as SubAlgo>::long(this, other)
    });
    if nonzero {
        panic!("Subtraction resulted in negative value for UBig");
    }

    UBig::new_slice(out)
});

impl_op!(div(self: UBig, rhs) => {
    let out = UBig::with_slices(self, rhs, |this, other| {
        <Bitwise as DivRemAlgo>::long(this, other).0
    });
    UBig::new_slice(out)
});

impl_op!(rem(self: UBig, rhs) => {
    let out = UBig::with_slices(self, rhs, |this, other| {
        <Bitwise as DivRemAlgo>::long(this, other).1
    });
    UBig::new_slice(out)
});

impl_op!(shl(self: UBig, rhs) => {
    let out = UBig::with_slices(self, rhs, |this, _| {
        <Element as ShlAlgo>::long(this, usize::try_from(rhs).expect("Shifts larger than a usize are not yet supported"))
    });
    UBig::new_slice(out)
});

impl_op!(shr(self: UBig, rhs) => {
    let out = UBig::with_slices(self, rhs, |this, _| {
        <Element as ShrAlgo>::long(this, usize::try_from(rhs).expect("Shifts larger than a usize are not yet supported"))
    });
    UBig::new_slice(out)
});

impl_op!(bitand(self: UBig, rhs) => {
    let out = UBig::with_slices(self, rhs, |this, other| {
        let len = usize::max(this.len(), other.len());
        let mut out = alloc::vec![0; len];
        <Element as BitAlgo>::and(this, other, &mut out);
        out
    });
    UBig::new_slice(out)
});

impl_op!(bitor(self: UBig, rhs) => {
    let out = UBig::with_slices(self, rhs, |this, other| {
        let len = usize::max(this.len(), other.len());
        let mut out = alloc::vec![0; len];
        <Element as BitAlgo>::or(this, other, &mut out);
        out
    });
    UBig::new_slice(out)
});

impl_op!(bitxor(self: UBig, rhs) => {
    let out = UBig::with_slices(self, rhs, |this, other| {
        let len = usize::max(this.len(), other.len());
        let mut out = alloc::vec![0; len];
        <Element as BitAlgo>::xor(this, other, &mut out);
        out
    });
    UBig::new_slice(out)
});

impl ops::Not for UBig {
    type Output = UBig;

    fn not(self) -> Self::Output {
        let out = UBig::with_slice(&self, |slice| {
            let mut out = slice.to_vec();
            <Element as AssignBitAlgo>::not(&mut out);
            out
        });
        UBig::new_slice(out)
    }
}

impl_assign_op!(add(self: UBig, rhs) => { *self = &*self + rhs });
impl_assign_op!(sub(self: UBig, rhs) => { *self = &*self - rhs });
impl_assign_op!(mul(self: UBig, rhs) => { *self = &*self * rhs });
impl_assign_op!(div(self: UBig, rhs) => { *self = &*self / rhs });
impl_assign_op!(rem(self: UBig, rhs) => { *self = &*self % rhs });
impl_assign_op!(shl(self: UBig, rhs) => { *self = &*self << rhs });
impl_assign_op!(shr(self: UBig, rhs) => { *self = &*self >> rhs });
impl_assign_op!(bitand(self: UBig, rhs) => { *self = &*self & rhs });
impl_assign_op!(bitor(self: UBig, rhs) => { *self = &*self | rhs });
impl_assign_op!(bitxor(self: UBig, rhs) => { *self = &*self ^ rhs });

impl Zero for UBig {
    fn zero() -> Self {
        Self::new()
    }

    fn is_zero(&self) -> bool {
        self.0.get() == (TaggedVal::Inline(0), 0)
    }
}

impl One for UBig {
    fn one() -> Self {
        UBig::new_inline(1)
    }

    fn is_one(&self) -> bool {
        self.0.get() == (TaggedVal::Inline(1), 0)
    }
}

impl Numeric for UBig {}

impl Integral for UBig {}

impl Unsigned for UBig {}

impl Pow<UBig> for UBig {
    type Output = UBig;

    fn pow(self, rhs: UBig) -> Self::Output {
        if rhs == 0 {
            UBig::from(1u32)
        } else {
            let mut rhs = rhs;
            let mut out = self.clone();
            while rhs > 1 {
                out *= self.clone();
                rhs -= 1u32;
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
        let b0 = UBig::new_slice(&[0usize] as &[_]);
        assert!(b0.is_inline());
        let b1 = UBig::new_slice(&[usize::MAX >> 2] as &[_]);
        assert!(b1.is_inline());
        let b2 = UBig::new_slice(&[(usize::MAX >> 2) + 1] as &[_]);
        assert!(b2.is_interned());
        let b3 = UBig::new_slice(&[0usize, 1] as &[_]);
        assert!(b3.is_interned());
    }

    #[test]
    fn test_print() {
        assert_eq!(UBig::from(1u32).to_string(), "1");
        assert_eq!(UBig::from(10u32).to_string(), "10");
        assert_eq!(UBig::from(111u32).to_string(), "111");
        assert_eq!(
            UBig::from(18_446_744_073_709_551_616u128).to_string(),
            "18446744073709551616"
        );
    }

    // #[test]
    // fn test_from_str() {
    //     assert_eq!(UBig::from_str_radix("123", 10).unwrap(), UBig::from(123));
    //     assert_eq!(UBig::from_str_radix("FF", 16).unwrap(), UBig::from(255));
    // }

    #[test]
    fn test_add() {
        assert_eq!(UBig::from(1u32) + UBig::from(1u32), UBig::from(2u32));
        assert_eq!(
            UBig::from(usize::MAX) + UBig::from(usize::MAX),
            UBig::from((usize::MAX as u128) * 2)
        )
    }

    #[test]
    fn test_sub() {
        assert_eq!(UBig::from(1u32) - UBig::from(1u32), UBig::from(0u32));
        assert_eq!(UBig::from(2u32) - UBig::from(1u32), UBig::from(1u32));
    }

    #[test]
    fn test_mul() {
        assert_eq!(UBig::from(0u32) * UBig::from(1u32), UBig::from(0u32));
        assert_eq!(UBig::from(1u32) * UBig::from(1u32), UBig::from(1u32));
        assert_eq!(UBig::from(2u32) * UBig::from(1u32), UBig::from(2u32));
        assert_eq!(UBig::from(2u32) * UBig::from(2u32), UBig::from(4u32));
    }

    #[test]
    fn test_div() {
        assert_eq!(UBig::from(2u32) / UBig::from(2u32), UBig::from(1u32));
        assert_eq!(UBig::from(1u32) / UBig::from(3u32), UBig::from(0u32));
        assert_eq!(
            UBig::new_slice(&[0usize, 0, 1] as &[_]) / UBig::new_slice(&[2usize] as &[_]),
            UBig::new_slice(&[0usize, (usize::MAX / 2) + 1] as &[_]),
        );
    }

    #[test]
    fn test_rem() {
        assert_eq!(UBig::from(1u32) % UBig::from(2u32), UBig::from(1u32));
        assert_eq!(UBig::from(2u32) % UBig::from(2u32), UBig::from(0u32));
        assert_eq!(UBig::from(3u32) % UBig::from(2u32), UBig::from(1u32));
        assert_eq!(UBig::from(4u32) % UBig::from(2u32), UBig::from(0u32));
        assert_eq!(UBig::from(usize::MAX) % UBig::from(10u32), UBig::from(5u32));
    }

    #[test]
    fn test_shl() {
        assert_eq!(UBig::from(1u32) << UBig::from(1u32), UBig::from(2u32));
        assert_eq!(UBig::from(2u32) << UBig::from(1u32), UBig::from(4u32));
        assert_eq!(UBig::from(3u32) << UBig::from(1u32), UBig::from(6u32));

        assert_eq!(
            UBig::from(usize::MAX) << UBig::from(1u32),
            UBig::from((usize::MAX as u128) * 2)
        );
    }

    #[test]
    fn test_shr() {
        assert_eq!(UBig::from(2u32) >> UBig::from(1u32), UBig::from(1u32));
        assert_eq!(UBig::from(6u32) >> UBig::from(1u32), UBig::from(3u32));

        assert_eq!(
            UBig::from((usize::MAX as u128) * 2) >> 1u32,
            UBig::from(usize::MAX)
        );
    }

    #[test]
    fn test_pow() {
        assert_eq!(UBig::from(1u32).pow(UBig::from(2u32)), UBig::from(1u32));
        assert_eq!(UBig::from(2u32).pow(UBig::from(2u32)), UBig::from(4u32));
    }

    #[test]
    fn test_eq() {
        let a = UBig::from(0u32);
        let b = UBig::from(1u32);

        let c = UBig::from(4u32) % UBig::from(2u32);

        assert_ne!(a, b);
        assert_eq!(a, c);

        assert_eq!(a, 0i32);
        assert_eq!(b, 1i32);

        assert_ne!(a, 1i32);
        assert_ne!(b, 0i32);
    }

    #[test]
    fn test_cmp() {
        let a = UBig::from(1u32);
        let b = UBig::from(2u32);
        let c = UBig::from(0u32);

        assert!(a < b);
        assert!(a > c);

        assert!(b > c);
        assert!(c < b);

        assert!(c < 1);
        assert!(a > -1);

        assert!(b > 0);
    }

    #[test]
    fn test_approx_float() {
        let zero = UBig::zero();
        let one = UBig::one();
        let max_int = UBig::from(9007199254740991u64);
        let max_u64 = UBig::from(u64::MAX);
        let pretty_big = max_u64.clone().pow(UBig::from(5u32));
        let very_big = max_u64.clone().pow(UBig::from(20u32));

        assert_eq!(zero.approx_float(), 0.0);
        assert_eq!(one.approx_float(), 1.0);
        assert_eq!(max_int.approx_float(), 9007199254740991.0);
        assert_eq!(max_u64.approx_float(), 18446744073709552000.0);
        assert_ulps_eq!(pretty_big.approx_float(), 2.13598703592091e96);
        assert_eq!(very_big.approx_float(), f64::INFINITY);
    }
}
