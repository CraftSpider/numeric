//! Fixed-precision value

#![allow(unused_variables)]

use std::cmp::Ordering;
use std::fmt::{self, Write};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use numeric_traits::cast::FromTruncating;
use numeric_traits::identity::{Zero, One};
use numeric_traits::class::{Numeric, Bounded, Integral, Real, BoundedSigned, Signed};
use numeric_traits::ops::Pow;

fn mask<T: Integral, const N: usize>() -> T {
    (T::one() << N) - T::one()
}

/// A fixed-precision value. Given a backing integer T, uses its first `N` bits as decimal
/// precision. If `T` is bounded, this value will also be bounded.
#[derive(Copy, Clone)]
pub struct Fixed<T, const N: usize>(T);

impl<T: Integral, const N: usize> Fixed<T, N> {
    pub fn new() -> Self {
        Fixed(T::zero())
    }

    pub fn from_val(val: T) -> Self {
        Fixed(val << N)
    }

    pub fn from_raw(val: T) -> Self {
        Fixed(val)
    }

    fn is_whole(&self) -> bool {
        self.0.clone() & !mask::<T, N>() == self.0
    }
}

impl<T: Integral, const N: usize> Default for Fixed<T, N> {
    fn default() -> Self {
        Fixed::new()
    }
}

impl<T, const N: usize> fmt::Debug for Fixed<T, N>
where
    T: Integral + fmt::Debug + FromTruncating<usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let two = T::truncate(2);
        let ten = T::truncate(10);
        // TODO: This can overflow. Find a non-overflowing method
        //       Maybe shift, print upper, then do this to the fractional part
        let mut whole = self.0.clone()
            * ten.clone().pow(T::truncate(N))
            / two.pow(T::truncate(N));
        let mut idx = 0;

        let mut buf = String::new();
        while whole > T::zero() {
            let digit = whole.clone() % ten.clone();
            if idx == N {
                write!(buf, ".")?;
            }
            write!(buf, "{:?}", digit)?;
            whole = whole / ten.clone();
            idx += 1;
        }
        if idx == N {
            write!(buf, ".")?;
        }
        buf = buf.chars().rev().collect::<String>();
        f.write_str(&buf)?;

        Ok(())
    }
}

impl<T, const N: usize> fmt::Binary for Fixed<T, N>
where
    T: Integral + fmt::Binary,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}.{:b}", self.clone().trunc().0, self.clone().fract().0)
    }
}

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

impl<T: Integral, const N: usize> One for Fixed<T, N> {
    fn one() -> Self {
        Fixed(T::one() << N)
    }

    fn is_one(&self) -> bool {
        *self == Self::one()
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

impl<T: Integral + Bounded + Signed, const N: usize> BoundedSigned for Fixed<T, N> {
    fn max_negative() -> Self {
        Fixed(-T::one())
    }

    fn min_positive() -> Self {
        Fixed(T::one())
    }
}

impl<T: Integral, const N: usize> Numeric for Fixed<T, N> {}

impl<T: Integral + Signed, const N: usize> Signed for Fixed<T, N> {
    fn abs(self) -> Self {
        Fixed(self.0.abs())
    }

    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

impl<T: Integral, const N: usize> Real for Fixed<T, N> {
    /// Get the largest integer less than or equal to this number.
    ///
    /// This respects signedness, and as such, negative will floor towards negative infinity.
    fn floor(self) -> Self {
        Fixed((self.0 >> N) << N)
    }

    fn ceil(self) -> Self {
        if self.is_whole() {
            self.floor()
        } else {
            self.floor() + Self::one()
        }
    }

    fn round(self) -> Self {
        let half = Self::one() / (Self::one() + Self::one());
        let f = self.clone().fract();
        let f = if f >= Self::zero() { f } else { Self::one() - f };
        if f > half {
            self.ceil()
        } else {
            self.floor()
        }
    }

    fn trunc(self) -> Self {
        self.clone() - self.fract()
    }

    fn fract(self) -> Self {
        self % Self::one()
    }

    fn log(self, base: Self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        assert_eq!(&format!("{:?}", Fixed::<_, 1>::from_raw(0b01)), ".5");
        assert_eq!(&format!("{:?}", Fixed::<_, 1>::from_raw(0b10)), "1.0");
        assert_eq!(&format!("{:?}", Fixed::<_, 1>::from_raw(0b11)), "1.5");
    }

    #[test]
    fn fixed_floor() {
        assert_eq!(Fixed::<_, 1>::from_val(2).floor(), Fixed::from_val(2));
        assert_eq!(Fixed::<_, 1>::from_raw(0b11).floor(), Fixed::from_val(1));
        assert_eq!(Fixed::<_, 1>::from_raw(-0b11).floor(), Fixed::from_val(-2));
        assert_eq!(Fixed::<_, 2>::from_raw(-0b110).floor(), Fixed::from_val(-2));
    }

    #[test]
    fn fixed_ceil() {
        assert_eq!(Fixed::<_, 1>::from_val(2).ceil(), Fixed::from_val(2));
        assert_eq!(Fixed::<_, 1>::from_raw(0b11).ceil(), Fixed::from_val(2));
        assert_eq!(Fixed::<_, 1>::from_raw(-0b11).ceil(), Fixed::from_val(-1));
        assert_eq!(Fixed::<_, 2>::from_raw(-0b110).ceil(), Fixed::from_val(-1));
    }

    #[test]
    fn fixed_trunc() {
        assert_eq!(Fixed::<_, 1>::from_val(2).trunc(), Fixed::from_val(2));
        assert_eq!(Fixed::<_, 1>::from_raw(0b11).trunc(), Fixed::from_val(1));
        assert_eq!(Fixed::<_, 1>::from_raw(-0b11).trunc(), Fixed::from_val(-1));
        assert_eq!(Fixed::<_, 2>::from_raw(-0b110).trunc(), Fixed::from_val(-1));
    }

    #[test]
    fn fixed_fract() {
        assert_eq!(Fixed::<_, 1>::from_val(2).fract(), Fixed::from_val(0));
        assert_eq!(Fixed::<_, 1>::from_raw(0b11).fract(), Fixed::from_raw(0b01));
        assert_eq!(Fixed::<_, 1>::from_raw(-0b11).fract(), Fixed::from_raw(-0b01));
        assert_eq!(Fixed::<_, 2>::from_raw(-0b110).fract(), Fixed::from_raw(-0b010));
    }
}
