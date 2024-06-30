
/// Generic trait for types implementing wrapping numeric operations.
/// This is automatically implemented for types which implement the wrapping math traits
pub trait WrappingOps<Rhs = Self, Out = Self>:
    WrappingAdd<Rhs, Output = Out>
    + WrappingSub<Rhs, Output = Out>
    + WrappingMul<Rhs, Output = Out>
{
}

impl<Rhs, Out, T> WrappingOps<Rhs, Out> for T where
    T: WrappingAdd<Rhs, Output = Out>
        + WrappingSub<Rhs, Output = Out>
        + WrappingMul<Rhs, Output = Out>
{
}

/// Generic trait for types implementing wrapping shift operations.
/// This is automatically implemented for types which implement the wrapping shift traits
pub trait WrappingShiftOps<Rhs = Self, Out = Self>:
    WrappingShl<Rhs, Output = Out> + WrappingShr<Rhs, Output = Out>
{
}

impl<Rhs, Out, T> WrappingShiftOps<Rhs, Out> for T where
    T: WrappingShl<Rhs, Output = Out> + WrappingShr<Rhs, Output = Out>
{
}

pub trait WrappingAdd<Rhs = Self> {
    type Output;

    fn wrapping_add(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingSub<Rhs = Self> {
    type Output;

    fn wrapping_sub(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingMul<Rhs = Self> {
    type Output;

    fn wrapping_mul(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingShl<Rhs = Self> {
    type Output;

    fn wrapping_shl(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingShr<Rhs = Self> {
    type Output;

    fn wrapping_shr(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingNeg {
    type Output;

    fn wrapping_neg(self) -> Self::Output;
}
