//! N-byte bounded unsigned integer implementation

#![allow(unused_variables)]

use core::array;
use core::cmp::Ordering;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};
use numeric_bits::algos::{ElementAdd, ElementMul, ElementShl, ElementShr, ElementSub};
use numeric_static_iter::{IntoStaticIter, StaticIter};
use numeric_traits::class::{Bounded, Integral, Numeric, Unsigned};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use numeric_traits::ops::saturating::{SaturatingAdd, SaturatingMul, SaturatingSub};
use numeric_traits::ops::Pow;
use numeric_utils::{static_assert, static_assert_traits};

#[cfg(feature = "rand")]
mod rand_impl;

// TODO: Manual Debug that prints the value
/// N-byte bounded, unsigned integer. `U<1> == u8`, `U<16> == u128`, etc.
#[derive(Debug)]
pub struct U<const N: usize>([u8; N]);

static_assert!(core::mem::size_of::<U<2>>() == 2);
static_assert!(core::mem::size_of::<U<4>>() == 4);
static_assert!(core::mem::size_of::<U<8>>() == 8);
static_assert_traits!(U<4>: Send + Sync);

impl<const N: usize> U<N> {
    /// Create a new instance containing the default value (0)
    #[inline]
    #[must_use]
    pub const fn new() -> U<N> {
        U([0; N])
    }

    /// Create a value from raw bytes, laid out in little-endian order
    #[must_use]
    pub const fn from_le_bytes(bytes: [u8; N]) -> U<N> {
        U(bytes)
    }

    /// Create a value from raw bytes, laid out in big-endian order
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; N]) -> U<N> {
        todo!()
    }

    /// Create a value from raw bytes, laid out in the native endianness
    #[must_use]
    pub const fn from_ne_bytes(bytes: [u8; N]) -> U<N> {
        todo!()
    }

    /// Convert this value to raw bytes, laid out in little-endian order
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; N] {
        self.0
    }

    /// Convert this value to raw bytes, laid out in big-endian order
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; N] {
        todo!()
    }

    /// Convert this value to raw bytes, laid out in the native endianness
    #[must_use]
    pub const fn to_ne_bytes(self) -> [u8; N] {
        todo!()
    }
}

impl U<1> {
    /// Lossless infallible conversion for `U<1> -> u8`
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        u8::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `u8 -> U<1>`
    #[must_use]
    pub const fn from_u8(val: u8) -> Self {
        Self(val.to_le_bytes())
    }
}

impl U<2> {
    /// Lossless infallible conversion for `U<2> -> u16`
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        u16::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `u16 -> U<2>`
    #[must_use]
    pub const fn from_u16(val: u16) -> Self {
        Self(val.to_le_bytes())
    }
}

impl U<4> {
    /// Lossless infallible conversion for `U<4> -> u32`
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        u32::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `u32 -> U<4>`
    #[must_use]
    pub const fn from_u32(val: u32) -> Self {
        Self(val.to_le_bytes())
    }
}

impl U<8> {
    /// Lossless infallible conversion for `U<8> -> u64`
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        u64::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `u64 -> U<8>`
    #[must_use]
    pub const fn from_u64(val: u64) -> Self {
        Self(val.to_le_bytes())
    }
}

impl U<16> {
    /// Lossless infallible conversion for `U<16> -> u128`
    #[must_use]
    pub const fn as_u128(self) -> u128 {
        u128::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `u128 -> U<16>`
    #[must_use]
    pub const fn from_u128(val: u128) -> Self {
        Self(val.to_le_bytes())
    }
}

#[cfg(target_pointer_width = "32")]
impl U<4> {
    /// Lossless infallible conversion for `U<PtrWidth> -> usize`
    #[must_use]
    pub const fn as_usize(self) -> usize {
        usize::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `usize -> U<PtrWidth>`
    #[must_use]
    pub const fn from_usize(val: usize) -> Self {
        Self(val.to_le_bytes())
    }
}

#[cfg(target_pointer_width = "64")]
impl U<8> {
    /// Lossless infallible conversion for `U<PtrWidth> -> usize`
    #[must_use]
    pub const fn as_usize(self) -> usize {
        usize::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `usize -> U<PtrWidth>`
    #[must_use]
    pub const fn from_usize(val: usize) -> Self {
        Self(val.to_le_bytes())
    }
}

impl<const N: usize> Copy for U<N> {}

impl<const N: usize> Clone for U<N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize> Default for U<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Add for U<N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        ElementAdd::add_checked(&mut self.0, &rhs.0).unwrap();
        #[cfg(not(debug_assertions))]
        ElementAdd::add_wrapping(&mut self.0, &rhs.0);
        self
    }
}

impl<const N: usize> Sub for U<N> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        ElementSub::sub_checked(&mut self.0, &rhs.0).unwrap();
        #[cfg(not(debug_assertions))]
        ElementSub::sub_wrapping(&mut self.0, &rhs.0);
        self
    }
}

