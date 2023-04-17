macro_rules! impl_bytes {
    ($ty:ty) => {
        const _: () = {
            const SIZE: usize = core::mem::size_of::<$ty>();

            impl crate::bytes::ConvertBytes<SIZE> for $ty {
                fn from_ne_bytes(bytes: [u8; SIZE]) -> Self {
                    <$ty>::from_ne_bytes(bytes)
                }

                fn from_be_bytes(bytes: [u8; SIZE]) -> Self {
                    <$ty>::from_be_bytes(bytes)
                }

                fn from_le_bytes(bytes: [u8; SIZE]) -> Self {
                    <$ty>::from_le_bytes(bytes)
                }

                fn to_ne_bytes(self) -> [u8; SIZE] {
                    <$ty>::to_ne_bytes(self)
                }

                fn to_be_bytes(self) -> [u8; SIZE] {
                    <$ty>::to_be_bytes(self)
                }

                fn to_le_bytes(self) -> [u8; SIZE] {
                    <$ty>::to_le_bytes(self)
                }
            }
        };
    };
}

macro_rules! truncating_as {
    ($into:ty, $from:ty) => {
        impl crate::cast::FromTruncating<$from> for $into {
            fn truncate(val: $from) -> Self {
                val as $into
            }
        }
    };
}

macro_rules! impl_int {
    (
        $ty:ty
    ) => {
        impl crate::class::Numeric for $ty {}

        impl crate::class::Integral for $ty {}

        impl crate::class::Bounded for $ty {
            fn max_value() -> Self {
                <$ty>::MAX
            }

            fn min_value() -> Self {
                <$ty>::MIN
            }
        }

        impl crate::identity::Zero for $ty {
            fn zero() -> Self {
                0
            }

            fn is_zero(&self) -> bool {
                *self == 0
            }
        }

        impl crate::identity::One for $ty {
            fn one() -> Self {
                1
            }

            fn is_one(&self) -> bool {
                *self == 1
            }
        }

        impl crate::ops::Pow for $ty {
            type Output = $ty;

            fn pow(self, rhs: Self) -> Self::Output {
                <$ty>::pow(self, rhs as u32)
            }
        }

        impl crate::ops::wrapping::WrappingAdd for $ty {
            type Output = $ty;

            fn wrapping_add(self, rhs: Self) -> Self::Output {
                <$ty>::wrapping_sub(self, rhs)
            }
        }

        impl crate::ops::wrapping::WrappingSub for $ty {
            type Output = $ty;

            fn wrapping_sub(self, rhs: Self) -> Self::Output {
                <$ty>::wrapping_sub(self, rhs)
            }
        }

        impl crate::ops::wrapping::WrappingMul for $ty {
            type Output = $ty;

            fn wrapping_mul(self, rhs: Self) -> Self::Output {
                <$ty>::wrapping_mul(self, rhs)
            }
        }

        impl crate::ops::wrapping::WrappingShl for $ty {
            type Output = $ty;

            fn wrapping_shl(self, rhs: Self) -> Self::Output {
                <$ty>::wrapping_shl(self, rhs as u32)
            }
        }

        impl crate::ops::wrapping::WrappingShr for $ty {
            type Output = $ty;

            fn wrapping_shr(self, rhs: Self) -> Self::Output {
                <$ty>::wrapping_shr(self, rhs as u32)
            }
        }

        impl crate::ops::overflowing::OverflowingAdd for $ty {
            type Output = $ty;

            fn overflowing_add(self, rhs: Self) -> (Self::Output, bool) {
                <$ty>::overflowing_add(self, rhs)
            }
        }

        impl crate::ops::overflowing::OverflowingSub for $ty {
            type Output = $ty;

            fn overflowing_sub(self, rhs: Self) -> (Self::Output, bool) {
                <$ty>::overflowing_sub(self, rhs)
            }
        }

        impl crate::ops::overflowing::OverflowingMul for $ty {
            type Output = $ty;

            fn overflowing_mul(self, rhs: Self) -> (Self::Output, bool) {
                <$ty>::overflowing_mul(self, rhs)
            }
        }

        truncating_as!($ty, u8);
        truncating_as!($ty, u16);
        truncating_as!($ty, u32);
        truncating_as!($ty, u64);
        truncating_as!($ty, u128);
        truncating_as!($ty, usize);

        truncating_as!($ty, i8);
        truncating_as!($ty, i16);
        truncating_as!($ty, i32);
        truncating_as!($ty, i64);
        truncating_as!($ty, i128);
        truncating_as!($ty, isize);

        impl crate::cast::FromApproximating<f32> for $ty {
            fn approx(val: f32) -> Self {
                val as $ty
            }
        }

        impl crate::cast::FromApproximating<f64> for $ty {
            fn approx(val: f64) -> Self {
                val as $ty
            }
        }

        impl_bytes!($ty);
    };
}

