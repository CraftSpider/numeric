
pub trait FromStrRadix: Sized {
    type Error;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::Error>;
}

pub trait FromChecked<T>: Sized {
    fn from_checked(val: T) -> Option<Self>;
}

pub trait FromSaturating<T> {
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
