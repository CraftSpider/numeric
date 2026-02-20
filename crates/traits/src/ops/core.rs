//! Common arithmetic operations. These are the most basic operations that can be performed on
//! any numeric type.

use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

/// Trait combining the common arithmetic operations. Blanket implemented for all valid types.
///
/// The [`Neg`](core::ops::Neg) trait is skipped due to not being valid for unsigned types.
pub trait NumOps<Rhs = Self, Out = Self>:
    Add<Rhs, Output = Out>
    + Sub<Rhs, Output = Out>
    + Mul<Rhs, Output = Out>
    + Div<Rhs, Output = Out>
    + Rem<Rhs, Output = Out>
{
}

impl<Rhs, Out, T> NumOps<Rhs, Out> for T where
    T: Add<Rhs, Output = Out>
        + Sub<Rhs, Output = Out>
        + Mul<Rhs, Output = Out>
        + Div<Rhs, Output = Out>
        + Rem<Rhs, Output = Out>
{
}

/// Trait combining the common arithmetic assignment operations. Blanket implemented for all valid
/// types.
pub trait NumAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}

impl<Rhs, T> NumAssignOps<Rhs> for T where
    T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}

/// Trait combining all bitwise operations. Blanket implemented for all valid types.
pub trait BitOps<Rhs = Self, Out = Self>:
    Not<Output = Out> + BitAnd<Rhs, Output = Out> + BitOr<Rhs, Output = Out> + BitXor<Rhs, Output = Out>
{
}

impl<Rhs, Out, T> BitOps<Rhs, Out> for T where
    T: Not<Output = Out>
        + BitAnd<Rhs, Output = Out>
        + BitOr<Rhs, Output = Out>
        + BitXor<Rhs, Output = Out>
{
}

/// Trait combining all bitwise assignment operations. Blanket implemented for all valid types.
pub trait BitAssignOps<Rhs = Self>:
    BitAndAssign<Rhs> + BitOrAssign<Rhs> + BitXorAssign<Rhs>
{
}

impl<Rhs, T> BitAssignOps<Rhs> for T where
    T: BitAndAssign<Rhs> + BitOrAssign<Rhs> + BitXorAssign<Rhs>
{
}

/// Trait combining all shift operations. Blanket implemented for all valid types.
pub trait ShiftOps<Rhs = Self, Out = Self>:
    Shl<Rhs, Output = Out> + Shr<Rhs, Output = Out>
{
}

impl<Rhs, Out, T> ShiftOps<Rhs, Out> for T where T: Shl<Rhs, Output = Out> + Shr<Rhs, Output = Out> {}

/// Trait combining all shift assignment operations. Blanket implemented for all valid types.
pub trait ShiftAssignOps<Rhs = Self>: ShlAssign<Rhs> + ShrAssign<Rhs> {}

impl<Rhs, T> ShiftAssignOps<Rhs> for T where T: ShlAssign<Rhs> + ShrAssign<Rhs> {}
