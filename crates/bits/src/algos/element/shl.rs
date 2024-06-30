use numeric_traits::class::Bounded;
use numeric_traits::identity::Zero;
#[cfg(feature = "std")]
use alloc::{vec, vec::Vec};
use numeric_traits::cast::FromTruncating;

use crate::bit_slice::{BitLike, BitSliceExt};
use crate::utils::IntSlice;

pub trait ElementShl: BitSliceExt {
    #[cfg(feature = "std")]
    /// Shift a slice left by `usize` items, implemented as a series of shifts and masks
    fn shl(left: &Self, right: usize) -> Vec<Self::Bit> {
        let arr_shift = (right / Self::Bit::BIT_LEN) + 1;
        let elem_shift = right % Self::Bit::BIT_LEN;
        let inverse_elem_shift = (Self::Bit::BIT_LEN - elem_shift) % Self::Bit::BIT_LEN;
        let elem_mask: Self::Bit = !(Self::Bit::max_value() << elem_shift);
        let zero = Self::Bit::zero();
        let mut out = vec![Self::Bit::zero(); left.len() + arr_shift];

        left.slice()
            .iter()
            .enumerate()
            .rev()
            .for_each(|(idx, &val)| {
                let high = val >> inverse_elem_shift;
                let low = val << elem_shift;

                let high = (out.get_opt(idx + arr_shift).unwrap_or(zero) & !elem_mask) | (high & elem_mask);

                out.set_ignore(
                    idx + arr_shift,
                    high,
                );

                let low = (out.get_opt(idx + arr_shift - 1).unwrap_or(zero) & elem_mask) | (low & !elem_mask);

                out.set_ignore(
                    idx + arr_shift - 1,
                    low,
                );
            });

        IntSlice::shrink(out)
    }

    fn inner_shl_wrap_and_mask(left: &mut Self, right: usize) -> &mut Self {
        let arr_shift = (right / Self::Bit::BIT_LEN) + 1;
        let elem_shift = right % Self::Bit::BIT_LEN;
        let inverse_elem_shift = (Self::Bit::BIT_LEN - elem_shift) % Self::Bit::BIT_LEN;
        let elem_mask: Self::Bit = !(Self::Bit::max_value() << elem_shift);
        let zero = Self::Bit::zero();

        (0..left.slice().len())
            .rev()
            .for_each(|idx| {
                // SAFETY: Iterating up to len - will never overrun
                let val = unsafe { left.get_opt(idx).unwrap_unchecked() };
                let high = val >> inverse_elem_shift;
                let low = val << elem_shift;

                let high = (left.get_opt(idx + arr_shift).unwrap_or(zero) & !elem_mask) | (high & elem_mask);

                left.set_ignore(
                    idx + arr_shift,
                    high,
                );

                let low = (left.get_opt(idx + arr_shift - 1).unwrap_or(zero) & elem_mask) | (low & !elem_mask);

                left.set_ignore(
                    idx + arr_shift - 1,
                    low,
                );
            });

        left
    }

    /// Shift a slice left by `usize` items, implemented as a series of shifts and masks, returning
    /// None if the shift value is greater than the number of bits in the left-hand side.
    fn shl_wrap_and_mask_checked(left: &mut Self, right: usize) -> Option<&mut Self> {
        if right > left.bit_len() {
            return None;
        }

        Some(Self::inner_shl_wrap_and_mask(left, right))
    }

    /// Shift a slice left by `usize` items, implemented as a series of shifts and masks, masking
    /// the shift value if it is greater than the number of bits in the left-hand side.
    fn shl_wrap_and_mask_wrapping(left: &mut Self, right: usize) -> &mut Self {
        let bit_len = left.bit_len();
        let num_zeroes = usize::truncate(bit_len.leading_zeros()) + 1;
        Self::inner_shl_wrap_and_mask(left, right & usize::MAX >> num_zeroes)
    }
}

impl<T> ElementShl for T
where
    T: ?Sized + BitSliceExt,
{}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(
            ElementShl::shl(&[0b00000000u8], 1),
            &[0],
        );
        assert_eq!(
            ElementShl::shl(&[0b01u8], 1),
            &[0b10],
        );
        assert_eq!(
            ElementShl::shl(&[0b0101u8], 1),
            &[0b1010],
        );
    }

    #[test]
    fn test_wrap() {
        let slice = &[0b1010101010101010u16, 0b1010101010101010];
        let res = ElementShl::shl(slice, 1);
        assert_eq!(res, &[0b0101010101010100, 0b0101010101010101, 0b1]);
    }
}
