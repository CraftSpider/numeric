//! Traits for identity operations. This includes the [additive][Zero] and [multiplicative][One]
//! identities,

/// The additive identity. Should be zero for [`Numeric`][crate::class::Numeric] types.
pub trait Zero {
    /// Get the zero value for this type
    fn zero() -> Self;
    /// Check whether an instance of this type is zero.
    fn is_zero(&self) -> bool;
}

/// The multiplicative identity. Should be one for [`Numeric`][crate::class::Numeric] types.
pub trait One {
    /// Get the one value for this type
    fn one() -> Self;
    /// Check whether an instance of this type is one.
    fn is_one(&self) -> bool;
}

/// Common, useful constant values. These constants are 'real' because they'll likely not have
/// meaningfully useful representations in the integer numbers.
pub trait RealConsts {
    /// Pi, the circle equation constant
    fn pi() -> Self;
    /// E, base of the natural log
    fn e() -> Self;
}
