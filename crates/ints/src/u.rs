//! N-byte bounded unsigned integer implementation

#![allow(unused_variables)]

use alloc::vec::Vec;
use core::cmp::Ordering;
use core::iter::Product;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};
use core::{array, fmt, iter};
use numeric_bits::algos::{BitwiseDiv, ElementCmp};
use numeric_bits::algos::{ElementAdd, ElementMul, ElementShl, ElementShr, ElementSub};
use numeric_bits::utils::const_reverse;
use numeric_static_iter::{IntoStaticIter, StaticIter};
use numeric_traits::cast::{FromChecked, FromSaturating, FromTruncating, IntoChecked};
use numeric_traits::class::{Bounded, Integral, Numeric, Unsigned};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use numeric_traits::ops::saturating::{SaturatingAdd, SaturatingMul, SaturatingSub};
use numeric_traits::ops::Pow;
use numeric_utils::{static_assert, static_assert_traits};

#[cfg(feature = "rand")]
mod rand_impl;

/// N-byte bounded, unsigned integer. `U<1> == u8`, `U<16> == u128`, etc.
pub struct U<const N: usize>([u8; N]);

static_assert!(size_of::<U<2>>() == 2);
static_assert!(size_of::<U<4>>() == 4);
static_assert!(size_of::<U<8>>() == 8);
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
        U(const_reverse(bytes))
    }

    /// Create a value from raw bytes, laid out in the native endianness
    #[must_use]
    pub const fn from_ne_bytes(bytes: [u8; N]) -> U<N> {
        #[cfg(target_endian = "little")]
        return Self::from_le_bytes(bytes);
        #[cfg(target_endian = "big")]
        return Self::from_be_bytes(bytes);
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
        #[cfg(target_endian = "little")]
        return self.to_le_bytes();
        #[cfg(target_endian = "big")]
        return self.to_be_bytes();
    }

    fn write_base<W: fmt::Write>(&self, base: usize, w: &mut W, chars: &[char]) -> fmt::Result {
        // This is the simplest way - mod base for digit, div base for next digit
        // It isn't super fast though, so there are probably optimization improvements
        let base: U<N> = base.into_checked().unwrap();
        let mut digits = Vec::new();
        let mut scratch = self.clone();

        while scratch > U::zero() {
            let digit = u8::from_checked(scratch.clone() % base)
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

impl<const N: usize> fmt::Debug for U<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<const N: usize> fmt::Display for U<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        self.write_base(10, f, DIGITS)
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

    fn div(mut self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        BitwiseDiv::div_long_checked(&mut self.0, &rhs.0, &mut [0; N]).unwrap();
        #[cfg(not(debug_assertions))]
        BitwiseDiv::div_long_wrapping(&mut self.0, &rhs.0, &mut [0; N]).unwrap();
        self
    }
}

impl<const N: usize> Rem for U<N> {
    type Output = Self;

    fn rem(mut self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        BitwiseDiv::rem_long_checked(&mut self.0, &rhs.0, &mut [0; N]).unwrap();
        #[cfg(not(debug_assertions))]
        BitwiseDiv::rem_long_checked(&mut self.0, &rhs.0, &mut [0; N]).unwrap();
        self
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

    fn shl(mut self, rhs: Self) -> Self::Output {
        let val: usize = usize::from_checked(rhs).unwrap();
        #[cfg(debug_assertions)]
        ElementShl::shl_checked(&mut self.0, val).unwrap();
        #[cfg(not(debug_assertions))]
        ElementShl::shl_wrapping(&mut self.0, val).unwrap();
        self
    }
}

impl<const N: usize> Shr for U<N> {
    type Output = Self;

    fn shr(mut self, rhs: Self) -> Self::Output {
        let val: usize = usize::from_checked(rhs).unwrap();
        #[cfg(debug_assertions)]
        ElementShr::shr_checked(&mut self.0, val).unwrap();
        #[cfg(not(debug_assertions))]
        ElementShr::shr_wrapping(&mut self.0, val).unwrap();
        self
    }
}

impl<const N: usize> Shl<usize> for U<N> {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self::Output {
        #[cfg(debug_assertions)]
        ElementShl::shl_checked(&mut self.0, rhs).unwrap();
        #[cfg(not(debug_assertions))]
        ElementShl::shl_wrapping(&mut self.0, rhs);
        self
    }
}

impl<const N: usize> Shr<usize> for U<N> {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        #[cfg(debug_assertions)]
        ElementShr::shr_checked(&mut self.0, rhs).unwrap();
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
        ElementCmp::cmp(&self.0, &other.0)
    }
}

impl<const N: usize> Pow for U<N> {
    type Output = Self;

    fn pow(self, rhs: Self) -> Self::Output {
        if self.is_zero() || self.is_one() {
            self
        } else if rhs.is_zero() {
            Self::one()
        } else {
            // Any value greater than N will definitely overflow, and N is capped by usize
            let val: usize = usize::from_checked(rhs).unwrap();
            iter::repeat_n(self, val).product()
        }
    }
}

impl<const N: usize> Product<U<N>> for U<N> {
    fn product<I: Iterator<Item = U<N>>>(iter: I) -> Self {
        iter.fold(U::one(), |a, b| a * b)
    }
}

impl<const N: usize> CheckedAdd for U<N> {
    type Output = Self;

    fn checked_add(mut self, rhs: Self) -> Option<Self> {
        ElementAdd::add_checked(&mut self.0, &rhs.0)?;
        Some(self)
    }
}

impl<const N: usize> CheckedSub for U<N> {
    type Output = Self;

    fn checked_sub(mut self, rhs: Self) -> Option<Self> {
        ElementSub::sub_checked(&mut self.0, &rhs.0)?;
        Some(self)
    }
}

impl<const N: usize> CheckedMul for U<N> {
    type Output = Self;

    fn checked_mul(mut self, rhs: Self) -> Option<Self> {
        ElementMul::mul_checked(&mut self.0, &rhs.0)?;
        Some(self)
    }
}

impl<const N: usize> CheckedDiv for U<N> {
    type Output = Self;

    fn checked_div(mut self, rhs: Self) -> Option<Self> {
        BitwiseDiv::div_long_checked(&mut self.0, &rhs.0, &mut [0; N])?;
        Some(self)
    }
}

impl<const N: usize> SaturatingAdd for U<N> {
    type Output = Self;

    fn saturating_add(mut self, rhs: Self) -> Self {
        match ElementAdd::add_checked(&mut self.0, &rhs.0) {
            Some(_) => self,
            None => Self::max_value(),
        }
    }
}

impl<const N: usize> SaturatingSub for U<N> {
    type Output = Self;

    fn saturating_sub(mut self, rhs: Self) -> Self {
        match ElementSub::sub_checked(&mut self.0, &rhs.0) {
            Some(_) => self,
            None => Self::min_value(),
        }
    }
}

impl<const N: usize> SaturatingMul for U<N> {
    type Output = Self;

    fn saturating_mul(mut self, rhs: Self) -> Self {
        match ElementMul::mul_checked(&mut self.0, &rhs.0) {
            Some(_) => self,
            None => Self::max_value(),
        }
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
        self.0[0] == 1 && self.0[1..].iter().all(|&b| b == 0)
    }
}

impl<const N: usize> Numeric for U<N> {}

impl<const N: usize> Integral for U<N> {}

impl<const N: usize> Unsigned for U<N> {}

macro_rules! impl_unsign_cast {
    ($num:ty) => {
        impl<const N: usize> FromChecked<U<N>> for $num {
            fn from_checked(val: U<N>) -> Option<Self> {
                const SIZE: usize = size_of::<$num>();
                let mut arr = [0; SIZE];
                if const { N <= SIZE } {
                    for i in 0..N {
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

        impl<const N: usize> FromSaturating<U<N>> for $num {
            fn saturate_from(val: U<N>) -> Self {
                <$num>::from_checked(val).unwrap_or(<$num>::MAX)
            }
        }

        impl<const N: usize> FromTruncating<U<N>> for $num {
            fn truncate_from(val: U<N>) -> Self {
                const SIZE: usize = size_of::<$num>();
                let mut arr = [0; SIZE];
                for i in 0..N {
                    arr[i] = val.0[i];
                }
                <$num>::from_le_bytes(arr)
            }
        }

        impl<const N: usize> FromChecked<$num> for U<N> {
            fn from_checked(val: $num) -> Option<Self> {
                const SIZE: usize = size_of::<$num>();
                let bytes = val.to_le_bytes();
                let mut arr = [0; N];
                if N >= SIZE {
                    for i in 0..N {
                        arr[i] = bytes[i];
                    }
                    Some(U::from_le_bytes(arr))
                } else {
                    for i in 0..N {
                        if i <= N {
                            arr[i] = bytes[i];
                        } else {
                            if bytes[i] != 0 {
                                return None;
                            }
                        }
                    }
                    Some(U::from_le_bytes(arr))
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
        assert!(one.is_one());
    }

    #[test]
    fn test_cmp() {
        let one: U<3> = U::one();
        let two = U([2, 0, 0]);
        assert_eq!(one, one);
        assert_ne!(one, two);
        assert_eq!(two, two);
    }

    #[test]
    fn test_add() {
        let one: U<3> = U::one();
        assert_eq!(one + one, U([2, 0, 0]));
    }

    #[test]
    fn test_div() {
        let two = U([2, 0, 0]);
        let four = U([4, 0, 0]);
        let ten = U([10, 0, 0]);
        assert_eq!(four / two, U([2, 0, 0]));
        assert_eq!(ten / two, U([5, 0, 0]));
    }
}
