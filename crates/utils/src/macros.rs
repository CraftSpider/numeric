
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
    ($ty:ty: $trait:ident $( + $traits:ident )*) => {
        const _: () = {
            const fn __check<T: $trait $( + $traits )*>() {}
            __check::<$ty>();
        };
    };
}
