use crate::identity::{One, Zero};
use crate::ops::core::{BitOps, NumOps, ShiftOps};
use crate::ops::Pow;
use core::ops::Neg;

/// Trait for generic number-like types. These are types that can generally be thought of as
/// representing values on the number line - they support common numeric operations as well
/// as being comparable, and having a [multiplicative][Zero] and [additive][One] identity.
pub trait Numeric:
    PartialEq + PartialOrd + Clone + Zero + One + NumOps + Pow<Output = Self>
{
}

/// Trait for types that are unsigned. These types can only represent non-negative values
pub trait Unsigned {}

/// Trait for types that are signed. These types can represent both positive and negative values
pub trait Signed: Neg<Output = Self> {
    /// Get the absolute value of this number. This is the positive form of the current value,
    /// or the distance of the value from 0
    fn abs(self) -> Self;

    /// Whether this value is positive (`>= 0`)
    fn is_positive(&self) -> bool;

    /// Whether this value is negative (`< 0`)
    fn is_negative(&self) -> bool;
}

/// Trait for types that are 'integer like'. These types should only represent whole numbers,
/// and are expected to support bitwise and shifting operations. An example of a type implementing
/// this trait would be [`u64`] or [`i64`].
pub trait Integral: Numeric + BitOps + ShiftOps + ShiftOps<usize> {}

/// Trait for types that are 'real-number like'. These types represent the real numbers,
/// and _generally_ don't support bitwise operations. An example of a type implementing this
/// trait would be [`f32`]
pub trait Real: Numeric {
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;

    fn trunc(self) -> Self;
    fn fract(self) -> Self;

    fn log(self, base: Self) -> Self;

    fn root(self, base: Self) -> Self {
        Self::pow(self, Self::one() / base)
    }

    fn sqrt(self) -> Self {
        Self::pow(self, Self::one() / (Self::one() + Self::one()))
    }

    fn cbrt(self) -> Self {
        Self::pow(self, Self::one() / (Self::one() + Self::one() + Self::one()))
    }
}

pub enum FloatClass {
    Nan,
    Infinite,
    Zero,
    Subnormal,
    Normal,
}

/// Trait for types that are floating point - real numbers that specifically use the float format,
/// and thus have `NaN` and `Inf` values.
pub trait Float: Real {
    fn nan() -> Self;
    fn inf() -> Self;
    fn neg_inf() -> Self;
    fn epsilon() -> Self;

    fn is_nan(&self) -> bool;
    fn is_inf(&self) -> bool;
    fn is_normal(&self) -> bool;
    fn classify(&self) -> FloatClass;
}

/// Trait for types that are 'bounded' - they represent only a finite range of numbers, and thus
/// have a minimum and maximum representable value.
pub trait Bounded {
    /// The minimum representable value for this type
    fn min_value() -> Self;

    /// The maximum representable value for this type
    fn max_value() -> Self;
}

/// Trait for types that are bounded and signed, and as such users may want to distinguish between
/// their minimum value, and their minimum positive value.
pub trait BoundedSigned: Bounded + Signed {
    /// The minimum value representable by this type that is greater than 0
    fn min_positive() -> Self;

    /// The maximum value representable by this type that is less than 0
    fn max_negative() -> Self;
}

pub trait BoundedBit: Bounded {
    fn leading_zeros(self) -> Self;
    fn trailing_zeros(self) -> Self;
}
