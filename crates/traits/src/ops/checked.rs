pub trait CheckedAdd<Rhs = Self> {
    type Output;

    fn checked_add(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedSub<Rhs = Self> {
    type Output;

    fn checked_sub(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedMul<Rhs = Self> {
    type Output;

    fn checked_mul(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedDiv<Rhs = Self> {
    type Output;

    fn checked_div(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedShl<Rhs = Self> {
    type Output;

    fn checked_shl(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedShr<Rhs = Self> {
    type Output;

    fn checked_shr(self, rhs: Rhs) -> Option<Self::Output>;
}

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
