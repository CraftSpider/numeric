use std::mem;
use num_traits::PrimInt;

use crate::bit_slice::BitSlice;
use crate::utils::shrink_vec;
use crate::bit_slice::algos::OwnedSlice;

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]>,
    I: PrimInt,
{
    /// Shift a slice left by `usize` items, implemented as a series of shifts and masks
    pub fn shl_wrap_and_mask(left: BitSlice<S, I>, right: usize) -> OwnedSlice<I> {
        let bit_size = mem::size_of::<I>() * 8;
        let arr_shift = (right / bit_size) + 1;
        let elem_shift = right % bit_size;
        let inverse_elem_shift = (bit_size - elem_shift) % bit_size;
        let elem_mask: I = !(I::max_value() << elem_shift);
        let zero = I::zero();
        let mut out = BitSlice::new(vec![I::zero(); left.len() + arr_shift]);

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

        BitSlice::new(shrink_vec(out.into_inner()))
    }
}

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]> + AsMut<[I]>,
    I: PrimInt,
{
    fn inner_shl_wrap_and_mask(mut left: BitSlice<S, I>, right: usize) -> BitSlice<S, I> {
        let bit_size = mem::size_of::<I>() * 8;
        let arr_shift = (right / bit_size) + 1;
        let elem_shift = right % bit_size;
        let inverse_elem_shift = (bit_size - elem_shift) % bit_size;
        let elem_mask: I = !(I::max_value() << elem_shift);
        let zero = I::zero();

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
    pub fn shl_wrap_and_mask_checked(left: BitSlice<S, I>, right: usize) -> Option<BitSlice<S, I>> {
        if right > left.bit_len() {
            return None;
        }

        Some(Self::inner_shl_wrap_and_mask(left, right))
    }

    /// Shift a slice left by `usize` items, implemented as a series of shifts and masks, masking
    /// the shift value if it is greater than the number of bits in the left-hand side.
    pub fn shl_wrap_and_mask_wrapping(left: BitSlice<S, I>, right: usize) -> BitSlice<S, I> {
        let bit_len = left.bit_len();
        let num_zeroes = (bit_len.leading_zeros() as usize) + 1;
        Self::inner_shl_wrap_and_mask(left, right & usize::MAX >> num_zeroes)
    }
}
