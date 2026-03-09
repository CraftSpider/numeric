use crate::algos::{AssignShlAlgo, AssignShrAlgo, Element};
use crate::bit_slice::{BitLike, BitSlice};
use numeric_traits::class::Bounded;
use numeric_traits::identity::Zero;

impl AssignShlAlgo for Element {
    fn overflowing<L>(left: &mut L, right: usize) -> bool
    where
        L: ?Sized + BitSlice,
    {
        let arr_shift = (right / L::Bit::BIT_LEN) + 1;
        let elem_shift = right % L::Bit::BIT_LEN;
        let inverse_elem_shift = (L::Bit::BIT_LEN - elem_shift) % L::Bit::BIT_LEN;
        let elem_mask: L::Bit = !(L::Bit::max_value() << elem_shift);
        let zero = L::Bit::zero();

        (0..left.len()).rev().for_each(|idx| {
            // SAFETY: Iterating up to len - will never overrun
            let val = unsafe { left.get(idx).unwrap_unchecked() };
            let high = val >> inverse_elem_shift;
            let low = val << elem_shift;

            let high =
                (left.get(idx + arr_shift).unwrap_or(zero) & !elem_mask) | (high & elem_mask);

            left.set_ignore(idx + arr_shift, high);

            let low = low & !elem_mask;

            left.set_ignore(idx + arr_shift - 1, low);
        });
        left.iter_mut().take(arr_shift - 1).for_each(|l| *l = zero);

        right > left.bit_len()
    }
}

impl AssignShrAlgo for Element {
    fn overflowing<L>(left: &mut L, right: usize) -> bool
    where
        L: ?Sized + BitSlice,
    {
        let arr_shift = (right / L::Bit::BIT_LEN) + 1;
        let elem_shift = right % L::Bit::BIT_LEN;
        let inverse_elem_shift = (L::Bit::BIT_LEN - elem_shift) % L::Bit::BIT_LEN;
        let elem_mask: L::Bit = !(L::Bit::max_value() >> elem_shift);
        let zero = L::Bit::zero();

        // dbg!(arr_shift, elem_shift);

        (0..left.len()).for_each(|idx| {
            // SAFETY: Iterating up to len - will never overrun
            let val = unsafe { left.get(idx).unwrap_unchecked() };
            let high = val >> elem_shift;
            let low = val << inverse_elem_shift;

            if let Some(idx) = usize::checked_sub(idx, arr_shift) {
                let low = (left.get(idx).unwrap_or(zero) & !elem_mask) | (low & elem_mask);

                left.set_ignore(idx, low);
            }

            if let Some(idx) = usize::checked_sub(idx + 1, arr_shift) {
                let high = high & !elem_mask;

                left.set_ignore(idx, high);
            }
        });
        let empty = left.len() - arr_shift + 1;
        left.iter_mut().skip(empty).for_each(|l| *l = zero);

        right > left.bit_len()
    }
}
