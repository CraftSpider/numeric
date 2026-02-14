//! N-byte bounded signed integer implementation

#![allow(unused_variables)]

use arrayvec::ArrayVec;
use core::cmp::Ordering;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};
use core::{array, fmt};
use numeric_bits::algos::{
    AssignAddAlgo, AssignDivRemAlgo, AssignMulAlgo, AssignShlAlgo, AssignShrAlgo, AssignSubAlgo,
    Bitwise, Element,
};
use numeric_bits::utils::const_reverse;
use numeric_static_iter::{IntoStaticIter, StaticIter};
use numeric_traits::cast::{FromChecked, FromSaturating, FromTruncating, IntoChecked};
use numeric_traits::class::{Bounded, BoundedSigned, Integral, Numeric, Signed};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use numeric_traits::ops::saturating::{SaturatingAdd, SaturatingMul, SaturatingSub};
use numeric_traits::ops::Pow;
use numeric_utils::{static_assert, static_assert_traits};

#[cfg(feature = "rand")]
mod rand_impl;

/// N-byte bounded, signed integer. `I<1> == i8`, `I<16> == i128`, etc.
///
/// Represented in two's complement, with the highest bit forming the sign bit
#[derive(Copy, Clone)]
pub struct I<const N: usize>([u8; N]);

static_assert!(size_of::<I<2>>() == 2);
static_assert!(size_of::<I<4>>() == 4);
static_assert!(size_of::<I<8>>() == 8);
static_assert_traits!(I<4>: Send + Sync);

impl<const N: usize> I<N> {
    /// Create a new instance containing the default value (0)
    #[inline]
    #[must_use]
    pub const fn new() -> I<N> {
        I([0; N])
    }

    /// Create a value from raw bytes, laid out in little-endian order
    #[must_use]
    pub const fn from_le_bytes(bytes: [u8; N]) -> I<N> {
        I(bytes)
    }

    /// Create a value from raw bytes, laid out in big-endian order
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; N]) -> I<N> {
        I(const_reverse(bytes))
    }

    /// Create a value from raw bytes, laid out in the native endianness
    #[must_use]
    pub const fn from_ne_bytes(bytes: [u8; N]) -> I<N> {
        if cfg!(target_endian = "little") {
            Self::from_le_bytes(bytes)
        } else {
            Self::from_be_bytes(bytes)
        }
    }

    /// Convert this value to raw bytes, laid out in little-endian order
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; N] {
        self.0
    }

    /// Convert this value to raw bytes, laid out in big-endian order
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; N] {
        const_reverse(self.0)
    }

    /// Convert this value to raw bytes, laid out in the native endianness
    #[must_use]
    pub const fn to_ne_bytes(self) -> [u8; N] {
        if cfg!(target_endian = "little") {
            self.to_le_bytes()
        } else {
            self.to_be_bytes()
        }
    }

    fn write_base<W: fmt::Write>(&self, base: usize, w: &mut W, chars: &[char]) -> fmt::Result {
        // This is the simplest way - mod base for digit, div base for next digit
        // It isn't super fast though, so there are probably optimization improvements
        let base: I<N> = base.into_checked().unwrap();
        // TODO: This may be insufficient for very large numbers
        let mut digits = ArrayVec::<u8, 255>::new();
        let mut scratch = *self;

        if scratch.is_negative() {
            w.write_char('-')?;
        }

        while scratch != I::zero() {
            let digit = u8::from_checked((scratch % base).abs())
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
}

impl<const N: usize> fmt::Debug for I<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<const N: usize> fmt::Display for I<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        self.write_base(10, f, DIGITS)
    }
}

impl<const N: usize> Add for I<N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        <Element as AssignAddAlgo>::wrapping(&mut self.0, &rhs.0);
        self
    }
}

impl<const N: usize> Sub for I<N> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        <Element as AssignSubAlgo>::wrapping(&mut self.0, &rhs.0);
        self
    }
}

impl<const N: usize> Mul for I<N> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        <Element as AssignMulAlgo>::wrapping(&mut self.0, &rhs.0);
        self
    }
}

impl<const N: usize> Div for I<N> {
    type Output = Self;

