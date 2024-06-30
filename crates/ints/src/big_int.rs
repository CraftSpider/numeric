//! An implementation of a 'big integer', optimized for rapid cloning and minimal memory usage.
//!
//! Small values are stored inline, large values are stored in a refcounted interner.

use core::cmp::Ordering;
use core::fmt::{Binary, Debug, Display, LowerHex, UpperHex, Write};
use core::hint::unreachable_unchecked;
use core::{fmt, ops, mem, num};
use core::borrow::Borrow;
use alloc::boxed::Box;
use alloc::vec::Vec;
use numeric_traits::cast::{FromChecked, FromStrRadix};
use numeric_traits::ops::Pow;
use numeric_traits::class::{Signed, Numeric, Integral};
use numeric_traits::identity::{Zero, One};
use numeric_utils::{Interner, static_assert, static_assert_traits};
use numeric_bits::algos::{ElementAdd, ElementSub, ElementMul, ElementShl, ElementShr, ElementBitand, ElementBitor, ElementBitxor, ElementNot, BitwiseDiv};
use numeric_bits::bit_slice::BitSliceExt;
use numeric_bits::utils::*;
use numeric_utils::intern::InternId;

#[macro_use]
mod macros;

static INT_STORE: Interner<Box<[usize]>> = Interner::new();

/// The tag associated with a `TaggedOffset`
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tag {
    None = 0,
    Neg = 1,
    Inline = 2,
    InlineNeg = 3,
}

impl Tag {
    #[must_use]
    #[inline]
    pub const fn from_usize_truncate(val: usize) -> Tag {
        // SAFETY: We truncate val to only contain valid values
        unsafe { Self::from_usize_unsafe(val & 0b11) }
    }

    #[must_use]
    #[inline]
    pub const unsafe fn from_usize_unsafe(val: usize) -> Tag {
        match val {
            0 => Tag::None,
            1 => Tag::Neg,
            2 => Tag::Inline,
            3 => Tag::InlineNeg,
            _ => unreachable_unchecked(),
        }
    }

    #[must_use]
    #[inline]
    pub const fn inline(self) -> bool {
        matches!(self, Tag::Inline | Tag::InlineNeg)
    }

    #[must_use]
    #[inline]
    pub const fn negative(self) -> bool {
        matches!(self, Tag::Neg | Tag::InlineNeg)
    }
}

impl TryFrom<usize> for Tag {
    type Error = ();

    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Tag::None,
            1 => Tag::Neg,
            2 => Tag::Inline,
            3 => Tag::InlineNeg,
            _ => return Err(()),
        })
    }
}

/// An offset containing a `Tag` in its lower two bits
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct TaggedOffset(usize);

impl TaggedOffset {
    #[must_use]
    #[inline]
    pub const fn new(offset: usize, tag: Tag) -> TaggedOffset {
        assert!(offset <= usize::MAX >> 2);
        TaggedOffset((offset << 2) | (tag as usize))
    }

    #[must_use]
    #[inline]
    pub const fn invert_neg(self) -> TaggedOffset {
        TaggedOffset(self.0 ^ 0b1)
    }

    #[must_use]
    #[inline]
    pub const fn get(&self) -> (usize, Tag) {
        (self.offset(), self.tag())
    }

    #[must_use]
    #[inline]
    pub const fn offset(&self) -> usize {
        self.0 >> 2
    }

    #[must_use]
    #[inline]
    pub const fn tag(&self) -> Tag {
        Tag::from_usize_truncate(self.0)
    }
}

enum MaybeInline<'a> {
    Inline(usize),
    Slice(&'a [usize]),
}

impl MaybeInline<'_> {
    #[inline]
    const fn slice(&self) -> &[usize] {
        match self {
            MaybeInline::Inline(i) => core::slice::from_ref(i),
            MaybeInline::Slice(s) => s,
        }
    }
}

/// A 'big' integer - an unbounded signed value, capable of representing any value up to however
/// many bytes the running computer can reasonably hold in memory.
pub struct BigInt(TaggedOffset);

static_assert!(mem::size_of::<BigInt>() == mem::size_of::<usize>());
static_assert_traits!(BigInt: Send + Sync);