impl<const N: usize> Mul for U<N> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        ElementMul::mul_checked(&mut self.0, &rhs.0).unwrap();
        #[cfg(not(debug_assertions))]
        ElementMul::mul_wrapping(&mut self.0, &rhs.0);
        self
    }
}

impl<const N: usize> Div for U<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Rem for U<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Not for U<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        U(self.0.map(|b| !b))
    }
}

impl<const N: usize> BitAnd for U<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let new = self
            .0
            .into_static_iter()
            .zip(rhs.0.into_static_iter())
            .map(|(l, r)| l & r)
            .collect();
        U(new)
    }
}

impl<const N: usize> BitOr for U<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let new = self
            .0
            .into_static_iter()
            .zip(rhs.0.into_static_iter())
            .map(|(l, r)| l | r)
            .collect();
        U(new)
    }
}

impl<const N: usize> BitXor for U<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let new = self
            .0
            .into_static_iter()
            .zip(rhs.0.into_static_iter())
            .map(|(l, r)| l ^ r)
            .collect();
        U(new)
    }
}

impl<const N: usize> Shl for U<N> {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Shr for U<N> {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Shl<usize> for U<N> {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self::Output {
        #[cfg(debug_assertions)]
        ElementShl::shl_checked(&mut self.0, rhs).unwrap();
        #[cfg(not(debug_assertions))]
        ElementShl::shl_wrap_and_mask_wrapping(&mut self.0, rhs);
        self
    }
}

impl<const N: usize> Shr<usize> for U<N> {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        #[cfg(debug_assertions)]
        ElementShr::shr_wrap_and_mask_checked(&mut self.0, rhs).unwrap();
        #[cfg(not(debug_assertions))]
        ElementShr::shr_wrap_and_mask_wrapping(&mut self.0, rhs);
        self
    }
}

impl<const N: usize> Bounded for U<N> {
    fn min_value() -> Self {
        U([0; N])
    }

    fn max_value() -> Self {
        U([u8::MAX; N])
    }
}

impl<const N: usize> PartialEq for U<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<const N: usize> Eq for U<N> {}

impl<const N: usize> PartialOrd for U<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for U<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl<const N: usize> Pow for U<N> {
    type Output = Self;

    fn pow(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> CheckedAdd for U<N> {
    type Output = Self;

    fn checked_add(self, rhs: Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> CheckedSub for U<N> {
    type Output = Self;

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> CheckedMul for U<N> {
    type Output = Self;

    fn checked_mul(self, rhs: Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> CheckedDiv for U<N> {
    type Output = Self;

    fn checked_div(self, rhs: Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> SaturatingAdd for U<N> {
    type Output = Self;

    fn saturating_add(self, rhs: Self) -> Self {
        todo!()
    }
}

impl<const N: usize> SaturatingSub for U<N> {
    type Output = Self;

    fn saturating_sub(self, rhs: Self) -> Self {
        todo!()
    }
}

impl<const N: usize> SaturatingMul for U<N> {
    type Output = Self;

    fn saturating_mul(self, rhs: Self) -> Self {
        todo!()
    }
}

impl<const N: usize> Zero for U<N> {
    fn zero() -> Self {
        U([0; N])
    }

    fn is_zero(&self) -> bool {
        self.0.into_static_iter().all(|b| b == 0)
    }
}

impl<const N: usize> One for U<N> {
    fn one() -> Self {
        U(array::from_fn(|idx| if idx == 0 { 1 } else { 0 }))
    }

    fn is_one(&self) -> bool {
        todo!()
    }
}

impl<const N: usize> Numeric for U<N> {}

impl<const N: usize> Integral for U<N> {}

impl<const N: usize> Unsigned for U<N> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let one: U<1> = U::one();
        assert_eq!(one, U([1]));
        let one = U::one();
        assert_eq!(one, U([1, 0]));
        let one: U<4> = U::one();
        assert_eq!(one, U([1, 0, 0, 0]));
    }
}
