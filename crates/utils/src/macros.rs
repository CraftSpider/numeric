#[macro_export]
macro_rules! static_assert {
    ($expr:expr) => {
        const _: () = assert!($expr);
    };
    ($expr:expr, $msg:literal) => {
        const _: () = assert!($expr, $msg);
    };
}

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
