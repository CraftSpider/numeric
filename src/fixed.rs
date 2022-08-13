use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use num_traits::{Num, Bounded, Zero, One, NumCast, ToPrimitive};
use num_traits::real::Real;
use crate::traits::Integral;

#[derive(Copy, Clone)]
pub struct Fixed<T>(T);

impl<T> PartialEq for Fixed<T> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<T: Eq> Eq for Fixed<T> {}

impl<T> PartialOrd for Fixed<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl<T: Ord> Ord for Fixed<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl<T> Add for Fixed<T> {
    type Output = Fixed<T>;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T> Sub for Fixed<T> {
    type Output = Fixed<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T> Mul for Fixed<T> {
    type Output = Fixed<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T> Div for Fixed<T> {
    type Output = Fixed<T>;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T> Rem for Fixed<T> {
    type Output = Fixed<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Neg<Output = T>> Neg for Fixed<T> {
    type Output = Fixed<T>;

    fn neg(self) -> Self::Output {
        Fixed(-self.0)
    }
}

impl<T> Zero for Fixed<T> {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl<T> One for Fixed<T> {
    fn one() -> Self {
        todo!()
    }
}

impl<T> Num for Fixed<T> {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl<T> ToPrimitive for Fixed<T> {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }
}

impl<T> NumCast for Fixed<T> {
    fn from<U: ToPrimitive>(n: U) -> Option<Self> {
        todo!()
    }
}

impl<T> Bounded for Fixed<T> {
    fn min_value() -> Self {
        todo!()
    }

    fn max_value() -> Self {
        todo!()
    }
}

impl<T: Copy + Neg<Output = T>> Real for Fixed<T> {
    fn min_value() -> Self {
        todo!()
    }

    fn min_positive_value() -> Self {
        todo!()
    }

    fn epsilon() -> Self {
        todo!()
    }

    fn max_value() -> Self {
        todo!()
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
