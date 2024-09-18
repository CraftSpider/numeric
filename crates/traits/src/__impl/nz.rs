use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

macro_rules! impl_nz {
    ($ty:ty) => {
        impl crate::identity::One for $ty {
            #[inline]
            fn one() -> Self {
                // SAFETY: 1 is not 0
                unsafe { <$ty>::new_unchecked(1) }
            }

            fn is_one(&self) -> bool {
                self.get() != 0
            }
        }

        impl crate::class::Bounded for $ty {
            #[inline]
            fn max_value() -> Self {
                <$ty>::MAX
            }

            #[inline]
            fn min_value() -> Self {
                <$ty>::MIN
            }
        }
    };
}

macro_rules! impl_unz {
    ($ty:ty) => {
        impl crate::class::Unsigned for $ty {}
    };
}

macro_rules! impl_snz {
    ($ty:ty) => {
        impl crate::class::Signed for $ty {
            fn abs(self) -> Self {
                <$ty>::abs(self)
            }

            fn is_positive(&self) -> bool {
                self.get().is_positive()
            }

            fn is_negative(&self) -> bool {
                // TODO: Change once nonzero_negation_ops stabilizes
                self.get().is_negative()
            }
        }

        impl crate::class::BoundedSigned for $ty {
            fn min_positive() -> Self {
                <Self as crate::identity::One>::one()
            }

            fn max_negative() -> Self {
                -<Self as crate::identity::One>::one()
            }
        }
    };
}

impl_nz!(NonZeroU8);
impl_nz!(NonZeroU16);
impl_nz!(NonZeroU32);
impl_nz!(NonZeroU64);
impl_nz!(NonZeroU128);
impl_nz!(NonZeroUsize);

impl_unz!(NonZeroU8);
impl_unz!(NonZeroU16);
impl_unz!(NonZeroU32);
impl_unz!(NonZeroU64);
impl_unz!(NonZeroU128);
impl_unz!(NonZeroUsize);

impl_nz!(NonZeroI8);
impl_nz!(NonZeroI16);
impl_nz!(NonZeroI32);
impl_nz!(NonZeroI64);
impl_nz!(NonZeroI128);
impl_nz!(NonZeroIsize);

impl_snz!(NonZeroI8);
impl_snz!(NonZeroI16);
impl_snz!(NonZeroI32);
impl_snz!(NonZeroI64);
impl_snz!(NonZeroI128);
impl_snz!(NonZeroIsize);
