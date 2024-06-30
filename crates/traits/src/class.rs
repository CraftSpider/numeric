//! Traits connected to wide-reaching classes of numeric types. The base of all numbers is
//! [`Numeric`], a trait for things that support common math operations.

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
/// trait would be [`f32`].
///
/// ## Important Note
///
/// Real, in this case, is not exactly equivalent to the real numbers. It's actually closer to
/// the non-negative reals. If you want the actual real numbers, you probably want [`RealSigned`]
pub trait Real: Numeric {
    /// Round this number towards 0
    fn floor(self) -> Self;
    /// Round this number away from 0
    fn ceil(self) -> Self;
    /// Round this number to the nearest whole number
    fn round(self) -> Self;

    /// Truncate this number, rounding towards negative infinity
    fn trunc(self) -> Self;
    /// Get just the fractional part of this number, this is effectively similar to doing
    /// `x - x.trunc()`
    fn fract(self) -> Self;

    /// Evaluate the logarithm of this number in a specified base
    fn log(self, base: Self) -> Self;

    /// Get the root of this number in a specified base
    fn root(self, base: Self) -> Self {
        Self::pow(self, Self::one() / base)
    }

    /// The square root of this number
    fn sqrt(self) -> Self {
        Self::pow(self, Self::one() / (Self::one() + Self::one()))
    }

    /// The cube root of this number
    fn cbrt(self) -> Self {
        Self::pow(self, Self::one() / (Self::one() + Self::one() + Self::one()))
    }
}

/// Trait for types that are both [`Real`] and [`Signed`].
pub trait RealSigned: Real + Signed {}

/// The 'kind' of a float, representing the possible types of values a float can hold
pub enum FloatClass {
    /// A `NaN` float, representing not-a-number
    Nan,
    /// An infinite float value, positive or negative
    Infinite,
    /// One of the two zero float values - positive or negative
    Zero,
    /// A 'subnormal' float. A value containing leading zeroes in the significand/mantissa.
    Subnormal,
    /// A normal float value. This is most numbers.
    Normal,
}

/// Trait for types that are floating point - real numbers that specifically use the float format,
/// and thus have `NaN` and `Inf` values.
pub trait Float: Real {
    /// An arbitrary `NaN` value for this float type
    fn nan() -> Self;
    /// The `inf` value for this float type
    fn inf() -> Self;
    /// The `-inf` value for this float type
    fn neg_inf() -> Self;
    /// The smallest possible value for this float greater than zero
    fn epsilon() -> Self;

    /// Check whether this value is `NaN`
    fn is_nan(&self) -> bool;
    /// Check whether this value is `inf` or `-inf`
    fn is_inf(&self) -> bool;
    /// Check whether this value is 'normal' - not zero, nan, infinity, or subnormal.
    fn is_normal(&self) -> bool;
    /// Classify this float - see [`FloatClass`] for the possible classes of a float value.
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

/// Trait for types that are bounded and support bitwise operations, and thus can have their
/// leading/trailing bits operated on.
pub trait BoundedBit: Bounded {
    /// The number of 0 bits, starting from the MSB
    fn leading_zeros(self) -> Self;
    /// The number of 0 bits, starting from the LSB
    fn trailing_zeros(self) -> Self;
}
