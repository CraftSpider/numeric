pub mod checked;
pub mod core;
pub mod overflowing;
pub mod saturating;
pub mod wrapping;
pub mod widening;

pub trait Pow<Rhs = Self> {
    type Output;

    fn pow(self, rhs: Rhs) -> Self::Output;
}

pub trait TrigOps {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;

    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
}

pub trait HypTrigOps {
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;

    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;
}
