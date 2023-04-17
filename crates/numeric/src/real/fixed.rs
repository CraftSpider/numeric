//! Fixed-precision value

#![allow(unused_variables)]

use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use num_traits::{NumCast, ToPrimitive};
use numeric_traits::identity::{Zero, One};
use numeric_traits::class::{Numeric, Bounded, Integral, Real};
use numeric_traits::ops::core::ShiftOps;
use numeric_traits::ops::Pow;

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

impl<T: Integral, const N: usize> Pow for Fixed<T, N> {
    type Output = Fixed<T, N>;

    fn pow(self, rhs: Self) -> Self::Output {
        todo!()
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

impl<T: Integral + ShiftOps<usize>, const N: usize> One for Fixed<T, N> {
    fn one() -> Self {
        Fixed(T::one() << N)
    }

    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}

impl<T: Integral + ShiftOps<usize>, const N: usize> Numeric for Fixed<T, N> {}

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

impl<T: Integral + ShiftOps<usize>, const N: usize> Real for Fixed<T, N> {
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

    fn log(self, base: Self) -> Self {
        todo!()
    }
}
