//! Common mathematical and programatical operators. This extends [`core::ops`] with many traits
//! representing more specific or less common operators.

pub mod checked;
pub mod core;
pub mod overflowing;
pub mod saturating;
pub mod widening;
pub mod wrapping;

/// The power, or exponent, operator. Generally represented as `xⁿ`.
pub trait Pow<Rhs = Self> {
    /// The type produced by applying this operation
    type Output;

    /// Bring this value to a given power. This is equivalent to calling [`Mul`][core::ops::Mul] on
    /// the value, `Rhs` many times.
    fn pow(self, rhs: Rhs) -> Self::Output;
}

/// The factorial operator. Generally represented as `x!`. If this operator is implemented on a
/// non-integer type, it should generally instead be equivalent to the
/// [Gamma function](https://en.wikipedia.org/wiki/Gamma_function).
pub trait Factorial {
    /// The type produced by applying this operation
    type Output;

    /// Generate the factorial of this value. This is equivalent to calling [`Mul`][core::ops::Mul]
    /// on this value, its value minus one, that value minus one, all the way down to zero. EG,
    /// `5!` is equivalent to `5*4*3*2*1`.
    fn factorial(self) -> Self::Output;
}

/// The greatest-common-denominator (sometimes called 'greatest common factor' or 'greatest common
/// multiple') operation. Gets the largest integer `N` such that for `a` and `b`, `a / N` and
/// `b / N` are integers.
pub trait Gcd<Rhs = Self> {
    type Output;

    fn gcd(self, other: Rhs) -> Self::Output;
}

/// The common trigonometric operators. These can be understood geometrically as
/// various values for a given angle in relation to the unit circle (a circle of radius 1).
/// Each of the common functions has an inverse equivalent.
///
/// The following is a graphical representation of the various trigonometric functions on a circle:
/// ![](https://upload.wikimedia.org/wikipedia/commons/4/46/Unit_Circle_Definitions_of_Six_Trigonometric_Functions.svg)
pub trait TrigOps {
    /// The `sine` function.
    ///
    /// ## Right-angle Triangle
    /// Equal to the side adjacent the given angle over the hypotenuse.
    ///
    /// ## Unit-circle
    /// The Y part of the vector from the center of the unit circle to its edge at the given angle.
    fn sin(self) -> Self;

    /// The `cosine` function.
    ///
    /// ## Right-angle Triangle
    /// Equal to the side opposite the given angle over the hypotenuse.
    ///
    /// ## Unit-circle
    /// The X part of the vector from the center of the unit circle to its edge at a given angle.
    fn cos(self) -> Self;

    /// The `tangent` function, equal to `x.sin()/x.cos()`.
    ///
    /// ## Right-angle Triangle
    /// Equal to the side opposite the given angle over the side adjacent to the given angle.
    ///
    /// ## Unit-circle
    /// The length where the ray from the center of the unit circle to its edge at a given angle has
    /// an X value equal to 1.
    fn tan(self) -> Self;

    /// The `cosecant` function, the multiplicative inverse of `sine`.
    ///
    /// ## Right-angle Triangle
    /// Equal to the hypotenuse over the side adjacent the given angle.
    ///
    /// ## Unit-circle
    /// Given the tangent line to a point at a given angle on the unit circle, the Y value where
    /// the line touches the Y axis.
    fn csc(self) -> Self;

    /// The `secant` function, the multiplicative inverse of `cosine`.
    ///
    /// ## Right-angle Triangle
    /// Equal to the hypotenuse over the side opposite the given angle.
    ///
    /// ## Unit-circle
    /// Given the tangent line to a point at a given angle on the unit circle, the X value where
    /// the line touches the X axis.
    fn sec(self) -> Self;

    /// The `cotangent` function, the multiplicative inverse of `tangent`.
    ///
    /// ## Right-angle Triangle
    /// Equal to the side adjacent to the given angle over the side opposite the given angle.
    ///
    /// ## Unit-circle
    /// The length where the ray from the center of the unit circle towards its edge at a given
    /// angle has a Y value equal to 1.
    fn cot(self) -> Self;
}

pub trait HypTrigOps {
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;

    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;
}