    fn div(mut self, mut rhs: Self) -> Self::Output {
        // Division of minimum value by negative one is invalid, since it would produce an
        // out-of-range positive value.
        assert!(
            self != I::min_value() || rhs != -I::one(),
            "attempt to divide with overflow"
        );
        assert!(!rhs.is_zero(), "attempt to divide by zero");
        let neg = self.is_negative() != rhs.is_negative();
        self = self.abs();
        rhs = rhs.abs();
        <Bitwise as AssignDivRemAlgo>::div_wrapping(&mut self.0, &rhs.0, &mut [0; N]);
        if neg {
            self = -self;
        }
        self
    }
}

impl<const N: usize> Rem for I<N> {
    type Output = Self;

    fn rem(mut self, mut rhs: Self) -> Self::Output {
        let neg = self.is_negative() != rhs.is_negative();
        self = self.abs();
        rhs = rhs.abs();
        <Bitwise as AssignDivRemAlgo>::rem_wrapping(&mut self.0, &rhs.0, &mut [0; N]);
        if neg {
            self = -self;
        }
        self
    }
}

impl<const N: usize> Neg for I<N> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.0 = self.0.map(|v| !v);
        self = self + Self::one();
        self
    }
}

impl<const N: usize> Not for I<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        I(self.0.map(|b| !b))
    }
}

impl<const N: usize> BitAnd for I<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let new = self
            .0
            .into_static_iter()
            .zip(rhs.0.into_static_iter())
            .map(|(l, r)| l & r)
            .collect();
        I(new)
    }
}

impl<const N: usize> BitOr for I<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let new = self
            .0
            .into_static_iter()
            .zip(rhs.0.into_static_iter())
            .map(|(l, r)| l | r)
            .collect();
        I(new)
    }
}

impl<const N: usize> BitXor for I<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let new = self
            .0
            .into_static_iter()
            .zip(rhs.0.into_static_iter())
            .map(|(l, r)| l ^ r)
            .collect();
        I(new)
    }
}

impl<const N: usize> Shl for I<N> {
    type Output = Self;

    fn shl(mut self, rhs: Self) -> Self::Output {
        let val: usize = usize::from_checked(rhs).unwrap();
        #[cfg(debug_assertions)]
        <Element as AssignShlAlgo>::checked(&mut self.0, val).unwrap();
        #[cfg(not(debug_assertions))]
        <Element as AssignShlAlgo>::wrapping(&mut self.0, val);
        self
    }
}

impl<const N: usize> Shr for I<N> {
    type Output = Self;

    fn shr(mut self, rhs: Self) -> Self::Output {
        let val: usize = usize::from_checked(rhs).unwrap();
        #[cfg(debug_assertions)]
        <Element as AssignShrAlgo>::checked(&mut self.0, val).unwrap();
        #[cfg(not(debug_assertions))]
        <Element as AssignShrAlgo>::wrapping(&mut self.0, val);
        self
    }
}

impl<const N: usize> Shl<usize> for I<N> {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self::Output {
        #[cfg(debug_assertions)]
        <Element as AssignShlAlgo>::checked(&mut self.0, rhs).unwrap();
        #[cfg(not(debug_assertions))]
        <Element as AssignShlAlgo>::wrapping(&mut self.0, rhs);
        self
    }
}

impl<const N: usize> Shr<usize> for I<N> {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        #[cfg(debug_assertions)]
        <Element as AssignShrAlgo>::checked(&mut self.0, rhs).unwrap();
        #[cfg(not(debug_assertions))]
        <Element as AssignShrAlgo>::wrapping(&mut self.0, rhs);
        self
    }
}

impl<const N: usize> Bounded for I<N> {
    fn min_value() -> Self {
        I(array::from_fn(|idx| if idx == N - 1 { 0x80 } else { 0 }))
    }

    fn max_value() -> Self {
        I(array::from_fn(|idx| if idx == N - 1 { 0x7F } else { 0xFF }))
    }
}

impl<const N: usize> BoundedSigned for I<N> {
    fn min_positive() -> Self {
        I::one()
    }

    fn max_negative() -> Self {
        // TODO: Maybe -I::one()
        I([0xFF; N])
    }
}

impl<const N: usize> PartialEq for I<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<const N: usize> Eq for I<N> {}

impl<const N: usize> PartialOrd for I<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for I<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.is_negative().cmp(&other.is_negative()) {
            Ordering::Equal => self.0.cmp(&other.0),
            ord => ord.reverse(),
        }
    }
}

