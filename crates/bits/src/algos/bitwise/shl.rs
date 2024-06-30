use crate::bit_slice::{BitSliceExt, BitVecExt};
use alloc::vec;
use alloc::vec::Vec;
use numeric_traits::identity::Zero;

pub trait BitwiseShl: BitSliceExt {
    #[cfg(feature = "std")]
    /// Shift a slice left by `usize` items, implemented as a series of bitwise swaps
    fn shl(left: &Self, right: usize) -> Vec<Self::Bit> {
        let bit_len = left.bit_len();
        let mut out = vec![Self::Bit::zero(); left.len()];
        left.iter_bits().enumerate().for_each(|(idx, new)| {
            if new || idx + right < bit_len {
                out.set_bit_push(idx + right, new);
            }
        });
        out
    }
}

impl<T> BitwiseShl for T where T: ?Sized + BitSliceExt {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shl() {
        let slice: &[u16] = &[0b1010101010101010, 0b1010101010101010];
        assert_eq!(
            BitwiseShl::shl(slice, 1),
            &[0b0101010101010100, 0b0101010101010101, 0b1]
        );

        let slice: &[u8] = &[0b11111111];
        assert_eq!(BitwiseShl::shl(slice, 8), &[0b0, 0b11111111]);
    }
}
