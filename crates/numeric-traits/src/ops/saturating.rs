
pub trait SaturatingOps<Rhs = Self, Out = Self>:
    SaturatingAdd<Rhs, Output = Out>
    + SaturatingSub<Rhs, Output = Out>
    + SaturatingMul<Rhs, Output = Out>
{}

impl<Rhs, Out, T> SaturatingOps<Rhs, Out> for T
where
    T: SaturatingAdd<Rhs, Output = Out>
    + SaturatingSub<Rhs, Output = Out>
    + SaturatingMul<Rhs, Output = Out>
{}

pub trait SaturatingAdd<Rhs = Self> {
    type Output;

    fn saturating_add(self, rhs: Rhs) -> Self::Output;
}

pub trait SaturatingSub<Rhs = Self> {
    type Output;

    fn saturating_sub(self, rhs: Rhs) -> Self::Output;
}

pub trait SaturatingMul<Rhs = Self> {
    type Output;

    fn saturating_mul(self, rhs: Rhs) -> Self::Output;
}
