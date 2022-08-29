//! Fixed-precision value

#![allow(unused_variables)]

use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use num_traits::{Num, Bounded, Zero, One, NumCast, ToPrimitive};
use num_traits::real::Real;
use crate::traits::Integral;

/// A fixed-precision value. Given a backing integer T, uses its first `N` bits as decimal
/// precision. If `T` is bounded, this value will also be bounded.
#[derive(Copy, Clone)]
pub struct Fixed<T, const N: usize>(T);

impl<T: Integral, const N: usize> PartialEq for Fixed<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Integral + Eq, const N: usize> Eq for Fixed<T, N> {}

impl<T: Integral + PartialOrd, const N: usize> PartialOrd for Fixed<T, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Integral + Ord, const N: usize> Ord for Fixed<T, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Integral, const N: usize> Add for Fixed<T, N> {
    type Output = Fixed<T, N>;

    fn add(self, rhs: Self) -> Self::Output {
        Fixed(self.0 + rhs.0)
    }
}

impl<T: Integral, const N: usize> Sub for Fixed<T, N> {
    type Output = Fixed<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        Fixed(self.0 - rhs.0)
    }
}

impl<T: Integral, const N: usize> Mul for Fixed<T, N> {
    type Output = Fixed<T, N>;

    fn mul(self, rhs: Self) -> Self::Output {
        Fixed(self.0 * rhs.0)
    }
}

impl<T: Integral, const N: usize> Div for Fixed<T, N> {
    type Output = Fixed<T, N>;

    fn div(self, rhs: Self) -> Self::Output {
        Fixed(self.0 / rhs.0)
    }
}

impl<T: Integral, const N: usize> Rem for Fixed<T, N> {
    type Output = Fixed<T, N>;

    fn rem(self, rhs: Self) -> Self::Output {
        Fixed(self.0 % rhs.0)
    }
}

impl<T: Integral + Neg<Output = T>, const N: usize> Neg for Fixed<T, N> {
    type Output = Fixed<T, N>;

    fn neg(self) -> Self::Output {
        Fixed(-self.0)
    }
}

impl<T: Integral, const N: usize> Zero for Fixed<T, N> {
    fn zero() -> Self {
        Fixed(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: Integral, const N: usize> One for Fixed<T, N> {
    fn one() -> Self {
        Fixed(T::one() << N)
    }
}

impl<T: Integral, const N: usize> Num for Fixed<T, N> {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl<T: Integral, const N: usize> ToPrimitive for Fixed<T, N> {
    fn to_i64(&self) -> Option<i64> {
        // If mask of lower bits fails, None
        // Otherwise result is shifted and converted
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        // If mask of lower bits fails, None
        // Otherwise result is shifted and converted
        todo!()
    }
}

impl<T: Integral, const N: usize> NumCast for Fixed<T, N> {
    fn from<U: ToPrimitive>(n: U) -> Option<Self> {
        todo!()
    }
}

impl<T: Integral + Bounded, const N: usize> Bounded for Fixed<T, N> {
    fn min_value() -> Self {
        Fixed(T::min_value())
    }

    fn max_value() -> Self {
        Fixed(T::max_value())
    }
}

impl<T: Integral + Copy + Bounded + PartialOrd + Neg<Output = T>, const N: usize> Real for Fixed<T, N> {
    fn min_value() -> Self {
        Fixed(T::min_value())
    }

    fn min_positive_value() -> Self {
        Fixed(T::one())
    }

    fn epsilon() -> Self {
        Fixed(T::one())
    }

    fn max_value() -> Self {
        Fixed(T::max_value())
    }

    fn floor(self) -> Self {
        todo!()
    }

    fn ceil(self) -> Self {
        todo!()
    }

    fn round(self) -> Self {
        todo!()
    }

    fn trunc(self) -> Self {
        todo!()
    }

    fn fract(self) -> Self {
        todo!()
    }

    fn abs(self) -> Self {
        todo!()
    }

    fn signum(self) -> Self {
        todo!()
    }

    fn is_sign_positive(self) -> bool {
        todo!()
    }

    fn is_sign_negative(self) -> bool {
        todo!()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        todo!()
    }

    fn recip(self) -> Self {
        todo!()
    }

    fn powi(self, n: i32) -> Self {
        todo!()
    }

    fn powf(self, n: Self) -> Self {
        todo!()
    }

    fn sqrt(self) -> Self {
        todo!()
    }

    fn exp(self) -> Self {
        todo!()
    }

    fn exp2(self) -> Self {
        todo!()
    }

    fn ln(self) -> Self {
        todo!()
    }

    fn log(self, base: Self) -> Self {
        todo!()
    }

    fn log2(self) -> Self {
        todo!()
    }

    fn log10(self) -> Self {
        todo!()
    }

    fn to_degrees(self) -> Self {
        todo!()
    }

    fn to_radians(self) -> Self {
        todo!()
    }

    fn max(self, other: Self) -> Self {
        todo!()
    }

    fn min(self, other: Self) -> Self {
        todo!()
    }

    fn abs_sub(self, other: Self) -> Self {
        todo!()
    }

    fn cbrt(self) -> Self {
        todo!()
    }

    fn hypot(self, other: Self) -> Self {
        todo!()
    }

    fn sin(self) -> Self {
        todo!()
    }

    fn cos(self) -> Self {
        todo!()
    }

    fn tan(self) -> Self {
        todo!()
    }

    fn asin(self) -> Self {
        todo!()
    }

    fn acos(self) -> Self {
        todo!()
    }

    fn atan(self) -> Self {
        todo!()
    }

    fn atan2(self, other: Self) -> Self {
        todo!()
    }

    fn sin_cos(self) -> (Self, Self) {
        todo!()
    }

    fn exp_m1(self) -> Self {
        todo!()
    }

    fn ln_1p(self) -> Self {
        todo!()
    }

    fn sinh(self) -> Self {
        todo!()
    }

    fn cosh(self) -> Self {
        todo!()
    }

    fn tanh(self) -> Self {
        todo!()
    }

    fn asinh(self) -> Self {
        todo!()
    }

    fn acosh(self) -> Self {
        todo!()
    }

    fn atanh(self) -> Self {
        todo!()
    }
}
