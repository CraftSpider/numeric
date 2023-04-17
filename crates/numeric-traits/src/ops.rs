pub mod checked;
pub mod core;
pub mod overflowing;
pub mod saturating;
pub mod wrapping;

pub trait Pow<Rhs = Self> {
    type Output;

    fn pow(self, rhs: Rhs) -> Self::Output;
}
