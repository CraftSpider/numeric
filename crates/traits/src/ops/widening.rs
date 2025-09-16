use crate::cast::{FromTruncating, IntoTruncating};

/// Trait for types that implement 'widening' multiplication
pub trait WideningMul<Rhs = Self>: Sized {
    /// Extended multiply-addition of `(lhs * rhs) + add`. The result is returned as a tuple of the wrapping part and the
    /// overflow part. No numerical overflow is possible even if all three arguments are set to their max values.
    fn widening_mul(self, mul: Rhs, add: Rhs) -> (Self, Self);
}

macro_rules! widening_impl {
    ($ty:ty, $wide:ty) => {
        impl WideningMul for $ty {
            fn widening_mul(self, mul: Self, add: Self) -> (Self, Self) {
                let wide = <$wide>::truncate_from(self)
                    .wrapping_mul(<$wide>::truncate_from(mul))
                    .wrapping_add(<$wide>::truncate_from(add));
                (wide.truncate(), (wide >> <$ty>::BITS).truncate())
            }
        }
    };
}

widening_impl!(u8, u16);
widening_impl!(u16, u32);
widening_impl!(u32, u64);
widening_impl!(u64, u128);

#[cfg(target_pointer_width = "16")]
widening_impl!(usize, u32);
#[cfg(target_pointer_width = "32")]
widening_impl!(usize, u64);
#[cfg(target_pointer_width = "64")]
widening_impl!(usize, u128);

impl WideningMul for u128 {
    fn widening_mul(self, rhs: Self, add: Self) -> (Self, Self) {
        {
            //                       [rhs_hi]  [rhs_lo]
            //                       [lhs_hi]  [lhs_lo]
            //                     X___________________
            //                       [------tmp0------]
            //             [------tmp1------]
            //             [------tmp2------]
            //     [------tmp3------]
            //                       [-------add------]
            // +_______________________________________
            //                       [------sum0------]
            //     [------sum1------]
            // Used as the form of T with all bits set
            let lo_mask = Self::MAX.wrapping_shr(64);

            let lhs_lo = self & lo_mask;
            let rhs_lo = rhs & lo_mask;
            let lhs_hi = self.wrapping_shr(64);
            let rhs_hi = rhs.wrapping_shr(64);
            let tmp0 = lhs_lo.wrapping_mul(rhs_lo);
            let tmp1 = lhs_lo.wrapping_mul(rhs_hi);
            let tmp2 = lhs_hi.wrapping_mul(rhs_lo);
            let tmp3 = lhs_hi.wrapping_mul(rhs_hi);
            // tmp1 and tmp2 straddle the boundary. We have to handle three carries
            let (sum0, carry0) = tmp0.overflowing_add(tmp1.wrapping_shl(64));
            let (sum0, carry1) = sum0.overflowing_add(tmp2.wrapping_shl(64));
            let (sum0, carry2) = sum0.overflowing_add(add);
            let sum1 = tmp3
                .wrapping_add(tmp1.wrapping_shr(64))
                .wrapping_add(tmp2.wrapping_shr(64))
                .wrapping_add(u128::from(carry0))
                .wrapping_add(u128::from(carry1))
                .wrapping_add(u128::from(carry2));
            (sum0, sum1)
        }
    }
}
