use std::cmp::Ordering;
use std::fmt::{Binary, Debug, Display, LowerHex, UpperHex, Write};
use std::hint::unreachable_unchecked;
use std::{fmt, ops};
use num_traits::{FromPrimitive, Num, One, Pow, Signed, ToPrimitive, Zero};

use crate::bit_slice::BitSlice;
use crate::intern::{Interner, SliceHack};
use crate::utils::*;

static INT_STORE: Interner<Box<[usize]>> = Interner::new();

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tag {
    None = 0,
    Neg = 1,
    Inline = 2,
    InlineNeg = 3,
}

impl Tag {
    #[must_use]
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
    pub fn inline(self) -> bool {
        matches!(self, Tag::Inline | Tag::InlineNeg)
    }

    #[must_use]
    pub fn negative(self) -> bool {
        matches!(self, Tag::Neg | Tag::InlineNeg)
    }
}

impl TryFrom<usize> for Tag {
    type Error = ();

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TaggedOffset(usize);

impl TaggedOffset {
    #[must_use]
    pub const fn new(offset: usize, tag: Tag) -> TaggedOffset {
        assert!(offset <= usize::MAX >> 2);
        TaggedOffset((offset << 2) | (tag as usize))
    }

    #[must_use]
    pub const fn invert_neg(self) -> TaggedOffset {
        TaggedOffset(self.0 ^ 0b1)
    }

    #[must_use]
    pub const fn get(&self) -> (usize, Tag) {
        (self.offset(), self.tag())
    }

    #[must_use]
    pub const fn offset(&self) -> usize {
        self.0 >> 2
    }

    #[must_use]
    pub const fn tag(&self) -> Tag {
        unsafe { Tag::from_usize_unsafe(self.0 & 0b11) }
    }
}

pub struct BigInt(TaggedOffset);

impl BigInt {
    fn with_slices<R>(left: &BigInt, right: &BigInt, f: impl FnOnce(BitSlice<&[usize]>, BitSlice<&[usize]>) -> R) -> R {
        left.with_slice(|left| right.with_slice(|right| f(left, right)))
    }

    #[must_use]
    pub fn new() -> BigInt {
        Self::zero()
    }

    const fn new_inline(val: usize, neg: bool) -> BigInt {
        BigInt(TaggedOffset::new(val, if neg { Tag::InlineNeg } else { Tag::Inline }))
    }

    fn new_intern(val: &[usize], neg: bool) -> BigInt {
        let offset = INT_STORE.add(SliceHack(val));
        BigInt(TaggedOffset::new(offset, if neg { Tag::Neg } else { Tag::None }))
    }

    fn new_slice(val: &[usize], neg: bool) -> BigInt {
        let val = shrink_slice(val);
        if val.len() == 1 && val[0] <= (usize::MAX >> 2) {
            BigInt::new_inline(val[0], neg)
        } else {
            BigInt::new_intern(val, neg)
        }
    }

    fn with_slice<R>(&self, f: impl FnOnce(BitSlice<&[usize]>) -> R) -> R {
        if self.0.tag().inline() {
            f(BitSlice::new(&[self.0.offset()]))
        } else {
            f(BitSlice::new(INT_STORE.get(self.0.offset()).val()))
        }
    }

    fn write_base<W: Write>(&self, base: usize, w: &mut W, chars: &[char]) -> fmt::Result {
        // This is the simplest way - mod base for digit, div base for next digit
        // It isn't super fast though, so there are probably optimization improvements
        let mut digits = Vec::new();
        let mut scratch = self.clone();

        while scratch > 0 {
            let digit = (scratch.clone() % base)
                .to_u8()
                .expect("Mod base should always be less than 255");
            digits.push(digit);
            scratch = scratch / base;
        }

        if digits.is_empty() {
            digits.push(0);
        }

        for &d in digits.iter().rev() {
            w.write_char(chars[d as usize])?;
        }
        Ok(())
    }

    #[must_use]
    pub fn is_inline(&self) -> bool {
        self.0.tag().inline()
    }

    #[must_use]
    pub fn is_interned(&self) -> bool {
        !self.0.tag().inline()
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
        self.write_base(10, f, &DIGITS)
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
                write!(f, "{}", slice.get_bit(idx) as u8)?;
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
        self.write_base(16, f, &DIGITS)
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
        self.write_base(16, f, &DIGITS)
    }
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        let (val, tag) = self.0.get();
        if tag.inline() {
            BigInt(self.0)
        } else {
            INT_STORE.incr(val);
            BigInt(self.0)
        }
    }
}

impl Drop for BigInt {
    fn drop(&mut self) {
        let (val, tag) = self.0.get();
        if !tag.inline() {
            INT_STORE.decr(val);
        }
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
        }

        Self::with_slices(self, other, |this, other| {
            if this.len() != other.len() {
                usize::cmp(&this.len(), &other.len())
            } else {
                this.inner()
                    .iter()
                    .zip(other.inner().iter())
                    .find_map(|(l, r)| {
                        match l.cmp(r) {
                            Ordering::Equal => None,
                            other => Some(other),
                        }
                    })
                    .unwrap_or(Ordering::Equal)
            }
        })
    }
}

