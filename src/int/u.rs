//! N-byte bounded unsigned integer implementation

#![allow(unused_variables)]

use std::cmp::Ordering;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};
use num_traits::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Num, NumCast, One, PrimInt, Saturating, ToPrimitive, Unsigned, Zero};
use crate::bit_slice::BitSlice;

/// N-byte bounded, unsigned integer. `U<1> == u8`, `U<16> == u128`, etc.
pub struct U<const N: usize>([u8; N]);

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

    fn into_slice(self) -> BitSlice<[u8; N], u8> {
        BitSlice::new(self.0)
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
    pub fn as_u8(self) -> u8 {
        u8::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `u8 -> U<1>`
    #[must_use]
    pub fn from_u8(val: u8) -> Self {
        Self(val.to_le_bytes())
    }
}

impl U<2> {
    /// Lossless infallible conversion for `U<2> -> u16`
    #[must_use]
    pub fn as_u16(self) -> u16 {
        u16::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `u16 -> U<2>`
    #[must_use]
    pub fn from_u16(val: u16) -> Self {
        Self(val.to_le_bytes())
    }
}

impl U<4> {
    /// Lossless infallible conversion for `U<4> -> u32`
    #[must_use]
    pub fn as_u32(self) -> u32 {
        u32::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `u32 -> U<4>`
    #[must_use]
    pub fn from_u32(val: u32) -> Self {
        Self(val.to_le_bytes())
    }
}

impl U<8> {
    /// Lossless infallible conversion for `U<8> -> u64`
    #[must_use]
    pub fn as_u64(self) -> u64 {
        u64::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `u64 -> U<8>`
    #[must_use]
    pub fn from_u64(val: u64) -> Self {
        Self(val.to_le_bytes())
    }
}

impl U<16> {
    /// Lossless infallible conversion for `U<16> -> u128`
    #[must_use]
    pub fn as_u128(self) -> u128 {
        u128::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `u128 -> U<16>`
    #[must_use]
    pub fn from_u128(val: u128) -> Self {
        Self(val.to_le_bytes())
    }
}

#[cfg(target_pointer_width = "32")]
impl U<4> {
    /// Lossless infallible conversion for `U<PtrWidth> -> usize`
    #[must_use]
    pub fn as_usize(self) -> usize {
        usize::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `usize -> U<PtrWidth>`
    #[must_use]
    pub fn from_usize(val: usize) -> Self {
        Self(val.to_le_bytes())
    }
}

#[cfg(target_pointer_width = "64")]
impl U<8> {
    /// Lossless infallible conversion for `U<PtrWidth> -> usize`
    #[must_use]
    pub fn as_usize(self) -> usize {
        usize::from_le_bytes(self.0)
    }

    /// Lossless infallible conversion for `usize -> U<PtrWidth>`
    #[must_use]
    pub fn from_usize(val: usize) -> Self {
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

    maybe_default!(
        fn add(self, rhs: Self) -> Self::Output {
            let left = self.into_slice();
            let right = rhs.into_slice();
            #[cfg(debug_assertions)]
            let out = BitSlice::add_element_checked(left, right).unwrap().into_inner();
            #[cfg(not(debug_assertions))]
            let out = BitSlice::add_element_wrapping(left, right).into_inner();
            U(out)
        }
    );
}

#[cfg(feature = "specialize")]
mod specialize_add {
    use super::*;

    impl Add for U<1> {
        fn add(self, rhs: Self) -> Self::Output {
            U::from_u8(self.as_u8() + rhs.as_u8())
        }
    }

    impl Add for U<2> {
        fn add(self, rhs: Self) -> Self::Output {
            U::from_u16(self.as_u16() + rhs.as_u16())
        }
    }

    impl Add for U<4> {
        fn add(self, rhs: Self) -> Self::Output {
            U::from_u32(self.as_u32() + rhs.as_u32())
        }
    }

    impl Add for U<8> {
        fn add(self, rhs: Self) -> Self::Output {
            U::from_u64(self.as_u64() + rhs.as_u64())
        }
    }

    impl Add for U<16> {
        fn add(self, rhs: Self) -> Self::Output {
            U::from_u128(self.as_u128() + rhs.as_u128())
        }
    }
}

impl<const N: usize> Sub for U<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let left = self.into_slice();
        let right = rhs.into_slice();
        #[cfg(debug_assertions)]
        let out = BitSlice::sub_element_checked(left, right).unwrap().into_inner();
        #[cfg(not(debug_assertions))]
        let out = BitSlice::sub_element_wrapping(left, right).into_inner();
        U(out)
    }
}

impl<const N: usize> Mul for U<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let left = self.into_slice();
        let right = rhs.into_slice();
        #[cfg(debug_assertions)]
        let out = BitSlice::mul_long_element_checked(left, right).unwrap().into_inner();
        #[cfg(not(debug_assertions))]
        let out = BitSlice::mul_long_element_wrapping(left, right).into_inner();
        U(out)
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
        U(self.0.zip(rhs.0).map(|(l, r)| l & r))
    }
}

impl<const N: usize> BitOr for U<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        U(self.0.zip(rhs.0).map(|(l, r)| l | r))
    }
}

impl<const N: usize> BitXor for U<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        U(self.0.zip(rhs.0).map(|(l, r)| l ^ r))
    }
}

impl<const N: usize> Shl<usize> for U<N> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        #[cfg(debug_assertions)]
        return U(BitSlice::shl_wrap_and_mask_checked(BitSlice::new(self.0), rhs).unwrap().into_inner());
        #[cfg(not(debug_assertions))]
        return U(BitSlice::shl_wrap_and_mask_wrapping(BitSlice::new(self.0), rhs).into_inner());
    }
}

impl<const N: usize> Shr<usize> for U<N> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        #[cfg(debug_assertions)]
        return U(BitSlice::shr_wrap_and_mask_checked(BitSlice::new(self.0), rhs).unwrap().into_inner());
        #[cfg(not(debug_assertions))]
        return U(BitSlice::shr_wrap_and_mask_wrapping(BitSlice::new(self.0), rhs).into_inner());
    }
}

impl<const N: usize> NumCast for U<N> {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> ToPrimitive for U<N> {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_i128(&self) -> Option<i128> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }

    fn to_u128(&self) -> Option<u128> {
        todo!()
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

impl<const N: usize> CheckedAdd for U<N> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> CheckedSub for U<N> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> CheckedMul for U<N> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> CheckedDiv for U<N> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> Saturating for U<N> {
    fn saturating_add(self, v: Self) -> Self {
        todo!()
    }

    fn saturating_sub(self, v: Self) -> Self {
        todo!()
    }
}

impl<const N: usize> Zero for U<N> {
    fn zero() -> Self {
        U([0; N])
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl<const N: usize> One for U<N> {
    fn one() -> Self {
        todo!()
    }
}

impl<const N: usize> PrimInt for U<N> {
    fn count_ones(self) -> u32 {
        self.into_slice().iter_bits().fold(0, |count, b| if b { count + 1 } else { count })
    }

    fn count_zeros(self) -> u32 {
        self.into_slice().iter_bits().fold(0, |count, b| if b { count } else { count + 1 })
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

impl<const N: usize> Num for U<N> {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl<const N: usize> Unsigned for U<N> {}