macro_rules! impl_uint {
    ($ty:ty) => {
        impl crate::class::Unsigned for $ty {}
    };
}

macro_rules! impl_sint {
    ($ty:ty) => {
        impl crate::class::Signed for $ty {
            fn abs(self) -> Self {
                <$ty>::abs(self)
            }

            fn is_positive(&self) -> bool {
                <$ty>::is_positive(*self)
            }

            fn is_negative(&self) -> bool {
                <$ty>::is_negative(*self)
            }
        }
    };
}

impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);
impl_int!(u128);
impl_int!(usize);

impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(i128);
impl_int!(isize);

impl_uint!(u8);
impl_uint!(u16);
impl_uint!(u32);
impl_uint!(u64);
impl_uint!(u128);
impl_uint!(usize);

impl_sint!(i8);
impl_sint!(i16);
impl_sint!(i32);
impl_sint!(i64);
impl_sint!(i128);
impl_sint!(isize);

macro_rules! impl_float {
    ($ty:ty) => {
        impl crate::class::Numeric for $ty {}

        impl crate::class::Signed for $ty {
            fn abs(self) -> Self {
                <$ty>::abs(self)
            }

            fn is_negative(&self) -> bool {
                *self < 0.0
            }

            fn is_positive(&self) -> bool {
                *self >= 0.0
            }
        }

        impl crate::class::Real for $ty {
            fn floor(self) -> Self {
                <$ty>::floor(self)
            }

            fn ceil(self) -> Self {
                <$ty>::ceil(self)
            }

            fn round(self) -> Self {
                <$ty>::round(self)
            }

            fn trunc(self) -> Self {
                <$ty>::trunc(self)
            }

            fn fract(self) -> Self {
                <$ty>::fract(self)
            }

            fn log(self, base: Self) -> Self {
                <$ty>::log(self, base)
            }

            fn sqrt(self) -> Self {
                <$ty>::sqrt(self)
            }
        }

        impl crate::class::Bounded for $ty {
            fn max_value() -> Self {
                <$ty>::MAX
            }

            fn min_value() -> Self {
                <$ty>::MIN
            }
        }

        impl crate::class::BoundedSigned for $ty {
            fn min_positive() -> Self {
                <$ty>::MIN_POSITIVE
            }

            fn max_negative() -> Self {
                -<$ty>::MIN_POSITIVE
            }
        }

        impl crate::identity::Zero for $ty {
            fn zero() -> Self {
                0.0
            }

            fn is_zero(&self) -> bool {
                *self == 0.0
            }
        }

        impl crate::identity::One for $ty {
            fn one() -> Self {
                1.0
            }

            fn is_one(&self) -> bool {
                *self == 1.0
            }
        }

        impl crate::ops::Pow for $ty {
            type Output = $ty;

            fn pow(self, rhs: Self) -> Self::Output {
                <$ty>::powf(self, rhs)
            }
        }

        impl_bytes!($ty);
    };
}

impl_float!(f32);
impl_float!(f64);

macro_rules! saturate_uint_impl {
    (
        $ty:ty,
        u_less_eq = [$($u_less:ty),*],
        u_greater = [$($u_greater:ty),*],
        s_less_eq = [$($s_less:ty),*],
        s_greater = [$($s_greater:ty),*],
    ) => {
        $(
        impl crate::cast::FromSaturating<$u_less> for $ty {
            fn saturate(val: $u_less) -> Self {
                val as $ty
            }
        }
        )*

        $(
        impl crate::cast::FromSaturating<$u_greater> for $ty {
            fn saturate(val: $u_greater) -> Self {
                if val > <$ty>::MAX as $u_greater {
                    <$ty>::MAX
                } else {
                    val as $ty
                }
            }
        }
        )*

        $(
        impl crate::cast::FromSaturating<$s_less> for $ty {
            fn saturate(val: $s_less) -> Self {
                if val < 0 {
                    <$ty>::MIN
                } else {
                    val as $ty
                }
            }
        }
        )*

        $(
        impl crate::cast::FromSaturating<$s_greater> for $ty {
            fn saturate(val: $s_greater) -> Self {
                if val < 0 {
                    <$ty>::MIN
                } else if val > <$ty>::MAX as $s_greater {
                    <$ty>::MAX
                } else {
                    val as $ty
                }
            }
        }
        )*
    }
}