impl BigInt {
    #[inline]
    fn val(&self) -> MaybeInline<'_> {
        if self.is_inline() {
            MaybeInline::Inline(self.0.offset())
        } else {
            MaybeInline::Slice(INT_STORE.get(InternId::from_usize(self.0.offset())))
        }
    }

    #[inline]
    fn with_slices<R>(left: &BigInt, right: &BigInt, f: impl FnOnce(&[usize], &[usize]) -> R) -> R {
        left.with_slice(|left| right.with_slice(|right| f(left, right)))
    }

    /// Create a new `BigInt` with the default value of zero
    #[must_use]
    #[inline]
    pub const fn new() -> BigInt {
        BigInt::new_inline(0, false)
    }

    #[inline]
    const fn new_inline(val: usize, neg: bool) -> BigInt {
        BigInt(TaggedOffset::new(val, if val != 0 && neg { Tag::InlineNeg } else { Tag::Inline }))
    }

    fn new_intern<V>(val: V, neg: bool) -> BigInt
    where
        V: Borrow<[usize]> + Into<Box<[usize]>>,
    {
        let offset = INT_STORE.add::<_, [usize]>(val);
        BigInt(TaggedOffset::new(offset.into_usize(), if neg { Tag::Neg } else { Tag::None }))
    }

    fn new_slice<V>(val: V, neg: bool) -> BigInt
    where
        V: IntSlice<usize> + Borrow<[usize]> + Into<Box<[usize]>>,
    {
        let val = IntSlice::shrink(val);
        if val.len() == 1 && val[0] <= (usize::MAX >> 2) {
            BigInt::new_inline(val[0], neg)
        } else {
            BigInt::new_intern(val, neg)
        }
    }

    #[inline]
    fn with_slice<R>(&self, f: impl FnOnce(&[usize]) -> R) -> R {
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
        self.0.tag().inline()
    }

    /// Check whether this value is stored in the global interner
    #[must_use]
    #[inline]
    pub const fn is_interned(&self) -> bool {
        !self.0.tag().inline()
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

impl Debug for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        if self.is_negative() {
            write!(f, "-")?;
        }
        self.write_base(10, f, DIGITS)
    }
}

impl Binary for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_negative() {
            write!(f, "-")?;
        }
        write!(f, "0b")?;
        self.with_slice(|slice| {
            for idx in (0..slice.bit_len()).rev() {
                write!(f, "{}", u8::from(slice.get_bit(idx)))?;
            }
            Ok(())
        })
    }
}

impl UpperHex for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DIGITS: &[char] = &[
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'
        ];

        if self.is_negative() {
            write!(f, "-")?;
        }
        write!(f, "0x")?;
        self.write_base(16, f, DIGITS)
    }
}

impl LowerHex for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DIGITS: &[char] = &[
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'
        ];

        if self.is_negative() {
            write!(f, "-")?;
        }
        write!(f, "0x")?;
        self.write_base(16, f, DIGITS)
    }
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        let (val, tag) = self.0.get();
        if !tag.inline() {
            INT_STORE.incr(InternId::from_usize(val));
        }
        BigInt(self.0)
    }
}

impl Drop for BigInt {
    fn drop(&mut self) {
        let (val, tag) = self.0.get();
        if !tag.inline() {
            INT_STORE.decr(InternId::from_usize(val));
        }
    }
}

impl Default for BigInt {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            true
        } else if self.0.tag() == other.0.tag() && !self.0.tag().inline() {
            Self::with_slices(self, other, |this, other| this == other)
        } else {
            false
        }
    }
}

