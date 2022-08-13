use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use num_traits::{Bounded, Num, NumCast, One, ToPrimitive, Zero};
use num_traits::real::Real;
use crate::decimal::sealed::DecSafe;
use crate::traits::Integral;

mod sealed {
    use super::*;

    /// Trait for types which are safe to use in `Decimal`
    pub trait DecSafe: Num + Integral {}

    impl<T: Num + Integral> DecSafe for T {}
}

/// A real value represented as a whole fraction -
#[derive(Copy, Clone)]
pub struct Decimal<T>(T, T);

impl<T: DecSafe> Decimal<T> {
    pub fn new() -> Decimal<T> {
        Decimal(T::zero(), T::one())
    }

    pub fn from_num_denom(numerator: T, denominator: T) -> Decimal<T> {
        Decimal(numerator, denominator).reduce()
    }

    fn reduce(self) -> Decimal<T> {
        todo!()
    }
}

impl<T: DecSafe> Default for Decimal<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: DecSafe> PartialEq for Decimal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<T: DecSafe + Eq> Eq for Decimal<T> {}

impl<T: DecSafe> PartialOrd for Decimal<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl<T: DecSafe + Ord> Ord for Decimal<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl<T: DecSafe> Add for Decimal<T> {
    type Output = Decimal<T>;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: DecSafe> Sub for Decimal<T> {
    type Output = Decimal<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: DecSafe> Mul for Decimal<T> {
    type Output = Decimal<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: DecSafe> Div for Decimal<T> {
    type Output = Decimal<T>;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: DecSafe> Rem for Decimal<T> {
    type Output = Decimal<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: DecSafe + Neg<Output = T>> Neg for Decimal<T> {
    type Output = Decimal<T>;

    fn neg(self) -> Self::Output {
        Decimal(-self.0, self.1)
    }
}

impl<T: DecSafe> Zero for Decimal<T> {
    fn zero() -> Self {
        Decimal::new()
    }

    fn is_zero(&self) -> bool {
        *self == Decimal::new()
    }
}

impl<T: DecSafe> One for Decimal<T> {
    fn one() -> Self {
        todo!()
    }
}

impl<T: DecSafe + Bounded> Bounded for Decimal<T> {
    fn min_value() -> Self {
        Decimal(T::min_value(), T::one())
    }

    fn max_value() -> Self {
        Decimal(T::max_value(), T::one())
    }
}

impl<T: DecSafe> Num for Decimal<T> {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl<T: DecSafe> ToPrimitive for Decimal<T> {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }
}

impl<T: DecSafe> NumCast for Decimal<T> {
    fn from<U: ToPrimitive>(n: U) -> Option<Self> {
        todo!()
    }
}

impl<T: DecSafe + Bounded + Neg<Output = T> + Copy> Real for Decimal<T> {
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
