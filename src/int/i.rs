//! N-byte bounded signed integer implementation

#![allow(unused_variables)]

use std::cmp::Ordering;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};
use num_traits::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Num, NumCast, One, PrimInt, Saturating, Signed, ToPrimitive, Zero};

/// N-byte bounded, signed integer. `I<1> == i8`, `I<16> == i128`, etc.
///
/// Represented in two's complement, with the highest bit forming the sign bit
pub struct I<const N: usize>([u8; N]);

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

impl<const N: usize> NumCast for I<N> {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> ToPrimitive for I<N> {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
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

impl<const N: usize> PartialEq for I<N> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<const N: usize> Eq for I<N> {}

impl<const N: usize> PartialOrd for I<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl<const N: usize> Ord for I<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl<const N: usize> CheckedAdd for I<N> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> CheckedSub for I<N> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> CheckedMul for I<N> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> CheckedDiv for I<N> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> Saturating for I<N> {
    fn saturating_add(self, v: Self) -> Self {
        todo!()
    }

    fn saturating_sub(self, v: Self) -> Self {
        todo!()
    }
}

impl<const N: usize> Zero for I<N> {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl<const N: usize> One for I<N> {
    fn one() -> Self {
        todo!()
    }
}

impl<const N: usize> PrimInt for I<N> {
    fn count_ones(self) -> u32 {
        todo!()
    }

    fn count_zeros(self) -> u32 {
        todo!()
    }

    fn leading_zeros(self) -> u32 {
        todo!()
    }

    fn trailing_zeros(self) -> u32 {
        todo!()
    }

    fn rotate_left(self, n: u32) -> Self {
        todo!()
    }

    fn rotate_right(self, n: u32) -> Self {
        todo!()
    }

    fn signed_shl(self, n: u32) -> Self {
        todo!()
    }

    fn signed_shr(self, n: u32) -> Self {
        todo!()
    }

    fn unsigned_shl(self, n: u32) -> Self {
        todo!()
    }

    fn unsigned_shr(self, n: u32) -> Self {
        todo!()
    }

    fn swap_bytes(self) -> Self {
        todo!()
    }

    fn from_be(x: Self) -> Self {
        todo!()
    }

    fn from_le(x: Self) -> Self {
        todo!()
    }

    fn to_be(self) -> Self {
        todo!()
    }

    fn to_le(self) -> Self {
        todo!()
    }

    fn pow(self, exp: u32) -> Self {
        todo!()
    }
}

impl<const N: usize> Num for I<N> {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl<const N: usize> Signed for I<N> {
    fn abs(&self) -> Self {
        todo!()
    }

    fn abs_sub(&self, other: &Self) -> Self {
        todo!()
    }

    fn signum(&self) -> Self {
        todo!()
    }

    fn is_positive(&self) -> bool {
        todo!()
    }

    fn is_negative(&self) -> bool {
        todo!()
    }
}
