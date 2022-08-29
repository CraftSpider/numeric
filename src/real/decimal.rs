//! Implementation of a numeric type that represents real values as whole fractions.

#![allow(unused_variables)]

use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use num_traits::{Bounded, Num, NumCast, One, ToPrimitive, Zero};
use num_traits::real::Real;
use crate::traits::Integral;

/// A real value represented as a whole fraction. With a bounded
/// backing type, this type can represent all whole values from the backing's maximum to minimum,
/// half step values from half maximum to half minimum, etc.
#[derive(Copy, Clone)]
pub struct Decimal<T>(T, T);

impl<T: Integral> Decimal<T> {
    /// Create a new decimal with the default value - zero
    #[must_use]
    pub fn new() -> Decimal<T> {
        Decimal(T::zero(), T::one())
    }

    /// Create a new decimal from a numerator and denominator
    #[must_use]
    pub fn from_num_denom(numerator: T, denominator: T) -> Decimal<T> {
        Decimal(numerator, denominator).reduce()
    }

    #[must_use]
    fn reduce(self) -> Decimal<T> {
        todo!()
    }

    /// Convert this value into a numerator/denominator pair
    pub fn into_num_denom(self) -> (T, T) {
        (self.0, self.1)
    }
}

impl<T: Integral> Default for Decimal<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Integral> PartialEq for Decimal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<T: Integral + Eq> Eq for Decimal<T> {}

impl<T: Integral> PartialOrd for Decimal<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl<T: Integral + Ord> Ord for Decimal<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl<T: Integral> Add for Decimal<T> {
    type Output = Decimal<T>;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Integral> Sub for Decimal<T> {
    type Output = Decimal<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Integral> Mul for Decimal<T> {
    type Output = Decimal<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Integral> Div for Decimal<T> {
    type Output = Decimal<T>;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Integral> Rem for Decimal<T> {
    type Output = Decimal<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Integral + Neg<Output = T>> Neg for Decimal<T> {
    type Output = Decimal<T>;

    fn neg(self) -> Self::Output {
        Decimal(-self.0, self.1)
    }
}

impl<T: Integral> Zero for Decimal<T> {
    fn zero() -> Self {
        Decimal::new()
    }

    fn is_zero(&self) -> bool {
        *self == Decimal::new()
    }
}

impl<T: Integral> One for Decimal<T> {
    fn one() -> Self {
        todo!()
    }
}

impl<T: Integral + Bounded> Bounded for Decimal<T> {
    fn min_value() -> Self {
        Decimal(T::min_value(), T::one())
    }

    fn max_value() -> Self {
        Decimal(T::max_value(), T::one())
    }
}

impl<T: Integral> Num for Decimal<T> {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl<T: Integral> ToPrimitive for Decimal<T> {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }
}

impl<T: Integral> NumCast for Decimal<T> {
    fn from<U: ToPrimitive>(n: U) -> Option<Self> {
        todo!()
    }
}

impl<T: Integral + Bounded + Neg<Output = T> + Copy> Real for Decimal<T> {
    fn min_value() -> Self {
        <Self as Bounded>::min_value()
    }

    fn min_positive_value() -> Self {
        Decimal(T::one(), T::max_value())
    }

    fn epsilon() -> Self {
        Decimal(T::one(), T::max_value())
    }

    fn max_value() -> Self {
        <Self as Bounded>::max_value()
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
