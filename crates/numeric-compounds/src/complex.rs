use std::ops::{Add, Div, Mul, Sub};

use numeric_traits::identity::Zero;
use numeric_traits::class::Real;

#[derive(Default)]
pub struct Complex<T> {
    real: T,
    imag: T,
}

impl<T> Complex<T> {
    pub const fn from_vals(real: T, imag: T) -> Complex<T> {
        Complex { real, imag }
    }
}

impl<T: Zero> Complex<T> {
    pub fn new() -> Complex<T> {
        Complex {
            real: T::zero(),
            imag: T::zero(),
        }
    }

    pub fn from_real(real: T) -> Complex<T> {
        Complex { real, imag: T::zero() }
    }

    pub fn from_imag(imag: T) -> Complex<T> {
        Complex { real: T::zero(), imag }
    }
}

impl<T: Clone + Real> Complex<T> {
    pub fn abs_squared(&self) -> T {
        self.real.clone() * self.real.clone() + self.imag.clone() * self.imag.clone()
    }

    pub fn abs(&self) -> T {
        self.abs_squared().sqrt()
    }
}

impl<T> Add for Complex<T>
where
    T: Add,
{
    type Output = Complex<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::from_vals(self.real + rhs.real, self.imag + rhs.imag)
    }
}

impl<T> Add<T> for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>;

    fn add(self, rhs: T) -> Self::Output {
        Complex::from_vals(self.real + rhs, self.imag)
    }
}

impl<T> Sub for Complex<T>
where
    T: Sub,
{
    type Output = Complex<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex::from_vals(self.real - rhs.real, self.imag - rhs.imag)
    }
}

impl<T> Sub<T> for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Complex<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Complex::from_vals(self.real - rhs, self.imag)
    }
}

impl<T, O> Mul for Complex<T>
where
    T: Mul + Clone,
    <T as Mul>::Output: Add<Output = O> + Sub<Output = O>,
{
    type Output = Complex<O>;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex::from_vals(
            self.real.clone() * rhs.real.clone() - self.imag.clone() * rhs.imag.clone(),
            self.real * rhs.imag + rhs.real * self.imag
        )
    }
}

impl<T> Mul<T> for Complex<T>
where
    T: Mul<Output = T> + Clone,
{
    type Output = Complex<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Complex::from_vals(self.real * rhs.clone(), self.imag * rhs)
    }
}

impl<T> Div for Complex<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone,
{
    type Output = Complex<T>;

    fn div(self, rhs: Self) -> Self::Output {
        let divisor = rhs.real.clone() * rhs.real.clone() + rhs.imag.clone() * rhs.imag.clone();
        Complex::from_vals(
            (self.real.clone() * rhs.real.clone() + self.imag.clone() * rhs.imag.clone()) / divisor.clone(),
            (self.imag * rhs.real - self.real * rhs.imag) / divisor,
        )
    }
}

impl<T> Div<T> for Complex<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone,
{
    type Output = Complex<T>;

    fn div(self, rhs: T) -> Self::Output {
        let divisor = rhs.clone() * rhs.clone();
        Complex::from_vals(
            (self.real.clone() * rhs.clone()) / divisor.clone(),
            (self.imag * rhs) / divisor,
        )
    }
}
