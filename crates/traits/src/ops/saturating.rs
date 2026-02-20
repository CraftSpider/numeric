//! Saturating arithmetic operations. These are similar to the regular arithmetic operations, but
//! return the maximum or minimum value of the type if the operation would overflow or underflow.

/// Saturating addition. Compare to [`core::ops::Add`].
pub trait SaturatingAdd<Rhs = Self> {
    /// The output of addition
    type Output;

    /// Performs the addition operation, returning the maximum or minimum value of the type if the
    /// operation would overflow or underflow.
    fn saturating_add(self, rhs: Rhs) -> Self::Output;
}

/// Saturating subtraction. Compare to [`core::ops::Sub`].
pub trait SaturatingSub<Rhs = Self> {
    /// The output of subtraction
    type Output;

    /// Performs the subtraction operation, returning the maximum or minimum value of the type if
    /// the operation would overflow or underflow.
    fn saturating_sub(self, rhs: Rhs) -> Self::Output;
}

/// Saturating multiplication. Compare to [`core::ops::Mul`].
pub trait SaturatingMul<Rhs = Self> {
    /// The output of multiplication
    type Output;

    /// Performs the multiplication operation, returning the maximum or minimum value of the type
    /// if the operation would overflow or underflow.
    fn saturating_mul(self, rhs: Rhs) -> Self::Output;
}

/// Trait combining the common saturating arithmetic operations. Blanket implemented for all valid
/// types.
pub trait SaturatingOps<Rhs = Self, Out = Self>:
    SaturatingAdd<Rhs, Output = Out>
    + SaturatingSub<Rhs, Output = Out>
    + SaturatingMul<Rhs, Output = Out>
{
}

impl<Rhs, Out, T> SaturatingOps<Rhs, Out> for T where
    T: SaturatingAdd<Rhs, Output = Out>
        + SaturatingSub<Rhs, Output = Out>
        + SaturatingMul<Rhs, Output = Out>
{
}