saturate_uint_impl!(
    u8,
    u_less_eq = [u8],
    u_greater = [u16, u32, u64, u128, usize],
    s_less_eq = [i8],
    s_greater = [i16, i32, i64, i128, isize],
);

saturate_uint_impl!(
    u16,
    u_less_eq = [u8, u16],
    u_greater = [u32, u64, u128],
    s_less_eq = [i8, i16],
    s_greater = [i32, i64, i128],
);

saturate_uint_impl!(
    u32,
    u_less_eq = [u8, u16, u32],
    u_greater = [u64, u128],
    s_less_eq = [i8, i16, i32],
    s_greater = [i64, i128],
);

saturate_uint_impl!(
    u64,
    u_less_eq = [u8, u16, u32, u64],
    u_greater = [u128],
    s_less_eq = [i8, i16, i32, i64],
    s_greater = [i128],
);

saturate_uint_impl!(
    u128,
    u_less_eq = [u8, u16, u32, u64, u128],
    u_greater = [],
    s_less_eq = [i8, i16, i32, i64, i128],
    s_greater = [],
);

macro_rules! saturate_sint_impl {
    (
        $ty:ty,
        u_less = [$($u_less:ty),*],
        u_greater_eq = [$($u_greater:ty),*],
        s_less_eq = [$($s_less:ty),*],
        s_greater = [$($s_greater:ty),*],
    ) => {
        $(
        impl crate::cast::FromSaturating<$u_less> for $ty {
            fn saturate(val: $u_less) -> Self {
                val as $ty
            }
        }
        )*

        $(
        impl crate::cast::FromSaturating<$u_greater> for $ty {
            fn saturate(val: $u_greater) -> Self {
                if val > <$ty>::MAX as $u_greater {
                    <$ty>::MAX
                } else {
                    val as $ty
                }
            }
        }
        )*

        $(
        impl crate::cast::FromSaturating<$s_less> for $ty {
            fn saturate(val: $s_less) -> Self {
                val as $ty
            }
        }
        )*

        $(
        impl crate::cast::FromSaturating<$s_greater> for $ty {
            fn saturate(val: $s_greater) -> Self {
                if val > <$ty>::MAX as $s_greater {
                    <$ty>::MAX
                } else if val < <$ty>::MIN as $s_greater {
                    <$ty>::MIN
                } else {
                    val as $ty
                }
            }
        }
        )*
    }
}

saturate_sint_impl!(
    i8,
    u_less = [],
    u_greater_eq = [u8, u16, u32, u64, u128, usize],
    s_less_eq = [i8],
    s_greater = [i16, i32, i64, i128],
);

saturate_sint_impl!(
    i16,
    u_less = [u8],
    u_greater_eq = [u16, u32, u64, u128],
    s_less_eq = [i8, i16],
    s_greater = [i32, i64, i128],
);

saturate_sint_impl!(
    i32,
    u_less = [u8, u16],
    u_greater_eq = [u32, u64, u128],
    s_less_eq = [i8, i16, i32],
    s_greater = [i64, i128],
);

saturate_sint_impl!(
    i64,
    u_less = [u8, u16, u32],
    u_greater_eq = [u64, u128],
    s_less_eq = [i8, i16, i32, i64],
    s_greater = [i128],
);

saturate_sint_impl!(
    i128,
    u_less = [u8, u16, u32, u64],
    u_greater_eq = [u128],
    s_less_eq = [i8, i16, i32, i64, i128],
    s_greater = [],
);

// TODO: {u, i}size saturating

macro_rules! checked_impl {
    ($ty:ty) => {
        checked_impl!(@ $ty, u8);
        checked_impl!(@ $ty, u16);
        checked_impl!(@ $ty, u32);
        checked_impl!(@ $ty, u64);
        checked_impl!(@ $ty, u128);
        checked_impl!(@ $ty, usize);

        checked_impl!(@ $ty, i8);
        checked_impl!(@ $ty, i16);
        checked_impl!(@ $ty, i32);
        checked_impl!(@ $ty, i64);
        checked_impl!(@ $ty, i128);
        checked_impl!(@ $ty, isize);
    };
    (@ $ty:ty, $from:ty) => {
        impl crate::cast::FromChecked<$from> for $ty {
            fn from_checked(val: $from) -> Option<Self> {
                val.try_into().ok()
            }
        }
    };
}

checked_impl!(u8);
checked_impl!(u16);
checked_impl!(u32);
checked_impl!(u64);
checked_impl!(u128);
checked_impl!(usize);

checked_impl!(i8);
checked_impl!(i16);
checked_impl!(i32);
checked_impl!(i64);
checked_impl!(i128);
checked_impl!(isize);
