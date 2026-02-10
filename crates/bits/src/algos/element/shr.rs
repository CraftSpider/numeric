use crate::bit_slice::{BitLike, BitSliceExt};
use crate::utils::IntSlice;
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use numeric_traits::class::Bounded;
use numeric_traits::identity::Zero;

pub trait ElementShr: BitSliceExt {
    #[cfg(feature = "alloc")]
    /// Shift a slice left by `usize` items, implemented as a series of shifts and masks
    fn shr(left: &Self, right: usize) -> Vec<Self::Bit> {
        let arr_shift = (right / Self::Bit::BIT_LEN) + 1;
        let elem_shift = right % Self::Bit::BIT_LEN;
        let inverse_elem_shift = (Self::Bit::BIT_LEN - elem_shift) % Self::Bit::BIT_LEN;
        let elem_mask: Self::Bit = !(Self::Bit::max_value() >> elem_shift);
        let zero = Self::Bit::zero();
        let mut out = vec![Self::Bit::zero(); left.len()];

        left.slice().iter().enumerate().for_each(|(idx, &val)| {
            let high = val << inverse_elem_shift;
            let low = val >> elem_shift;

            if idx != 0 {
                let high = (out.get_opt(idx - arr_shift).unwrap_or(zero) & !elem_mask)
                    | (high & elem_mask);

                out.set_ignore(idx - arr_shift, high);
            }

            let low =
                (out.get_opt(idx + 1 - arr_shift).unwrap_or(zero) & elem_mask) | (low & !elem_mask);

            out.set_ignore(idx + 1 - arr_shift, low);
        });

        IntSlice::shrink(out)
    }

    fn inner_shr_wrap_and_mask(left: &mut Self, right: usize) -> &mut Self {
        let arr_shift = (right / Self::Bit::BIT_LEN) + 1;
        let elem_shift = right % Self::Bit::BIT_LEN;
        let inverse_elem_shift = (Self::Bit::BIT_LEN - elem_shift) % Self::Bit::BIT_LEN;
        let elem_mask: Self::Bit = !(Self::Bit::max_value() >> elem_shift);
        let zero = Self::Bit::zero();

        // dbg!(arr_shift, elem_shift);

        (0..left.slice().len()).for_each(|idx| {
            // SAFETY: Iterating up to len - will never overrun
            let val = unsafe { left.get_opt(idx).unwrap_unchecked() };
            let high = val << inverse_elem_shift;
            let low = val >> elem_shift;

            if let Some(idx) = usize::checked_sub(idx, arr_shift) {
                let high = (left.get_opt(idx).unwrap_or(zero) & !elem_mask) | (high & elem_mask);

                left.set_ignore(idx, high);
            }

            if let Some(idx) = usize::checked_sub(idx + 1, arr_shift) {
                let low = (left.get_opt(idx).unwrap_or(zero) & elem_mask) | (low & !elem_mask);

                left.set_ignore(idx, low);
            }
        });
        let empty = left.len() - arr_shift + 1;
        left.slice_mut()[empty..].fill(zero);

        left
    }

    /// Shift a slice left by `usize` items, implemented as a series of shifts and masks, returning
    /// None if the shift value is greater than the number of bits in the left-hand side.
    fn shr_checked(left: &mut Self, right: usize) -> Option<&mut Self> {
        if right > left.bit_len() {
            return None;
        }

        Some(Self::inner_shr_wrap_and_mask(left, right))
    }

    /// Shift a slice left by `usize` items, implemented as a series of shifts and masks, masking
    /// the shift value if it is greater than the number of bits in the left-hand side.
    fn shr_wrapping(left: &mut Self, right: usize) -> &mut Self {
        let bit_len = left.bit_len();
        let num_zeroes = (bit_len.leading_zeros() as usize) + 1;
        Self::inner_shr_wrap_and_mask(left, right & usize::MAX >> num_zeroes)
    }
}

impl<T> ElementShr for T where T: ?Sized + BitSliceExt {}
