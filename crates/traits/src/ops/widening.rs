
/// Trait for types that implement 'widening' multiplication
pub trait WideningMul<Rhs = Self>: Sized {
    /// Extended multiply-addition of `(lhs * rhs) + add`. The result is returned as a tuple of the wrapping part and the
    /// overflow part. No numerical overflow is possible even if all three arguments are set to their max values.
    fn widening_mul(self, mul: Rhs, add: Rhs) -> (Self, Self);
}

impl WideningMul for u8 {
    fn widening_mul(self, rhs: u8, add: u8) -> (u8, u8) {
        let wide = u16::from(self).wrapping_mul(u16::from(rhs)).wrapping_add(u16::from(add));
        (wide as u8, (wide >> 8) as u8)
    }
}

impl WideningMul for u16 {
    fn widening_mul(self, rhs: u16, add: u16) -> (u16, u16) {
        let wide = u32::from(self).wrapping_mul(u32::from(rhs)).wrapping_add(u32::from(add));
        (wide as u16, (wide >> 16) as u16)
    }
}

impl WideningMul for u32 {
    fn widening_mul(self, rhs: u32, add: u32) -> (u32, u32) {
        let wide = u64::from(self).wrapping_mul(u64::from(rhs)).wrapping_add(u64::from(add));
        (wide as u32, (wide >> 32) as u32)
    }
}

impl WideningMul for u64 {
    fn widening_mul(self, rhs: u64, add: u64) -> (u64, u64) {
        let wide = u128::from(self).wrapping_mul(u128::from(rhs)).wrapping_add(u128::from(add));
        (wide as u64, (wide >> 64) as u64)
    }
}

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
            let max_val = Self::MAX;
            let lo_mask = max_val.wrapping_shr(64);

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

#[cfg(target_pointer_width = "64")]
impl WideningMul for usize {
    fn widening_mul(self, rhs: usize, add: usize) -> (usize, usize) {
        let wide = (self as u128).wrapping_mul(rhs as u128).wrapping_add(add as u128);
        (wide as usize, (wide >> 32) as usize)
    }
}
