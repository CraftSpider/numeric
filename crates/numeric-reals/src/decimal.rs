//! Implementation of a numeric type that represents real values as whole fractions.

#![allow(unused_variables)]

use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use numeric_traits::identity::{Zero, One};
use numeric_traits::class::{Integral, Numeric, Real, Bounded, BoundedSigned, Signed};
use numeric_traits::ops::Pow;

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
        Decimal(T::one(), T::one())
    }

    fn is_one(&self) -> bool {
        *self == Decimal::one()
    }
}

impl<T: Integral + Signed> Signed for Decimal<T> {
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

impl<T: Integral + Bounded> Bounded for Decimal<T> {
    fn min_value() -> Self {
        Decimal(T::min_value(), T::one())
    }

    fn max_value() -> Self {
        Decimal(T::max_value(), T::one())
    }
}

impl<T: Integral + BoundedSigned> BoundedSigned for Decimal<T> {
    fn min_positive() -> Self {
        Decimal(T::min_positive(), T::max_value())
    }

    fn max_negative() -> Self {
        Decimal(-T::min_positive(), T::max_value())
    }
}

impl<T: Integral> Pow for Decimal<T> {
    type Output = Decimal<T>;

    fn pow(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Integral> Numeric for Decimal<T> {}

impl<T: Integral> Real for Decimal<T> {
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

    fn sqrt(self) -> Self {
        todo!()
    }

    fn log(self, base: Self) -> Self {
        todo!()
    }
}
