//! Complex numbers, values of the form `Ai + B`.

use core::ops::{Add, Div, Mul, Sub};

use numeric_traits::class::Real;
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::core::NumOps;

/// Complex number pair, containing both a real and imaginary component.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Complex<T> {
    real: T,
    imag: T,
}

impl<T> Complex<T> {
    /// Create a complex number from its real and imaginary components.
    pub const fn new(real: T, imag: T) -> Complex<T> {
        Complex { real, imag }
    }

    /// Get the real component of this complex number.
    #[inline]
    pub fn real(&self) -> &T {
        &self.real
    }

    /// Get the imaginary component of this complex number.
    #[inline]
    pub fn imag(&self) -> &T {
        &self.imag
    }
}

impl<T: Zero> Complex<T> {
    /// Create a complex number with the provided real value, and a zero imaginary component.
    pub fn from_real(real: T) -> Complex<T> {
        Complex {
            real,
            imag: T::zero(),
        }
    }

    /// Create a complex number with a zero real component, and the provided imaginary value.
    pub fn from_imag(imag: T) -> Complex<T> {
        Complex {
            real: T::zero(),
            imag,
        }
    }
}

impl<T: Real> Complex<T> {
    /// Get the squared absolute value of this complex number.
    ///
    /// This is equivalent to the sum of the squares of the real and imaginary components.
    pub fn abs_squared(&self) -> T {
        self.real.clone() * self.real.clone() + self.imag.clone() * self.imag.clone()
    }

    /// Get the absolute value of this complex number
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
        Complex::new(self.real + rhs.real, self.imag + rhs.imag)
    }
}

impl<T> Add<T> for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>;

    fn add(self, rhs: T) -> Self::Output {
        Complex::new(self.real + rhs, self.imag)
    }
}

impl<T> Sub for Complex<T>
where
    T: Sub,
{
    type Output = Complex<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex::new(self.real - rhs.real, self.imag - rhs.imag)
    }
}

impl<T> Sub<T> for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Complex<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Complex::new(self.real - rhs, self.imag)
    }
}

impl<T, O> Mul for Complex<T>
where
    T: Mul + Clone,
    <T as Mul>::Output: Add<Output = O> + Sub<Output = O>,
{
    type Output = Complex<O>;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex::new(
            self.real.clone() * rhs.real.clone() - self.imag.clone() * rhs.imag.clone(),
            self.real * rhs.imag + rhs.real * self.imag,
        )
    }
}

impl<T> Mul<T> for Complex<T>
where
    T: Mul<Output = T> + Clone,
{
    type Output = Complex<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Complex::new(self.real * rhs.clone(), self.imag * rhs)
    }
}

impl<T> Div for Complex<T>
where
    T: NumOps + Clone,
{
    type Output = Complex<T>;

    fn div(self, rhs: Self) -> Self::Output {
        let divisor = rhs.real.clone() * rhs.real.clone() + rhs.imag.clone() * rhs.imag.clone();
        Complex::new(
            (self.real.clone() * rhs.real.clone() + self.imag.clone() * rhs.imag.clone())
                / divisor.clone(),
            (self.imag * rhs.real - self.real * rhs.imag) / divisor,
        )
    }
}

impl<T> Div<T> for Complex<T>
where
    T: NumOps + Clone,
{
    type Output = Complex<T>;

    fn div(self, rhs: T) -> Self::Output {
        let divisor = rhs.clone() * rhs.clone();
        Complex::new(
            (self.real.clone() * rhs.clone()) / divisor.clone(),
            (self.imag * rhs) / divisor,
        )
    }
}

// TODO: Rem and Pow

impl<T: PartialEq + Zero> Zero for Complex<T> {
    fn zero() -> Self {
        Complex {
            real: T::zero(),
            imag: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl<T: PartialEq + Zero + One> One for Complex<T> {
    fn one() -> Self {
        Complex::from_real(T::one())
    }

    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}
