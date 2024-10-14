//! Implementation of a numeric type that represents real values as whole fractions.

#![allow(unused_variables)]

use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use numeric_traits::class::{Bounded, BoundedSigned, Integral, Numeric, Real, Signed};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::{Gcd, Pow};

/// A real value represented as a whole fraction. With a bounded
/// backing type, this type can represent all whole values from the backing's maximum to minimum,
/// half step values from half maximum to half minimum, etc.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Rat<T> {
    num: T,
    denom: T,
}

impl<T: Integral> Rat<T> {
    /// Create a new rational from a numerator and denominator, performing no validation or
    /// simplification of both values.
    ///
    /// # Safety
    ///
    /// The implementation and users of `Rat` may assume that it is always in reduced form,
    /// and that the denominator is non-zero. Creating a `Rat` that breaks these rules may
    /// lead to UB.
    #[must_use]
    pub const unsafe fn new_unchecked(numerator: T, denominator: T) -> Rat<T> {
        Rat {
            num: numerator,
            denom: denominator,
        }
    }

    /// Create a new rational from a numerator and denominator, with both values simplified.
    #[must_use]
    pub fn new(numerator: T, denominator: T) -> Option<Rat<T>>
    where
        T: Gcd<Output = T>,
    {
        if denominator == T::zero() {
            None
        } else {
            Some(Self::reduce(numerator, denominator))
        }
    }

    fn reduce(num: T, denom: T) -> Rat<T>
    where
        T: Gcd<Output = T>,
    {
        if num == T::zero() {
            Rat::zero()
        } else if (num == T::one() && denom == T::one())
            || num.clone() % denom.clone() == T::zero()
            || denom.clone() % num.clone() == T::zero()
        {
            unsafe { Rat::new_unchecked(num, denom) }
        } else {
            let gcd = num.clone().gcd(denom.clone());
            unsafe { Rat::new_unchecked(num / gcd.clone(), denom / gcd) }
        }
    }

    pub fn numerator(&self) -> &T {
        &self.num
    }

    pub fn denominator(&self) -> &T {
        &self.denom
    }

    /// Convert this value into a numerator/denominator pair
    pub fn into_pair(self) -> (T, T) {
        (self.num, self.denom)
    }
}

impl<T: Integral + fmt::Debug> fmt::Debug for Rat<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Print as decimal
        write!(f, "{:?} / {:?}", self.num, self.denom)
    }
}

impl<T: Integral> Default for Rat<T> {
    fn default() -> Self {
        Rat::zero()
    }
}

impl<T: Integral> PartialOrd for Rat<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // For a one denominator, this is simple.
        // For other values, we want to figure out how to handle the range of the denominator
        //
        // Hint from C++: there's some trick here involving the sequence of numerator, denominator,
        // the division of them, and the modulo of them. Something about euclid GCD
        todo!()
    }
}

impl<T: Integral + Ord> Ord for Rat<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl<T: Integral + Gcd<Output = T>> Add for Rat<T> {
    type Output = Rat<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Rat::new(
            self.num * rhs.denom.clone() + rhs.num * self.denom.clone(),
            self.denom * rhs.denom,
        )
        .unwrap()
    }
}

impl<T: Integral + Gcd<Output = T>> Sub for Rat<T> {
    type Output = Rat<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Rat::new(
            self.num * rhs.denom.clone() - rhs.num * self.denom.clone(),
            self.denom * rhs.denom,
        )
        .unwrap()
    }
}

impl<T: Integral + Gcd<Output = T>> Mul for Rat<T> {
    type Output = Rat<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Rat::new(self.num * rhs.num, self.denom * rhs.denom).unwrap()
    }
}

impl<T: Integral + Gcd<Output = T>> Div for Rat<T> {
    type Output = Rat<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Rat::new(self.num * rhs.denom, self.denom * rhs.num).unwrap()
    }
}

impl<T: Integral> Rem for Rat<T> {
    type Output = Rat<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Integral + Neg<Output = T>> Neg for Rat<T> {
    type Output = Rat<T>;

    fn neg(self) -> Self::Output {
        unsafe { Rat::new_unchecked(-self.num, self.denom) }
    }
}

impl<T: Integral> Zero for Rat<T> {
    fn zero() -> Self {
        unsafe { Rat::new_unchecked(T::zero(), T::one()) }
    }

    fn is_zero(&self) -> bool {
        self.numerator().is_zero()
    }
}

impl<T: Integral> One for Rat<T> {
    fn one() -> Self {
        unsafe { Rat::new_unchecked(T::one(), T::one()) }
    }

    fn is_one(&self) -> bool {
        *self == Rat::one()
    }
}

impl<T: Integral + Signed> Signed for Rat<T> {
    fn abs(self) -> Self {
        unsafe { Rat::new_unchecked(self.num.abs(), self.denom) }
    }

    fn is_positive(&self) -> bool {
        self.num.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.num.is_negative()
    }
}

impl<T: Integral + Bounded> Bounded for Rat<T> {
    fn min_value() -> Self {
        unsafe { Rat::new_unchecked(T::min_value(), T::one()) }
    }

    fn max_value() -> Self {
        unsafe { Rat::new_unchecked(T::max_value(), T::one()) }
    }
}

impl<T: Integral + BoundedSigned> BoundedSigned for Rat<T> {
    fn min_positive() -> Self {
        unsafe { Rat::new_unchecked(T::min_positive(), T::max_value()) }
    }

    fn max_negative() -> Self {
        unsafe { Rat::new_unchecked(T::max_negative(), T::max_value()) }
    }
}

impl<T: Integral> Pow for Rat<T> {
    type Output = Rat<T>;

    fn pow(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Integral + Gcd<Output = T>> Numeric for Rat<T> {}

impl<T: Integral + Gcd<Output = T>> Real for Rat<T> {
    fn floor(self) -> Self {
        let diff = self.num.clone() % self.denom.clone();
        Rat::new((self.num - diff) / self.denom, T::one()).unwrap()
    }

    fn ceil(self) -> Self {
        let diff = self.denom.clone() - (self.num.clone() % self.denom.clone());
        Rat::new((self.num + diff) / self.denom, T::one()).unwrap()
    }

    fn round(self) -> Self {
        let two = T::one() + T::one();
        let diff = self.num.clone() % self.denom.clone();
        // Check if remainder is more or less than half denom
        let add = if diff > (self.denom.clone() / two) {
            T::one()
        } else {
            T::zero()
        };
        // Integer division should be round-to-zero division
        // TODO: This may be wrong for negatives? We probably want floor-division
        let div = self.num / self.denom;
        // Floor div, add one if we should round up
        Rat::new(div + add, T::one()).unwrap()
    }

    fn trunc(self) -> Self {
        // Rounds towards -inf, so we can't just  div - that will round towards zero
        todo!()
    }

    fn fract(self) -> Self {
        todo!()
    }

    fn log(self, base: Self) -> Self {
        todo!()
    }

    fn sqrt(self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Rat::new(1, 2).unwrap();
        let b = Rat::new(3, 4).unwrap();
        assert_eq!(a + b, Rat::new(5, 4).unwrap());
    }

    #[test]
    fn test_round() {
        let a = Rat::new(3, 7).unwrap();
        let b = Rat::new(4, 7).unwrap();
        let c = Rat::new(10, 7).unwrap();
        let d = Rat::new(11, 7).unwrap();
        assert_eq!(a.round(), Rat::zero());
        assert_eq!(b.round(), Rat::one());
        assert_eq!(c.round(), Rat::one());
        assert_eq!(d.round(), Rat::one() + Rat::one());
    }
}