impl Eq for BigInt {}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for BigInt {
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
                    .find_map(|(l, r)| {
                        match l.cmp(r) {
                            Ordering::Equal => None,
                            other => Some(other),
                        }
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

#[derive(Debug)]
enum Side {
    Above,
    Below,
}

/// The error for when you try to convert a `BigInt` with a value that is too large or small for
/// the type being converted into.
#[derive(Debug)]
pub struct OutOfRangeError(Side);

impl OutOfRangeError {
    fn above() -> Self {
        Self(Side::Above)
    }

    fn below() -> Self {
        Self(Side::Below)
    }
}

const fn arr_size<T>() -> usize {
    (mem::size_of::<T>() / mem::size_of::<usize>()) + 1
}

impl_for_int!(i8, u8);
impl_for_int!(i16, u16);
impl_for_int!(i32, u32);
impl_for_int!(i64, u64);
impl_for_int!(i128, u128);
impl_for_int!(isize, usize);

impl_op!(add(self, rhs) => {
    let (out, neg) = BigInt::with_slices(self, rhs, |this, other| {
        match (self.is_positive(), rhs.is_positive()) {
            (true, true) | (false, false) => {
                (ElementAdd::add(this, other), self.is_negative())
            }
            (true, _) => {
                let (out, neg) = ElementSub::sub(this, other);
                (out, neg)
            }
            (_, true) => {
                let (out, neg) = ElementSub::sub(this, other);
                (out, !neg)
            }
        }
    });

    BigInt::new_slice(out, neg)
});

impl_op!(mul(self, rhs) => {
    let out = BigInt::with_slices(self, rhs, |this, other| {
        ElementMul::mul(this, other)
    });

    BigInt::new_slice(out, self.is_negative() != rhs.is_negative())
});

impl_op!(sub(self, rhs) => {
    let (out, neg) = BigInt::with_slices(self, rhs, |this, other| {
        match (self.is_positive(), rhs.is_positive()) {
            (true, false) | (false, true) => {
                let out = ElementAdd::add(this, other);
                (out, self.is_negative())
            }
            (true, true) => {
                let (out, neg) = ElementSub::sub(this, other);
                (out, neg)
            }
            (false, false) => {
                let (out, neg) = ElementSub::sub(this, other);
                (out, !neg)
            }
        }
    });

    BigInt::new_slice(out, neg)
});

impl_op!(div(self, rhs) => {
    let out = BigInt::with_slices(self, rhs, |this, other| {
        BitwiseDiv::div_long(this, other).0
    });
    BigInt::new_slice(out, self.is_negative() != rhs.is_negative())
});

impl_op!(rem(self, rhs) => {
    let out = BigInt::with_slices(self, rhs, |this, other| {
        BitwiseDiv::div_long(this, other).1
    });
    BigInt::new_slice(out, self.is_negative() != rhs.is_negative())
});

impl_op!(shl(self, rhs) => {
    let out = BigInt::with_slices(self, rhs, |this, _| {
        ElementShl::shl(this, usize::try_from(rhs).expect("Shifts larger than a usize are not yet supported"))
    });
    BigInt::new_slice(out, self.is_negative())
});

impl_op!(shr(self, rhs) => {
    let out = BigInt::with_slices(self, rhs, |this, _| {
        ElementShr::shr_wrap_and_mask(this, usize::try_from(rhs).expect("Shifts larger than a usize are not yet supported"))
    });
    BigInt::new_slice(out, self.is_negative())
});

impl_op!(bitand(self, rhs) => {
    let out = BigInt::with_slices(self, rhs, |this, other| {
        ElementBitand::bitand(this, other)
    });
    BigInt::new_slice(out, self.is_negative())
});

impl_op!(bitor(self, rhs) => {
    let out = BigInt::with_slices(self, rhs, |this, other| {
        ElementBitor::bitor(this, other)
    });
    BigInt::new_slice(out, self.is_negative())
});

impl_op!(bitxor(self, rhs) => {
    let out = BigInt::with_slices(self, rhs, |this, other| {
        ElementBitxor::bitxor(this, other)
    });
    BigInt::new_slice(out, self.is_negative())
});

impl ops::Not for BigInt {
    type Output = BigInt;

    fn not(self) -> Self::Output {
        let out = BigInt::with_slice(&self, |slice| {
            let mut out = slice.to_vec();
            ElementNot::not(&mut out);
            out
        });
        BigInt::new_slice(out, self.is_negative())
    }
}

impl ops::Neg for BigInt {
    type Output = BigInt;

    fn neg(mut self) -> Self::Output {
        self.0 = self.0.invert_neg();
        self
    }
}

impl ops::Neg for &BigInt {
    type Output = BigInt;

    fn neg(self) -> Self::Output {
        let mut out = self.clone();
        out.0 = out.0.invert_neg();
        out
    }
}

macro_rules! impl_assign_op {
    (add($self:ident, $rhs:ident) => $block:block) => {
        impl_assign_op!(add_assign, AddAssign, $self, $rhs, $block);
    };
    (sub($self:ident, $rhs:ident) => $block:block) => {
        impl_assign_op!(sub_assign, SubAssign, $self, $rhs, $block);
    };
    (mul($self:ident, $rhs:ident) => $block:block) => {
        impl_assign_op!(mul_assign, MulAssign, $self, $rhs, $block);
    };
    (div($self:ident, $rhs:ident) => $block:block) => {
        impl_assign_op!(div_assign, DivAssign, $self, $rhs, $block);
    };
    (rem($self:ident, $rhs:ident) => $block:block) => {
        impl_assign_op!(rem_assign, RemAssign, $self, $rhs, $block);
    };
    ($meth:ident, $trait:ident, $self:ident, $rhs:ident, $block:block) => {
        impl core::ops::$trait<BigInt> for BigInt {
            fn $meth(&mut self, rhs: BigInt) {
                <BigInt as core::ops::$trait<&BigInt>>::$meth(self, &rhs)
            }
        }

        impl core::ops::$trait<&BigInt> for BigInt {
            fn $meth(&mut $self, $rhs: &BigInt) $block
        }
    };
}

impl_assign_op!(add(self, rhs) => { *self = &*self + rhs });
impl_assign_op!(sub(self, rhs) => { *self = &*self - rhs });
impl_assign_op!(mul(self, rhs) => { *self = &*self * rhs });
impl_assign_op!(div(self, rhs) => { *self = &*self / rhs });
impl_assign_op!(rem(self, rhs) => { *self = &*self % rhs });

impl Zero for BigInt {
    fn zero() -> Self {
        Self::new()
    }

    fn is_zero(&self) -> bool {
        self.0.get() == (0, Tag::Inline)
    }
}

impl One for BigInt {
    fn one() -> Self {
        BigInt::new_inline(1, false)
    }

    fn is_one(&self) -> bool {
        self.0.get() == (1, Tag::Inline)
    }
}

/// The error for when you try to create a `BigInt` from a string and either the radix is invalid,
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
                chars.iter()
                    .enumerate()
                    .find_map(|(idx, &c2)| if c2 == c.to_ascii_lowercase() {
                        Some(u32::try_from(idx).unwrap())
                    } else {
                        None
                    })
                    .ok_or(FromStrError::InvalidChar(c))
            }
            _ => Err(FromStrError::InvalidRadix(radix)),
        }
    }
}

