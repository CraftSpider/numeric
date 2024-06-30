use crate::algos::{ElementAdd, ElementShl};
use crate::bit_slice::BitSliceExt;
#[cfg(feature = "std")]
use alloc::{vec, vec::Vec};
use numeric_traits::identity::Zero;

pub trait BitwiseMul: BitSliceExt {
    #[cfg(feature = "std")]
    /// Multiply two slices, implemented as a bitwise shift-and-add
    fn mul<T>(left: &Self, right: &T) -> Vec<Self::Bit>
    where
        T: BitSliceExt<Bit = Self::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let mut new_self = ElementShl::shl(left, 0);
        let mut out = vec![Self::Bit::zero(); len * 2];

        for idx in 0..right.bit_len() {
            let r = right.get_bit(idx);
            if r {
                out = ElementAdd::add(&out, &new_self);
            }
            new_self = ElementShl::shl(&new_self, 1);
        }

        out
    }
}

impl<T> BitwiseMul for T where T: ?Sized + BitSliceExt {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let slice1: &[u8] = &[0b00000000];
        let slice2 = &[0b00000001];

        assert_eq!(BitwiseMul::mul(slice1, slice2), &[0b0]);

        let slice3: &[u8] = &[0b00000001];
        let slice4 = &[0b00000001];

        assert_eq!(BitwiseMul::mul(slice3, slice4), &[0b1]);

        let slice5: &[u8] = &[0b00000001];
        let slice6 = &[0b00000010];

        assert_eq!(BitwiseMul::mul(slice5, slice6), &[0b10]);

        let slice7: &[u8] = &[0b00000010];
        let slice8 = &[0b00000010];

        assert_eq!(BitwiseMul::mul(slice7, slice8), &[0b100]);
    }
}
