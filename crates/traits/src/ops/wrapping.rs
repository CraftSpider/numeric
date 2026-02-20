//! Wrapping arithmetic operations. These are similar to the regular arithmetic operations, except
//! that they wrap around on overflow or underflow.

/// Wrapping addition. Compare to [`core::ops::Add`].
pub trait WrappingAdd<Rhs = Self> {
    /// The output of addition
    type Output;

    /// Performs the addition operation, wrapping around on overflow.
    fn wrapping_add(self, rhs: Rhs) -> Self::Output;
}

/// Wrapping subtraction. Compare to [`core::ops::Sub`].
pub trait WrappingSub<Rhs = Self> {
    /// The output of subtraction
    type Output;

    /// Performs the subtraction operation, wrapping around on overflow.
    fn wrapping_sub(self, rhs: Rhs) -> Self::Output;
}

/// Wrapping multiplication. Compare to [`core::ops::Mul`].
pub trait WrappingMul<Rhs = Self> {
    /// The output of multiplication
    type Output;

    /// Performs the multiplication operation, wrapping around on overflow.
    fn wrapping_mul(self, rhs: Rhs) -> Self::Output;
}

/// Wrapping left-shift. Compare to [`core::ops::Shl`].
pub trait WrappingShl<Rhs = Self> {
    /// The output of the left shift
    type Output;

    /// Performs the shift operation, wrapping the right hand side if it is larger than the type's
    /// bit width.
    fn wrapping_shl(self, rhs: Rhs) -> Self::Output;
}

/// Wrapping right-shift. Compare to [`core::ops::Shr`].
pub trait WrappingShr<Rhs = Self> {
    /// The output of the right shift
    type Output;

    /// Performs the shift operation, wrapping the right hand side if it is larger than the type's
    /// bit width.
    fn wrapping_shr(self, rhs: Rhs) -> Self::Output;
}

/// Wrapping negation. Compare to [`core::ops::Neg`].
pub trait WrappingNeg {
    /// The output of negation
    type Output;

    /// Performs the negation operation, wrapping around on overflow.
    fn wrapping_neg(self) -> Self::Output;
}

/// Generic trait for types implementing wrapping numeric operations.
/// This is automatically implemented for types which implement the wrapping math traits
pub trait WrappingOps<Rhs = Self, Out = Self>:
    WrappingAdd<Rhs, Output = Out> + WrappingSub<Rhs, Output = Out> + WrappingMul<Rhs, Output = Out>
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