#[derive(Debug)]
pub struct OutOfRangeError;

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
        impl core::ops::$trait<$ty> for BigInt {
            type Output = BigInt;

            fn $meth(self, other: $ty) -> BigInt {
                self $op BigInt::from(other)
            }
        }
    };
}

macro_rules! impl_for_int {
    ($signed:ty, $unsigned:ty) => {
        impl From<$signed> for BigInt {
            fn from(val: $signed) -> Self {
                let neg = val.is_negative();
                BigInt::new_slice(&int_to_arr(val.abs() as $unsigned), neg)
            }
        }

        impl From<$unsigned> for BigInt {
            fn from(val: $unsigned) -> Self {
                BigInt::new_slice(&int_to_arr(val), false)
            }
        }

        impl TryFrom<&BigInt> for $signed {
            type Error = OutOfRangeError;

            fn try_from(bi: &BigInt) -> Result<Self, Self::Error> {
                if bi > &BigInt::from(Self::MAX) || bi < &BigInt::from(Self::MIN) {
                    Err(OutOfRangeError)
                } else {
                    bi.with_slice(|s| arr_to_int(s.inner())).ok_or(OutOfRangeError)
                }
            }
        }

        impl TryFrom<&BigInt> for $unsigned {
            type Error = OutOfRangeError;

            fn try_from(bi: &BigInt) -> Result<Self, Self::Error> {
                if bi > &BigInt::from(Self::MAX) || bi < &BigInt::from(Self::MIN) {
                    Err(OutOfRangeError)
                } else {
                    bi.with_slice(|s| arr_to_int(s.inner())).ok_or(OutOfRangeError)
                }
            }
        }

        impl PartialEq<$signed> for BigInt {
            fn eq(&self, other: &$signed) -> bool {
                if self.is_negative() != other.is_negative() {
                    return false;
                }
                let other = other.abs();

                self.with_slice(|this| {
                    this == BitSlice::new(int_to_arr(other as $unsigned))
                })
            }
        }

        impl PartialEq<$unsigned> for BigInt {
            fn eq(&self, other: &$unsigned) -> bool {
                self.with_slice(|this| {
                    this == BitSlice::new(int_to_arr(*other))
                })
            }
        }

        impl PartialOrd<$signed> for BigInt {
            fn partial_cmp(&self, other: &$signed) -> Option<Ordering> {
                Some(BigInt::cmp(self, &BigInt::from(*other)))
            }
        }

        impl PartialOrd<$unsigned> for BigInt {
            fn partial_cmp(&self, other: &$unsigned) -> Option<Ordering> {
                Some(BigInt::cmp(self, &BigInt::from(*other)))
            }
        }

        impl_ops_for_int!($signed);
        impl_ops_for_int!($unsigned);
    }
}

impl_for_int!(i8, u8);
impl_for_int!(i16, u16);
impl_for_int!(i32, u32);
impl_for_int!(i64, u64);
impl_for_int!(i128, u128);
impl_for_int!(isize, usize);

