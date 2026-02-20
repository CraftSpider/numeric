//! Checked arithmetic operations. These are similar to the regular arithmetic operations, except
//! that they return `None` if the operation would overflow or otherwise fail.

/// Checked addition. Compare to [`core::ops::Add`].
pub trait CheckedAdd<Rhs = Self> {
    /// The output of addition
    type Output;

    /// Performs the addition operation, returning `None` on overflow or underflow.
    fn checked_add(self, rhs: Rhs) -> Option<Self::Output>;
}

/// Checked subtraction. Compare to [`core::ops::Sub`].
pub trait CheckedSub<Rhs = Self> {
    /// The output of subtraction
    type Output;

    /// Performs the subtraction operation, returning `None` on overflow or underflow.
    fn checked_sub(self, rhs: Rhs) -> Option<Self::Output>;
}

/// Checked multiplication. Compare to [`core::ops::Mul`].
pub trait CheckedMul<Rhs = Self> {
    /// The output of multiplication
    type Output;

    /// Performs the multiplication operation, returning `None` on overflow or underflow.
    fn checked_mul(self, rhs: Rhs) -> Option<Self::Output>;
}

/// Checked division. Compare to [`core::ops::Div`].
pub trait CheckedDiv<Rhs = Self> {
    /// The output of division
    type Output;

    /// Performs the division operation, returning `None` on underflow or division by zero.
    fn checked_div(self, rhs: Rhs) -> Option<Self::Output>;
}

/// Checked left-shift. Compare to [`core::ops::Shl`].
pub trait CheckedShl<Rhs = Self> {
    /// The output of the left shift
    type Output;

    /// Performs the shift operation, returning `None` if the right-hand is larger than the
    /// type's bit width.
    fn checked_shl(self, rhs: Rhs) -> Option<Self::Output>;
}

/// Checked right-shift. Compare to [`core::ops::Shr`].
pub trait CheckedShr<Rhs = Self> {
    /// The output of the right shift
    type Output;

    /// Performs the shift operation, returning `None` if the right-hand is larger than the
    /// type's bit width.
    fn checked_shr(self, rhs: Rhs) -> Option<Self::Output>;
}

/// Trait combining the common checked arithmetic operations. Blanket implemented for all valid
/// types.
pub trait CheckedOps<Rhs = Self, Out = Self>:
    CheckedAdd<Rhs, Output = Out>
    + CheckedSub<Rhs, Output = Out>
    + CheckedMul<Rhs, Output = Out>
    + CheckedDiv<Rhs, Output = Out>
{
}

impl<T, Rhs, Out> CheckedOps<Rhs, Out> for T where
    T: CheckedAdd<Rhs, Output = Out>
        + CheckedSub<Rhs, Output = Out>
        + CheckedMul<Rhs, Output = Out>
        + CheckedDiv<Rhs, Output = Out>
{
}
