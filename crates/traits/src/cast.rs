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

/// Trait for numeric types that can be converted between, rounding to the nearest valid value if
/// the provided instance is out of range. For integer conversions, this means becoming `T::MIN` or
/// `T::MAX` if the other integer is out-of-range. For floats, this means rounding to the nearest
/// value.
pub trait FromSaturating<T> {
    /// Create this type from an instance of another, returning the nearest value if it cannot be
    /// represented exactly.
    fn saturate(val: T) -> Self;
}

pub trait FromTruncating<T> {
    fn truncate(val: T) -> Self;
}

pub trait FromApproximating<T> {
    fn approx(val: T) -> Self;
}

pub trait FromAll<T>: FromChecked<T> + FromSaturating<T> + FromTruncating<T> {}

impl<T, U> FromAll<U> for T
where
    T: FromChecked<U> + FromSaturating<U> + FromTruncating<U>
{}

pub trait FromPrimChecked:
    FromChecked<u8>
    + FromChecked<u16>
    + FromChecked<u32>
    + FromChecked<u64>
    + FromChecked<i8>
    + FromChecked<i16>
    + FromChecked<i32>
    + FromChecked<i64>
{}

impl<T> FromPrimChecked for T
where
    T: FromChecked<u8>
    + FromChecked<u16>
    + FromChecked<u32>
    + FromChecked<u64>
    + FromChecked<i8>
    + FromChecked<i16>
    + FromChecked<i32>
    + FromChecked<i64>
{}

pub trait FromPrim:
    FromAll<u8>
    + FromAll<u16>
    + FromAll<u32>
    + FromAll<u64>
    + FromAll<i8>
    + FromAll<i16>
    + FromAll<i32>
    + FromAll<i64>
{}

impl<T> FromPrim for T
where
    T: FromAll<u8>
    + FromAll<u16>
    + FromAll<u32>
    + FromAll<u64>
    + FromAll<i8>
    + FromAll<i16>
    + FromAll<i32>
    + FromAll<i64>
{}
