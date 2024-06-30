//! N-byte bounded signed integer implementation

#![allow(unused_variables)]

use core::cmp::Ordering;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};
use numeric_utils::{static_assert, static_assert_traits};
use numeric_traits::class::{Bounded, Signed, BoundedSigned, Numeric, Integral};
use numeric_traits::ops::checked::{CheckedAdd, CheckedSub, CheckedMul, CheckedDiv};
use numeric_traits::ops::saturating::{SaturatingAdd, SaturatingSub, SaturatingMul};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::Pow;
use numeric_static_iter::{IntoStaticIter, StaticIter};

#[cfg(feature = "rand")]
mod rand_impl;

// TODO: Manual debug that prints the value
/// N-byte bounded, signed integer. `I<1> == i8`, `I<16> == i128`, etc.
///
/// Represented in two's complement, with the highest bit forming the sign bit
#[derive(Debug)]
pub struct I<const N: usize>([u8; N]);

static_assert!(core::mem::size_of::<I<2>>() == 2);
static_assert!(core::mem::size_of::<I<4>>() == 4);
static_assert!(core::mem::size_of::<I<8>>() == 8);
static_assert_traits!(I<4>: Send + Sync);

impl<const N: usize> Copy for I<N> {}

impl<const N: usize> Clone for I<N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize> Add for I<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Sub for I<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Mul for I<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Div for I<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Rem for I<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Neg for I<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Not for I<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> BitAnd for I<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> BitOr for I<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> BitXor for I<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        todo!()
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
        todo!()
    }

    fn max_value() -> Self {
        todo!()
    }
}

impl<const N: usize> BoundedSigned for I<N> {
    fn min_positive() -> Self {
        todo!()
    }

    fn max_negative() -> Self {
        todo!()
    }
}

impl<const N: usize> PartialEq for I<N> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
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
        todo!()
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
        todo!()
    }

    fn is_one(&self) -> bool {
        todo!()
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
    fn abs(self) -> Self {
        todo!()
    }

    fn is_positive(&self) -> bool {
        todo!()
    }

    fn is_negative(&self) -> bool {
        todo!()
    }
}

impl<const N: usize> Integral for I<N> {

}
