//! Overflowing arithmetic operations. These are similar to the regular arithmetic operations, but
//! return a tuple of the (wrapped) result and a boolean indicating whether an overflow occurred.

/// Overflowing addition. Compare to [`core::ops::Add`].
pub trait OverflowingAdd<Rhs = Self> {
    /// The output of addition
    type Output;

    /// Performs the addition operation, returning the wrapped result and whether an overflow
    /// occurred.
    fn overflowing_add(self, rhs: Rhs) -> (Self::Output, bool);
}

/// Overflowing subtraction. Compare to [`core::ops::Sub`].
pub trait OverflowingSub<Rhs = Self> {
    /// The output of subtraction
    type Output;

    /// Performs the subtraction operation, returning the wrapped result and whether an overflow
    /// occurred.
    fn overflowing_sub(self, rhs: Rhs) -> (Self::Output, bool);
}

/// Overflowing multiplication. Compare to [`core::ops::Mul`].
pub trait OverflowingMul<Rhs = Self> {
    /// The output of multiplication
    type Output;

    /// Performs the multiplication operation, returning the wrapped result and whether an overflow
    /// occurred.
    fn overflowing_mul(self, rhs: Rhs) -> (Self::Output, bool);
}

/// Trait combining the common overflowing arithmetic operations. Blanket implemented for all valid
/// types.
pub trait OverflowingOps<Rhs = Self, Out = Self>:
    OverflowingAdd<Rhs, Output = Out>
    + OverflowingSub<Rhs, Output = Out>
    + OverflowingMul<Rhs, Output = Out>
{
}

impl<Rhs, Out, T> OverflowingOps<Rhs, Out> for T where
    T: OverflowingAdd<Rhs, Output = Out>
        + OverflowingSub<Rhs, Output = Out>
        + OverflowingMul<Rhs, Output = Out>
{
}