impl<const N: usize> CheckedAdd for I<N> {
    type Output = Self;

    fn checked_add(self, rhs: Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> CheckedSub for I<N> {
    type Output = Self;

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> CheckedMul for I<N> {
    type Output = Self;

    fn checked_mul(self, rhs: Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> CheckedDiv for I<N> {
    type Output = Self;

    fn checked_div(self, rhs: Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> SaturatingAdd for I<N> {
    type Output = Self;

    fn saturating_add(self, v: Self) -> Self {
        todo!()
    }
}

impl<const N: usize> SaturatingSub for I<N> {
    type Output = Self;

    fn saturating_sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> SaturatingMul for I<N> {
    type Output = Self;

    fn saturating_mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Zero for I<N> {
    fn zero() -> Self {
        I([0; N])
    }

    fn is_zero(&self) -> bool {
        self.0.into_static_iter().all(|b| b == 0)
    }
}

impl<const N: usize> One for I<N> {
    fn one() -> Self {
        I(array::from_fn(|idx| u8::from(idx == 0)))
    }

    fn is_one(&self) -> bool {
        self.0[0] == 1 && self.0[1..].iter().all(|&b| b == 0)
    }
}

impl<const N: usize> Pow for I<N> {
    type Output = I<N>;

    fn pow(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Numeric for I<N> {}

impl<const N: usize> Signed for I<N> {
    fn abs(mut self) -> Self {
        if self.is_negative() {
            self = -self;
        }
        self
    }

    fn is_positive(&self) -> bool {
        let last = *self.0.last().unwrap();
        last & 0x80 == 0
    }

    fn is_negative(&self) -> bool {
        let last = *self.0.last().unwrap();
        last & 0x80 != 0
    }
}

impl<const N: usize> Integral for I<N> {}

macro_rules! impl_unsign_cast {
    ($num:ty) => {
        impl<const N: usize> FromChecked<I<N>> for $num {
            fn from_checked(val: I<N>) -> Option<Self> {
                if val.is_negative() {
                    return None;
                }
                const SIZE: usize = size_of::<$num>();
                let mut arr = [0; SIZE];
                if const { N <= SIZE } {
                    for i in 0..N {
                        arr[i] = val.0[i];
                    }
                    Some(<$num>::from_le_bytes(arr))
                } else {
                    for i in 0..N {
                        if i < SIZE {
                            arr[i] = val.0[i];
                        } else {
                            if val.0[i] != 0 {
                                return None;
                            }
                        }
                    }
                    Some(<$num>::from_le_bytes(arr))
                }
            }
        }

        impl<const N: usize> FromSaturating<I<N>> for $num {
            fn saturate_from(val: I<N>) -> Self {
                match <$num>::from_checked(val) {
                    Some(val) => val,
                    None => {
                        if val.is_negative() {
                            <$num>::MIN
                        } else {
                            <$num>::MAX
                        }
                    }
                }
            }
        }

        impl<const N: usize> FromTruncating<I<N>> for $num {
            fn truncate_from(val: I<N>) -> Self {
                const SIZE: usize = size_of::<$num>();
                let mut arr = [0; SIZE];
                for i in 0..N {
                    arr[i] = val.0[i];
                }
                <$num>::from_le_bytes(arr)
            }
        }

        impl<const N: usize> FromChecked<$num> for I<N> {
            fn from_checked(val: $num) -> Option<Self> {
                const SIZE: usize = size_of::<$num>();
                let bytes = val.to_le_bytes();
                let mut arr = [0; N];
                if N > SIZE {
                    for i in 0..SIZE {
                        arr[i] = bytes[i];
                    }
                    Some(I::from_le_bytes(arr))
                } else {
                    for i in 0..SIZE {
                        if i < N - 1 || (i == N - 1 && bytes[i] < 0x80) {
                            arr[i] = bytes[i];
                        } else {
                            if bytes[i] != 0 {
                                return None;
                            }
                        }
                    }
                    Some(I::from_le_bytes(arr))
                }
            }
        }
    };
}

macro_rules! impl_sign_cast {
    ($num:ty) => {
        impl<const N: usize> FromChecked<I<N>> for $num {
            fn from_checked(val: I<N>) -> Option<Self> {
                const SIZE: usize = size_of::<$num>();
                let mut arr = [if val.is_negative() { 0xFF } else { 0 }; SIZE];
                if const { N <= SIZE } {
                    for i in 0..SIZE {
                        arr[i] = val.0[i];
                    }
                    Some(<$num>::from_le_bytes(arr))
                } else {
                    for i in 0..N {
                        if i <= SIZE {
                            arr[i] = val.0[i];
                        } else {
                            if val.0[i] != 0 {
                                return None;
                            }
                        }
                    }
                    Some(<$num>::from_le_bytes(arr))
                }
            }
        }

        impl<const N: usize> FromSaturating<I<N>> for $num {
            fn saturate_from(val: I<N>) -> Self {
                match <$num>::from_checked(val) {
                    Some(val) => val,
                    None => {
                        if val.is_negative() {
                            <$num>::MIN
                        } else {
                            <$num>::MAX
                        }
                    }
                }
            }
        }

        impl<const N: usize> FromTruncating<I<N>> for $num {
            fn truncate_from(val: I<N>) -> Self {
                const SIZE: usize = size_of::<$num>();
                let mut arr = [0; SIZE];
                for i in 0..N {
                    arr[i] = val.0[i];
                }
                <$num>::from_le_bytes(arr)
            }
        }

        impl<const N: usize> FromChecked<$num> for I<N> {
            fn from_checked(val: $num) -> Option<Self> {
                const SIZE: usize = size_of::<$num>();
                let bytes = val.to_le_bytes();
                let mut arr = [if val.is_negative() { 0xFF } else { 0 }; N];
                if N >= SIZE {
                    for i in 0..SIZE {
                        arr[i] = bytes[i];
                    }
                    Some(I::from_le_bytes(arr))
                } else {
                    for i in 0..SIZE {
                        if i < N
                            && (bytes[i] >= 0x80 && val.is_negative()
                                || bytes[i] < 0x80 && val.is_positive())
                        {
                            arr[i] = bytes[i];
                        } else {
                            if (val.is_positive() && bytes[i] != 0)
                                || (val.is_negative() && bytes[i] != 0xFF)
                            {
                                return None;
                            }
                        }
                    }
                    Some(I::from_le_bytes(arr))
                }
            }
        }
    };
}

impl_unsign_cast!(u8);
impl_unsign_cast!(u16);
impl_unsign_cast!(u32);
impl_unsign_cast!(u64);
impl_unsign_cast!(u128);
impl_unsign_cast!(usize);

impl_sign_cast!(i8);
impl_sign_cast!(i16);
impl_sign_cast!(i32);
impl_sign_cast!(i64);
impl_sign_cast!(i128);
impl_sign_cast!(isize);

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_display() {
        let one: I<1> = I::one();
        assert_eq!(one.to_string(), "1");
        let neg_one: I<1> = -I::one();
        assert_eq!(neg_one.to_string(), "-1");
        let max: I<1> = I::max_value();
        assert_eq!(max.to_string(), "127");
        let min: I<1> = I::min_value();
        assert_eq!(min.to_string(), "-128");

        let big: I<5> = I::from_checked(i32::MAX).unwrap();
        assert_eq!(big.to_string(), "2147483647");
        let small: I<5> = I::from_checked(i32::MIN).unwrap();
        assert_eq!(small.to_string(), "-2147483648");
    }

    #[test]
    fn test_one() {
        let one: I<1> = I::one();
        assert_eq!(one, I([1]));
        let one: I<2> = I::one();
        assert_eq!(one, I([1, 0]));
        let one: I<4> = I::one();
        assert_eq!(one, I([1, 0, 0, 0]));
        assert!(one.is_one());
    }

    #[test]
    fn test_cmp() {
        let one: I<3> = I::one();
        let two = I([2, 0, 0]);
        assert_eq!(one, one);
        assert_ne!(one, two);
        assert_eq!(two, two);
    }

    #[test]
    fn test_ord() {
        let zero: I<3> = I::zero();
        let one = I::one();
        let neg_one = -I::one();
        let two = one + one;
        let neg_two = -two;

        assert!(neg_one > neg_two);
        assert!(neg_one < zero);
        assert!(neg_one < one);
        assert!(neg_one < two);

        assert!(zero > neg_two);
        assert!(zero > neg_one);
        assert!(zero < one);
        assert!(zero < two);

        assert!(one > neg_two);
        assert!(one > neg_one);
        assert!(one > zero);
        assert!(one < two);
    }

    #[test]
    fn test_abs() {
        let one: I<3> = I::one();
        let neg_one: I<3> = -I::one();
        let big: I<3> = I::max_value();
        let small = I::min_value() + one;

        assert_eq!(neg_one.abs(), one);
        assert_eq!(small.abs(), big);
    }

    #[test]
    fn test_neg() {
        let zero: I<3> = I::zero();
        let one: I<3> = I::one();
        let neg_one = I::max_negative();

        assert_eq!(-one, neg_one);
        assert_eq!(-neg_one, one);
        assert_eq!(-zero, zero);
    }

    #[test]
    fn test_add() {
        let one: I<3> = I::one();
        let zero = I::zero();
        let neg_one = -I::one();
        assert_eq!(one + one, I([2, 0, 0]));
        assert_eq!(one + zero, one);
        assert_eq!(zero + zero, zero);
        assert_eq!(neg_one + one, zero);
        assert_eq!(neg_one + zero, neg_one);
        assert_eq!(neg_one + neg_one, I([0xFE, 0xFF, 0xFF]));
    }

    #[test]
    fn test_sub() {
        let one: I<3> = I::one();
        let zero = I::zero();
        assert_eq!(one + one, I([2, 0, 0]));
        assert_eq!(one + zero, one);
        assert_eq!(zero + zero, zero);
    }

    #[test]
    fn test_mul() {
        let one: I<3> = I::one();
        let zero = I::zero();
        let neg_one = -I::one();
        let two = one + one;

        assert_eq!(zero * zero, zero);
        assert_eq!(one * one, one);
        assert_eq!(one * two, I([2, 0, 0]));
        assert_eq!(one * zero, zero);
        assert_eq!(neg_one * one, neg_one);
        assert_eq!(neg_one * zero, zero);
        assert_eq!(neg_one * two, I([0xFE, 0xFF, 0xFF]));
    }

    #[test]
    fn test_div() {
        let one: I<3> = I::one();
        let zero = I::zero();
        let neg_one = -I::one();
        let two = one + one;
        let max = I::max_value();
        let min = I::min_value();

        assert_eq!(zero / one, zero);
        assert_eq!(one / one, one);
        assert_eq!(one / two, zero);
        assert_eq!(two / one, two);
        assert_eq!(two / two, one);
        assert_eq!(neg_one / one, -one);
        assert_eq!(one / neg_one, -one);
        assert_eq!(neg_one / neg_one, one);
        assert_eq!(max / one, max);
        assert_eq!(max / max, one);
        assert_eq!(max / neg_one, min + one);
    }

    #[test]
    #[should_panic]
    fn test_div_invalid() {
        let _ = I::<1>::min_value() / -I::one();
    }

    #[test]
    fn test_from_checked() {
        assert_eq!(I::<1>::from_checked(0u8), Some(I::zero()));
        assert_eq!(I::<1>::from_checked(127u8), Some(I::max_value()));
        assert_eq!(I::<1>::from_checked(128u8), None);
        assert_eq!(I::<1>::from_checked(255u8), None);
        assert_eq!(I::<1>::from_checked(-128i8), Some(I::min_value()));
        assert_eq!(I::<1>::from_checked(127i8), Some(I::max_value()));

        assert_eq!(I::<1>::from_checked(0u16), Some(I::zero()));
        assert_eq!(I::<1>::from_checked(127u16), Some(I::max_value()));
        assert_eq!(I::<1>::from_checked(128u16), None);
        assert_eq!(I::<1>::from_checked(255u16), None);
        assert_eq!(I::<1>::from_checked(u16::MAX), None);
        assert_eq!(I::<1>::from_checked(-128i16), Some(I::min_value()));
        assert_eq!(I::<1>::from_checked(-129i16), None);
        assert_eq!(I::<1>::from_checked(127i16), Some(I::max_value()));
        assert_eq!(I::<1>::from_checked(128i16), None);

        assert_eq!(I::<2>::from_checked(0u8), Some(I::zero()));
        assert_eq!(I::<2>::from_checked(255u8), Some(I([0xFF, 0x0])));
        assert_eq!(I::<2>::from_checked(i8::MIN), Some(I([0x80, 0xFF])));
        assert_eq!(I::<2>::from_checked(i8::MAX), Some(I([0x7F, 0x00])));
    }
}
