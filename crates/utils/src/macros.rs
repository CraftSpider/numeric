/// Create a static assertion, failing compilation if the expression is false.
#[macro_export]
macro_rules! static_assert {
    ($expr:expr) => {
        const _: () = assert!($expr);
    };
    ($expr:expr, $msg:literal) => {
        const _: () = assert!($expr, $msg);
    };
}

/// Create a static assertion that a type implements a set of traits. Generics can be used to ensure
/// the type blanket implements the trait.
///
/// # Examples
/// ```
/// # use numeric_utils::static_assert_traits;
/// # use std::fmt::Debug;
/// static_assert_traits!(u8: Send + Sync);
/// static_assert_traits!([T: Debug] Vec<T>: Debug);
/// ```
#[macro_export]
macro_rules! static_assert_traits {
    ([$($bounds:tt)*] $ty:ty: $( $traits:tt )*) => {
        const _: () = {
            const fn __check<T: $( $traits )*>() {}
            const fn __check_outer<$($bounds)*>() {
                __check::<$ty>();
            }
        };
    };
    ($ty:ty: $( $traits:tt )*) => {
        const _: () = {
            const fn __check<T: $( $traits )*>() {}
            __check::<$ty>();
        };
    };
}
