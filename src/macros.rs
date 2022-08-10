
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
