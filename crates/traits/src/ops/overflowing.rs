
pub trait OverflowingOps<Rhs = Self, Out = Self>:
    OverflowingAdd<Rhs, Output = Out>
    + OverflowingSub<Rhs, Output = Out>
    + OverflowingMul<Rhs, Output = Out>
{
}

impl<Rhs, Out, T> OverflowingOps<Rhs, Out> for T
where
    T: OverflowingAdd<Rhs, Output = Out>
    + OverflowingSub<Rhs, Output = Out>
    + OverflowingMul<Rhs, Output = Out>
{}

pub trait OverflowingAdd<Rhs = Self> {
    type Output;

    fn overflowing_add(self, rhs: Rhs) -> (Self::Output, bool);
}

pub trait OverflowingSub<Rhs = Self> {
    type Output;

    fn overflowing_sub(self, rhs: Rhs) -> (Self::Output, bool);
}

pub trait OverflowingMul<Rhs = Self> {
    type Output;

    fn overflowing_mul(self, rhs: Rhs) -> (Self::Output, bool);
}
