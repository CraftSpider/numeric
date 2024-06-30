use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

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

pub trait NumAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}

impl<Rhs, T> NumAssignOps<Rhs> for T where
    T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}

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

pub trait BitAssignOps<Rhs = Self>:
    BitAndAssign<Rhs> + BitOrAssign<Rhs> + BitXorAssign<Rhs>
{
}

impl<Rhs, T> BitAssignOps<Rhs> for T where
    T: BitAndAssign<Rhs> + BitOrAssign<Rhs> + BitXorAssign<Rhs>
{
}

pub trait ShiftOps<Rhs = Self, Out = Self>:
    Shl<Rhs, Output = Out> + Shr<Rhs, Output = Out>
{
}

impl<Rhs, Out, T> ShiftOps<Rhs, Out> for T where T: Shl<Rhs, Output = Out> + Shr<Rhs, Output = Out> {}

pub trait ShiftAssignOps<Rhs = Self>: ShlAssign<Rhs> + ShrAssign<Rhs> {}

impl<Rhs, T> ShiftAssignOps<Rhs> for T where T: ShlAssign<Rhs> + ShrAssign<Rhs> {}