impl FromStrRadix for BigInt {
    type Error = FromStrError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::Error> {
        let mut out = BigInt::zero();
        for digit in str.chars() {
            let new_val = RadixChars::val_from_char(digit, radix)?;
            out = (out * radix) + new_val;
        }
        Ok(out)
    }
}

impl Numeric for BigInt {}

impl Integral for BigInt {}

impl Signed for BigInt {
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
    //         BigInt::from(0)
    //     } else if self.is_negative() {
    //         BigInt::from(-1)
    //     } else {
    //         BigInt::from(1)
    //     }
    // }

    fn is_positive(&self) -> bool {
        !self.0.tag().negative()
    }

    fn is_negative(&self) -> bool {
        self.0.tag().negative()
    }
}

impl Pow<BigInt> for BigInt {
    type Output = BigInt;

    fn pow(self, rhs: BigInt) -> Self::Output {
        if rhs == 0 {
            BigInt::from(1)
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

    #[test]
    fn test_new() {
        let b0 = BigInt::new_slice(&[0usize] as &[_], false);
        assert!(b0.is_inline());
        let b1 = BigInt::new_slice(&[usize::MAX >> 2] as &[_], false);
        assert!(b1.is_inline());
        let b2 = BigInt::new_slice(&[(usize::MAX >> 2) + 1] as &[_], false);
        assert!(b2.is_interned());
        let b3 = BigInt::new_slice(&[0usize, 1] as &[_], false);
        assert!(b3.is_interned());
    }

    #[test]
    fn test_no_neg_zero() {
        assert_eq!(BigInt::new_slice(&[0usize] as &[_], true), BigInt::from(0));
    }

    #[test]
    fn test_print() {
        assert_eq!(BigInt::from(1).to_string(), "1");
        assert_eq!(BigInt::from(10).to_string(), "10");
        assert_eq!(BigInt::from(111).to_string(), "111");
        assert_eq!(BigInt::from(18446744073709551616u128).to_string(), "18446744073709551616");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(BigInt::from_str_radix("123", 10).unwrap(), BigInt::from(123));
        assert_eq!(BigInt::from_str_radix("FF", 16).unwrap(), BigInt::from(255));
    }

    #[test]
    fn test_add() {
        assert_eq!(BigInt::from(1) + BigInt::from(1), BigInt::from(2));
        assert_eq!(BigInt::from(-10) + BigInt::from(5), BigInt::from(-5));
        assert_eq!(BigInt::from(-10) + BigInt::from(15), BigInt::from(5));
        assert_eq!(BigInt::from(5) + BigInt::from(-10), BigInt::from(-5));
        assert_eq!(BigInt::from(15) + BigInt::from(-10), BigInt::from(5));
        assert_eq!(BigInt::from(-1) + BigInt::from(-1), BigInt::from(-2));

        assert_eq!(BigInt::from(-1) + BigInt::from(1), BigInt::from(0));
        assert_eq!(BigInt::from(1) + BigInt::from(-1), BigInt::from(0));

        assert_eq!(BigInt::from(usize::MAX) + BigInt::from(usize::MAX), BigInt::from((usize::MAX as u128) * 2))
    }

    #[test]
    fn test_sub() {
        assert_eq!(BigInt::from(1) - BigInt::from(1), BigInt::from(0));
        assert_eq!(BigInt::from(2) - BigInt::from(1), BigInt::from(1));

        assert_eq!(BigInt::from(-1) - BigInt::from(1), BigInt::from(-2));
        assert_eq!(BigInt::from(-1) - BigInt::from(-1), BigInt::from(0));
    }

    #[test]
    fn test_mul() {
        assert_eq!(BigInt::from(0) * BigInt::from(1), BigInt::from(0));
        assert_eq!(BigInt::from(1) * BigInt::from(1), BigInt::from(1));
        assert_eq!(BigInt::from(2) * BigInt::from(1), BigInt::from(2));
        assert_eq!(BigInt::from(2) * BigInt::from(2), BigInt::from(4));

        assert_eq!(BigInt::from(-1) * BigInt::from(1), BigInt::from(-1));
        assert_eq!(BigInt::from(1) * BigInt::from(-1), BigInt::from(-1));
        assert_eq!(BigInt::from(-1) * BigInt::from(-1), BigInt::from(1));
    }

    #[test]
    fn test_div() {
        assert_eq!(BigInt::from(2) / BigInt::from(2), BigInt::from(1));
        assert_eq!(BigInt::from(-2) / BigInt::from(2), BigInt::from(-1));
        assert_eq!(BigInt::from(2) / BigInt::from(-2), BigInt::from(-1));
        assert_eq!(BigInt::from(-2) / BigInt::from(-2), BigInt::from(1));
        assert_eq!(BigInt::from(1) / BigInt::from(3), BigInt::from(0));
        assert_eq!(
            BigInt::new_slice(&[0usize, 0, 1] as &[_], false) / BigInt::new_slice(&[2usize] as &[_], false),
            BigInt::new_slice(&[0usize, (usize::MAX / 2) + 1] as &[_], false),
        );
    }

    #[test]
    fn test_rem() {
        assert_eq!(BigInt::from(1) % BigInt::from(2), BigInt::from(1));
        assert_eq!(BigInt::from(2) % BigInt::from(2), BigInt::from(0));
        assert_eq!(BigInt::from(3) % BigInt::from(2), BigInt::from(1));
        assert_eq!(BigInt::from(4) % BigInt::from(2), BigInt::from(0));
        assert_eq!(BigInt::from(usize::MAX) % BigInt::from(10), BigInt::from(5));
    }

    #[test]
    fn test_shl() {
        assert_eq!(BigInt::from(1) << BigInt::from(1), BigInt::from(2));
        assert_eq!(BigInt::from(2) << BigInt::from(1), BigInt::from(4));
        assert_eq!(BigInt::from(3) << BigInt::from(1), BigInt::from(6));

        assert_eq!(BigInt::from(usize::MAX) << BigInt::from(1), BigInt::from((usize::MAX as u128) * 2));
    }

    #[test]
    fn test_pow() {
        assert_eq!(BigInt::from(1).pow(BigInt::from(2)), BigInt::from(1));
        assert_eq!(BigInt::from(2).pow(BigInt::from(2)), BigInt::from(4));
    }

    #[test]
    fn test_eq() {
        let a = BigInt::from(0);
        let b = BigInt::from(1);

        let c = BigInt::from(4) % BigInt::from(2);

        assert_ne!(a, b);
        assert_eq!(a, c);

        assert_eq!(a, 0i32);
        assert_eq!(b, 1i32);

        assert_ne!(a, 1i32);
        assert_ne!(b, 0i32);
    }

    #[test]
    fn test_cmp() {
        let a = BigInt::from(0);
        let b = BigInt::from(1);
        let c = BigInt::from(-1);

        assert!(a < b);
        assert!(a > c);

        assert!(b > c);
        assert!(c < b);

        assert!(a < 1);
        assert!(a > -1);

        assert!(b > 0);

        assert!(c < 0);
    }
}