macro_rules! impl_op {
    (add($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(add, Add, $self, $rhs, $block);
    };
    (sub($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(sub, Sub, $self, $rhs, $block);
    };
    (mul($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(mul, Mul, $self, $rhs, $block);
    };
    (div($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(div, Div, $self, $rhs, $block);
    };
    (rem($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(rem, Rem, $self, $rhs, $block);
    };
    (shl($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(shl, Shl, $self, $rhs, $block);
    };
    (shr($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(shr, Shr, $self, $rhs, $block);
    };
    ($meth:ident, $trait:ident, $self:ident, $rhs:ident, $block:block) => {
        impl core::ops::$trait<BigInt> for BigInt {
            type Output = BigInt;

            fn $meth(self, rhs: BigInt) -> Self::Output {
                <&BigInt as core::ops::$trait<&BigInt>>::$meth(&self, &rhs)
            }
        }

        impl core::ops::$trait<&BigInt> for BigInt {
            type Output = BigInt;

            fn $meth(self, rhs: &BigInt) -> Self::Output {
                <&BigInt as core::ops::$trait<&BigInt>>::$meth(&self, rhs)
            }
        }

        impl core::ops::$trait<BigInt> for &BigInt {
            type Output = BigInt;

            fn $meth(self, rhs: BigInt) -> Self::Output {
                <&BigInt as core::ops::$trait<&BigInt>>::$meth(self, &rhs)
            }
        }

        impl core::ops::$trait<&BigInt> for &BigInt {
            type Output = BigInt;

            fn $meth($self, $rhs: &BigInt) -> Self::Output $block
        }
    };
}

impl_op!(add(self, rhs) => {
    let (out, neg) = BigInt::with_slices(self, rhs, |this, other| {
        if self.is_negative() == rhs.is_negative() {
            ((this + other).into_inner(), self.is_negative())
        } else if self > rhs {
            let (out, neg) = this - other;
            (out.into_inner(), neg != self.is_negative())
        } else {
            let (out, neg) = this - other;
            (out.into_inner(), neg == rhs.is_negative())
        }
    });

    BigInt::new_slice(&out, neg)
});

impl_op!(sub(self, rhs) => {
    let (out, neg) = BigInt::with_slices(self, rhs, |this, other| {
        if self.is_negative() == rhs.is_negative() {
            if self > rhs {
                let (out, neg) = this - other;
                (out.into_inner(), neg == self.is_negative())
            } else {
                let (out, neg) = this - other;
                (out.into_inner(), neg == rhs.is_negative())
            }
        } else {
            ((this + other).into_inner(), self.is_negative())
        }
    });

    BigInt::new_slice(&out, neg)
});

impl_op!(mul(self, rhs) => {
    let (out, neg) = BigInt::with_slices(self, rhs, |this, other| {
        (this * other, self.is_negative() != rhs.is_negative())
    });

    BigInt::new_slice(&out.into_inner(), neg)
});

impl_op!(div(self, rhs) => {
    let out = BigInt::with_slices(self, rhs, |this, other| {
        (this / other).into_inner()
    });
    BigInt::new_slice(&out, self.is_negative() != rhs.is_negative())
});

impl_op!(rem(self, rhs) => {
    let out = BigInt::with_slices(self, rhs, |this, other| {
        (this % other).into_inner()
    });
    BigInt::new_slice(&out, self.is_negative() != rhs.is_negative())
});

// TODO: Don't convert to usize for these

impl_op!(shl(self, rhs) => {
    let out = BigInt::with_slices(self, rhs, |this, _| {
        (this << usize::try_from(rhs).unwrap()).into_inner()
    });
    BigInt::new_slice(&out, self.is_negative())
});

impl_op!(shr(self, rhs) => {
    let out = BigInt::with_slices(self, rhs, |this, _| {
        (this >> usize::try_from(rhs).unwrap()).into_inner()
    });
    BigInt::new_slice(&out, self.is_negative())
});

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
        BigInt::new_inline(0, false)
    }

    fn is_zero(&self) -> bool {
        self.0.get() == (0, Tag::Inline)
    }
}

impl One for BigInt {
    fn one() -> Self {
        BigInt::new_inline(1, false)
    }
}

#[derive(Debug)]
pub enum FromStrError {
    InvalidRadix(u32),
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
                        Some(idx as u32)
                    } else {
                        None
                    })
                    .ok_or(FromStrError::InvalidChar(c))
            }
            _ => Err(FromStrError::InvalidRadix(radix)),
        }
    }
}

impl Num for BigInt {
    type FromStrRadixErr = FromStrError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let mut out = BigInt::zero();
        for digit in str.chars() {
            let new_val = RadixChars::val_from_char(digit, radix)?;
            out = (out * radix) + new_val;
        }
        Ok(out)
    }
}

impl Signed for BigInt {
    fn abs(&self) -> Self {
        if self.is_negative() {
            -self.clone()
        } else {
            self.clone()
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        (self - other).abs()
    }

    fn signum(&self) -> Self {
        if self.is_zero() {
            BigInt::from(0)
        } else if self.is_negative() {
            BigInt::from(-1)
        } else {
            BigInt::from(1)
        }
    }

    fn is_positive(&self) -> bool {
        !self.0.tag().negative()
    }

    fn is_negative(&self) -> bool {
        self.0.tag().negative()
    }
}

impl ToPrimitive for BigInt {
    fn to_i64(&self) -> Option<i64> {
        self.try_into().ok()
    }

    fn to_u64(&self) -> Option<u64> {
        self.try_into().ok()
    }
}

impl FromPrimitive for BigInt {
    fn from_i64(n: i64) -> Option<Self> {
        Some(BigInt::from(n))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(BigInt::from(n))
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
            while rhs != 0 {
                out *= self.clone();
                rhs = rhs - 1;
            }
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let b0 = BigInt::new_slice(&[0], false);
        assert!(b0.is_inline());
        let b1 = BigInt::new_slice(&[usize::MAX >> 2], false);
        assert!(b1.is_inline());
        let b2 = BigInt::new_slice(&[(usize::MAX >> 2) + 1], false);
        assert!(b2.is_interned());
        let b3 = BigInt::new_slice(&[0, 1], false);
        assert!(b3.is_interned());
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

        assert_eq!(BigInt::from(usize::MAX) + BigInt::from(usize::MAX), BigInt::from((usize::MAX as u128) * 2))
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
            BigInt::new_slice(&[0, 0, 1], false) / BigInt::new_slice(&[2], false),
            BigInt::new_slice(&[0, (usize::MAX / 2) + 1], false),
        );
    }

    #[test]
    fn test_shl() {
        assert_eq!(BigInt::from(1) << BigInt::from(1), BigInt::from(2));
        assert_eq!(BigInt::from(2) << BigInt::from(1), BigInt::from(4));
        assert_eq!(BigInt::from(3) << BigInt::from(1), BigInt::from(6));

        assert_eq!(BigInt::from(usize::MAX) << BigInt::from(1), BigInt::from((usize::MAX as u128) * 2));
    }
}
