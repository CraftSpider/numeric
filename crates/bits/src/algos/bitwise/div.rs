use crate::algos::{ElementCmp, ElementShl, ElementSub};
use crate::bit_slice::BitSliceExt;
#[cfg(feature = "std")]
use alloc::{vec, vec::Vec};
use numeric_traits::identity::Zero;

pub trait BitwiseDiv: BitSliceExt {
    #[cfg(feature = "std")]
    /// Divide two slices, implemented as bitwise long division
    fn div_long<T>(num: &Self, div: &T) -> (Vec<Self::Bit>, Vec<Self::Bit>)
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let len = usize::max(num.len(), div.len());
        let bit_len = usize::max(num.bit_len(), div.bit_len());

        let mut quotient = vec![Self::Bit::zero(); len];
        let mut remainder = vec![Self::Bit::zero(); len];

        for idx in (0..bit_len).rev() {
            remainder = ElementShl::shl(&remainder, 1);
            remainder.set_bit(0, num.get_bit(idx));
            if ElementCmp::cmp(&remainder, div).is_ge() {
                // Ignore the bool - subtract will never overflow
                remainder = ElementSub::sub(&remainder, div).0;
                quotient.set_bit(idx, true);
            }
        }

        (quotient, remainder)
    }
}

impl<T> BitwiseDiv for T where T: ?Sized + BitSliceExt {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div() {
        let slice1: &[u8] = &[0b10];
        let slice2: &[u8] = &[0b01];

        assert_eq!(BitwiseDiv::div_long(slice1, slice2).0, &[0b10]);

        let slice3: &[u8] = &[0b10];
        let slice4: &[u8] = &[0b10];

        assert_eq!(BitwiseDiv::div_long(slice3, slice4).0, &[0b01]);

        let slice5: &[u8] = &[0b00000000, 0b1];
        let slice6: &[u8] = &[0b00000010];

        assert_eq!(BitwiseDiv::div_long(slice5, slice6).0, &[0b10000000, 0b0]);

        let slice7: &[u8] = &[0b0, 0b0, 0b0, 0b1];
        let slice8: &[u8] = &[0b10];

        assert_eq!(
            BitwiseDiv::div_long(slice7, slice8).0,
            &[0b0, 0b0, 0b10000000, 0b0]
        );
    }

    #[test]
    fn test_rem() {
        for i in 0..4 {
            let slice1: &[u8] = &[i];
            let slice2 = &[0b10];

            assert_eq!(BitwiseDiv::div_long(slice1, slice2).1, &[i % 2]);
        }

        for i in 0..6 {
            let slice3: &[u8] = &[i];
            let slice4 = &[0b11];

            assert_eq!(BitwiseDiv::div_long(slice3, slice4).1, &[i % 3]);
        }

        let slice5: &[u8] = &[0b00000001, 0b111];
        let slice6 = &[0b00000010];

        assert_eq!(BitwiseDiv::div_long(slice5, slice6).1, &[0b01]);
    }
}
