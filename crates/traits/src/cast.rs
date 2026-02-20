//! Traits for numeric conversions - distinct operations are offered for different
//! kinds of conversions.

/// Trait for types that can be read from a string containing a representation of this number in a
/// given base. For simple values such as ints, that would look like `-24`, while for complex
/// numbers, that might look like `3.4 - 2.5i`.
pub trait FromStrRadix: Sized {
    /// The error type returned if parsing fails
    type Error;

    /// Parse a string in a given base and return the value it represent, or `Err` if it isn't valid
    /// for this type.
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::Error>;
}

/// Similar to [`TryFrom`], but for numeric types. This trait has the advantage of no blanket
/// `From<T>` impl, so one can apply their own blanket implementations. Implementations of this
/// trait should only succeed if the provided value is within the range of the type. For example,
/// `u16::from_checked(u16::MAX as u32 + 1)` or `u16::from_checked(-1)` are expected to return
/// `None`.
pub trait FromChecked<T>: Sized {
    /// Attempt to create this type from an instance of another, returning `None` if the value is
    /// outside the range of this type.
    fn from_checked(val: T) -> Option<Self>;
}

/// Similar to [`TryInto`], but for numeric types. Users should prefer implementing [`FromChecked`]
pub trait IntoChecked<T> {
    /// Attempt to convert this value into `T`, returning `None` if the value is outside the valid
    /// range.
    fn into_checked(self) -> Option<T>;
}

impl<T, U> IntoChecked<U> for T
where
    U: FromChecked<T>,
{
    fn into_checked(self) -> Option<U> {
        U::from_checked(self)
    }
}

/// Trait for numeric types that can be converted between, rounding to the nearest valid value if
/// the provided instance is out of range. For integer conversions, this means becoming `T::MIN` or
/// `T::MAX` if the other integer is out-of-range. For floats, this means rounding to the nearest
/// value.
pub trait FromSaturating<T> {
    /// Create this type from an instance of another, returning the nearest value if it cannot be
    /// represented exactly.
    fn saturate_from(val: T) -> Self;
}

/// Trait for numeric types that can be converted between, rounding to the nearest valid value if
/// the provided instance is out of range. Users should prefer implementing [`FromSaturating`].
pub trait IntoSaturating<T> {
    /// Convert this value into `T`, rounding to the nearest valid value if it cannot be represented
    /// exactly.
    fn saturate(self) -> T;
}

impl<T, U> IntoSaturating<U> for T
where
    U: FromSaturating<T>,
{
    fn saturate(self) -> U {
        U::saturate_from(self)
    }
}

/// Trait for numeric types that can be converted between, truncating if the provided instance is
/// out of range. For integer conversions, this tends to mean simply cutting off the high bits,
/// but users shouldn't rely on this behavior.
pub trait FromTruncating<T> {
    /// Create this type from an instance of another, truncating if the value is out of range.
    fn truncate_from(val: T) -> Self;
}

/// Trait for numeric types that can be converted between, truncating if the provided instance is
/// out of range. Users should prefer implementing [`FromTruncating`].
pub trait IntoTruncating<T> {
    /// Convert this value into `T`, truncating if the value is out of range.
    fn truncate(self) -> T;
}

impl<T, U> IntoTruncating<U> for T
where
    U: FromTruncating<T>,
{
    fn truncate(self) -> U {
        U::truncate_from(self)
    }
}

/// Trait combining [`FromChecked`], [`FromSaturating`], and [`FromTruncating`].
pub trait FromAll<T>: FromChecked<T> + FromSaturating<T> + FromTruncating<T> {}

impl<T, U> FromAll<U> for T where T: FromChecked<U> + FromSaturating<U> + FromTruncating<U> {}

/// Trait for types that support checked conversion from primitive integer types.
pub trait FromPrimChecked:
    FromChecked<u8>
    + FromChecked<u16>
    + FromChecked<u32>
    + FromChecked<u64>
    + FromChecked<i8>
    + FromChecked<i16>
    + FromChecked<i32>
    + FromChecked<i64>
{
}

impl<T> FromPrimChecked for T where
    T: FromChecked<u8>
        + FromChecked<u16>
        + FromChecked<u32>
        + FromChecked<u64>
        + FromChecked<i8>
        + FromChecked<i16>
        + FromChecked<i32>
        + FromChecked<i64>
{
}

/// Trait for types that support all conversions from primitive integer types.
pub trait FromPrim:
    FromAll<u8>
    + FromAll<u16>
    + FromAll<u32>
    + FromAll<u64>
    + FromAll<i8>
    + FromAll<i16>
    + FromAll<i32>
    + FromAll<i64>
{
}

impl<T> FromPrim for T where
    T: FromAll<u8>
        + FromAll<u16>
        + FromAll<u32>
        + FromAll<u64>
        + FromAll<i8>
        + FromAll<i16>
        + FromAll<i32>
        + FromAll<i64>
{
}
