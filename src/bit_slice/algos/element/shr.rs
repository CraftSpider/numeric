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
    pub fn shr_wrap_and_mask(left: BitSlice<S, I>, right: usize) -> OwnedSlice<I> {
        let bit_size = mem::size_of::<I>() * 8;
        let arr_shift = (right / bit_size) + 1;
        let elem_shift = right % bit_size;
        let inverse_elem_shift = (bit_size - elem_shift) % bit_size;
        let elem_mask: I = !(I::max_value() >> elem_shift);
        let zero = I::zero();
        let mut out = BitSlice::new(vec![I::zero(); left.len()]);

        left.slice()
            .iter()
            .enumerate()
            .for_each(|(idx, &val)| {
                let high = val << inverse_elem_shift;
                let low = val >> elem_shift;

                if idx != 0 {
                    let high = (out.get_opt(idx - arr_shift).unwrap_or(zero) & !elem_mask) | (high & elem_mask);

                    out.set_ignore(
                        idx - arr_shift,
                        high,
                    );
                }

                let low = (out.get_opt(idx + 1 - arr_shift).unwrap_or(zero) & elem_mask) | (low & !elem_mask);

                out.set_ignore(
                    idx + 1 - arr_shift,
                    low,
                );
            });

        BitSlice::new(shrink_vec(out.into_inner()))
    }
}

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]> + AsMut<[I]>,
    I: PrimInt + core::fmt::Binary,
{
    fn inner_shr_wrap_and_mask(mut left: BitSlice<S, I>, right: usize) -> BitSlice<S, I> {
        let bit_size = mem::size_of::<I>() * 8;
        let arr_shift = (right / bit_size) + 1;
        let elem_shift = right % bit_size;
        let inverse_elem_shift = (bit_size - elem_shift) % bit_size;
        let elem_mask: I = !(I::max_value() >> elem_shift);
        let zero = I::zero();

        dbg!(arr_shift, elem_shift);

        (0..left.slice().len())
            .for_each(|idx| {
                // SAFETY: Iterating up to len - will never overrun
                let val = unsafe { left.get_opt(idx).unwrap_unchecked() };
                let high = val << inverse_elem_shift;
                let low = val >> elem_shift;

                println!("Idx: {:?}", idx);
                println!("Val: {:b}", val);
                println!("High: {:b}, Low: {:b}", high, low);

                if let Some(idx) = usize::checked_sub(idx, arr_shift) {
                    let high = (left.get_opt(idx).unwrap_or(zero) & !elem_mask) | (high & elem_mask);

                    println!("New High: {:b}", high);

                    left.set_ignore(
                        idx,
                        high,
                    );
                }

                let low = (left.get_opt(idx + 1 - arr_shift).unwrap_or(zero) & elem_mask) | (low & !elem_mask);

                println!("New Low: {:b}", low);

                left.set_ignore(
                    idx + 1 - arr_shift,
                    low,
                );
            });

        // We need to zero-out items we shifted out of
        if let Some(item) = left.slice_mut().last_mut() {
            *item = zero & elem_mask | *item & !elem_mask;
        }

        left
    }

    /// Shift a slice left by `usize` items, implemented as a series of shifts and masks, returning
    /// None if the shift value is greater than the number of bits in the left-hand side.
    pub fn shr_wrap_and_mask_checked(left: BitSlice<S, I>, right: usize) -> Option<BitSlice<S, I>> {
        if right > left.bit_len() {
            return None;
        }

        Some(Self::inner_shr_wrap_and_mask(left, right))
    }

    /// Shift a slice left by `usize` items, implemented as a series of shifts and masks, masking
    /// the shift value if it is greater than the number of bits in the left-hand side.
    pub fn shr_wrap_and_mask_wrapping(left: BitSlice<S, I>, right: usize) -> BitSlice<S, I> {
        let bit_len = left.bit_len();
        let num_zeroes = (bit_len.leading_zeros() as usize) + 1;
        Self::inner_shr_wrap_and_mask(left, right & usize::MAX >> num_zeroes)
    }
}
