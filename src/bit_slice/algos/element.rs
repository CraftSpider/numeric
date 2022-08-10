use std::mem;
use num_traits::{Bounded, PrimInt, Zero};

use crate::bit_slice::{BitSlice, IndexOpt, Len};
use crate::bit_slice::private::IndexOptMut;
use crate::utils::shrink_vec;
use super::OwnedSlice;

impl<S> BitSlice<S>
where
    S: IndexOpt<usize> + Len,
    S::Output: PrimInt,
{
    /// Shift a slice left by `usize` items, implemented as a series of shifts and masks
    pub fn shl_wrap_and_mask(left: BitSlice<S>, right: usize) -> OwnedSlice<S::Output> {
        let bit_size = mem::size_of::<S::Output>() * 8;
        let arr_shift = (right / bit_size) + 1;
        let elem_shift = right % bit_size;
        let inverse_elem_shift = bit_size - elem_shift;
        let elem_mask: S::Output = !(<S::Output>::max_value() << elem_shift);
        let mut out = BitSlice::new(vec![<S::Output>::zero(); left.len() + arr_shift]);

        for (idx, val) in left.iter().rev() {
            let high = val >> inverse_elem_shift;
            let low = val << elem_shift;

            let high = (out.get_opt(idx + arr_shift).unwrap_or(<S::Output>::zero()) & !elem_mask) | (high & elem_mask);

            if high != <S::Output>::zero() {
                out.set_ignore(
                    idx + arr_shift,
                    high,
                );
            }

            let low = (out.get_opt(idx + arr_shift - 1).unwrap_or(<S::Output>::zero()) & elem_mask) | (low & !elem_mask);

            if low != <S::Output>::zero() {
                out.set_ignore(
                    idx + arr_shift - 1,
                    low,
                );
            }
        }

        BitSlice(shrink_vec(out.into_inner()))
    }

    // Operation Types: carrying, checked, overflowing (wrapping + carrying), saturating, wrapping
    // Shift Types: checked, overflowing (wrapping + checked), wrapping
}

impl<S> BitSlice<S>
where
    S: IndexOptMut<usize> + Len,
    S::Output: PrimInt,
{
    pub fn shl_wrap_and_mask_checked(left: BitSlice<S>, right: usize) -> BitSlice<S> {
        todo!()
    }

    pub fn shl_wrap_and_mask_wrapping(left: BitSlice<S>, right: usize) -> BitSlice<S> {
        todo!()
    }
}
