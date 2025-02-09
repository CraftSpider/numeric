//! N-byte bounded signed integer implementation

#![allow(unused_variables)]

use core::array;
use core::cmp::Ordering;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};
use numeric_bits::algos::{BitwiseDiv, ElementAdd, ElementMul, ElementSub};
use numeric_static_iter::{IntoStaticIter, StaticIter};
use numeric_traits::class::{Bounded, BoundedSigned, Integral, Numeric, Signed};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use numeric_traits::ops::saturating::{SaturatingAdd, SaturatingMul, SaturatingSub};
use numeric_traits::ops::Pow;
use numeric_utils::{static_assert, static_assert_traits};

#[cfg(feature = "rand")]
mod rand_impl;

// TODO: Manual debug that prints the value
/// N-byte bounded, signed integer. `I<1> == i8`, `I<16> == i128`, etc.
///
/// Represented in two's complement, with the highest bit forming the sign bit
#[derive(Debug)]
pub struct I<const N: usize>([u8; N]);

static_assert!(size_of::<I<2>>() == 2);
static_assert!(size_of::<I<4>>() == 4);
static_assert!(size_of::<I<8>>() == 8);
static_assert_traits!(I<4>: Send + Sync);

impl<const N: usize> Copy for I<N> {}

impl<const N: usize> Clone for I<N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize> Add for I<N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        ElementAdd::add_wrapping(&mut self.0, &rhs.0);
        self
    }
}

impl<const N: usize> Sub for I<N> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        ElementSub::sub_wrapping(&mut self.0, &rhs.0);
        self
    }
}

impl<const N: usize> Mul for I<N> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        ElementMul::mul_wrapping(&mut self.0, &rhs.0);
        self
    }
}

impl<const N: usize> Div for I<N> {
    type Output = Self;

    fn div(mut self, mut rhs: Self) -> Self::Output {
        let neg = self.is_negative() != rhs.is_negative();
        self = self.abs();
        rhs = rhs.abs();
        BitwiseDiv::div_long_wrapping(&mut self.0, &rhs.0, &mut [0; N]);
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
        BitwiseDiv::rem_long_wrapping(&mut self.0, &rhs.0, &mut [0; N]);
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

    fn shl(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Shr for I<N> {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Shl<usize> for I<N> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Shr<usize> for I<N> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        todo!()
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
        I(array::from_fn(|idx| if idx == 0 { 1 } else { 0 }))
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
        let last = self.0.last_mut().unwrap();
        *last &= 0x7F;
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
