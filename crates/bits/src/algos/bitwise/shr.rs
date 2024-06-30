use alloc::vec;
use alloc::vec::Vec;
use numeric_traits::identity::Zero;
use crate::bit_slice::BitSliceExt;
use crate::utils::IntSlice;

pub trait BitwiseShr: BitSliceExt {
    #[cfg(feature = "std")]
    /// Shift a slice right by `usize` items, implemented as a series of bitwise swaps
    fn shr(left: &Self, right: usize) -> Vec<Self::Bit> {
        let mut out = vec![Self::Bit::zero(); left.len()];
        for idx in (0..=left.bit_len()).rev() {
            let new = left.get_bit_opt(idx).unwrap_or(false);
            if let Some(idx) = idx.checked_sub(right) {
                out.set_bit_ignore(idx, new);
            }
        }
        IntSlice::shrink(out)
    }
}

impl<T> BitwiseShr for T
where
    T: ?Sized + BitSliceExt,
{}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shr() {
        let slice: &[u16] = &[0b1010101010101010, 0b1010101010101010];
        assert_eq!(BitwiseShr::shr(slice, 1), &[0b0101010101010101, 0b0101010101010101]);

        let slice: &[u8] = &[0b0, 0b11111111];
        assert_eq!(BitwiseShr::shr(slice, 8), &[0b11111111]);
    }
}
