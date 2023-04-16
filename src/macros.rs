
#[cfg(feature = "__bench_priv")]
macro_rules! bench_public {
    ($($tt:tt)*) => {
        pub $($tt)*
    }
}

#[cfg(not(feature = "__bench_priv"))]
macro_rules! bench_public {
    ($($tt:tt)*) => {
        $($tt)*
    }
}

#[cfg(feature = "specialize")]
macro_rules! maybe_default {
    (fn $($tt:tt)*) => {
        default fn $($tt)*
    }
}

#[cfg(not(feature = "specialize"))]
macro_rules! maybe_default {
    (fn $($tt:tt)*) => {
        fn $($tt)*
    }
}

macro_rules! static_assert {
    ($expr:expr) => {
        const _: () = assert!($expr);
    };
    ($expr:expr, $msg:literal) => {
        const _: () = assert!($expr, $msg);
    };
}

macro_rules! static_assert_traits {
    ($ty:ty: $trait:ident $( + $traits:ident )*) => {
        const _: () = {
            const fn __check<T: $trait $( + $traits )*>() {}
            __check::<$ty>();
        };
    };
}
