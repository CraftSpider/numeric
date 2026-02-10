use crate::algos::{AssignShlAlgo, AssignShrAlgo, Bitwise, Element, ShlAlgo, ShrAlgo};
#[cfg(feature = "alloc")]
use crate::bit_slice::BitVecExt;
use crate::bit_slice::{BitLike, BitSliceExt};
#[cfg(feature = "alloc")]
use crate::utils::IntSlice;
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use numeric_traits::class::Bounded;
use numeric_traits::identity::Zero;

impl ShlAlgo for Bitwise {
    #[cfg(feature = "alloc")]
    fn long<L>(left: &L, right: usize) -> Vec<L::Bit>
    where
        L: ?Sized + BitSliceExt,
    {
        let bit_len = left.bit_len();
        let mut out = vec![L::Bit::zero(); left.len()];
        left.iter_bits().enumerate().for_each(|(idx, new)| {
            if new || idx + right < bit_len {
                out.set_bit_push(idx + right, new);
            }
        });
        out
    }

    fn overflowing<'a, L>(left: &L, right: usize, out: &'a mut [L::Bit]) -> (&'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
    {
        let bit_len = left.bit_len();
        left.iter_bits().enumerate().for_each(|(idx, new)| {
            if new || idx + right < bit_len {
                out.set_bit_ignore(idx + right, new);
            }
        });
        (out, right > bit_len)
    }
}

impl ShrAlgo for Bitwise {
    #[cfg(feature = "alloc")]
    fn long<L>(left: &L, right: usize) -> Vec<L::Bit>
    where
        L: ?Sized + BitSliceExt,
    {
        let mut out = vec![L::Bit::zero(); left.len()];
        for idx in (0..=left.bit_len()).rev() {
            let new = left.get_bit_opt(idx).unwrap_or(false);
            if let Some(idx) = idx.checked_sub(right) {
                out.set_bit_ignore(idx, new);
            }
        }
        IntSlice::shrink(out)
    }

    fn overflowing<'a, L>(left: &L, right: usize, out: &'a mut [L::Bit]) -> (&'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
    {
        let bit_len = left.bit_len();
        for idx in (0..=bit_len).rev() {
            let new = left.get_bit_opt(idx).unwrap_or(false);
            if let Some(idx) = idx.checked_sub(right) {
                out.set_bit_ignore(idx, new);
            }
        }
        (out, right > bit_len)
    }
}

impl ShlAlgo for Element {
    #[cfg(feature = "alloc")]
    fn long<L>(left: &L, right: usize) -> Vec<L::Bit>
    where
        L: ?Sized + BitSliceExt,
    {
        let arr_shift = (right / L::Bit::BIT_LEN) + 1;
        let elem_shift = right % L::Bit::BIT_LEN;
        let inverse_elem_shift = (L::Bit::BIT_LEN - elem_shift) % L::Bit::BIT_LEN;
        let elem_mask: L::Bit = !(L::Bit::max_value() << elem_shift);
        let zero = L::Bit::zero();
        let mut out = vec![L::Bit::zero(); left.len() + arr_shift];

        left.iter().enumerate().rev().for_each(|(idx, val)| {
            let high = val >> inverse_elem_shift;
            let low = val << elem_shift;

            let high = (out.get(idx + arr_shift).unwrap_or(zero) & !elem_mask) | (high & elem_mask);

            out.set_ignore(idx + arr_shift, high);

            // We don't need to consider the existing value of the output. `low` always goes
            // into it before `high` since we're iterating backwards.
            let low = low & !elem_mask;

            out.set_ignore(idx + arr_shift - 1, low);
        });

        IntSlice::shrink(out)
    }

    fn overflowing<'a, L>(left: &L, right: usize, out: &'a mut [L::Bit]) -> (&'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
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

            let high = (out.get(idx + arr_shift).copied().unwrap_or(zero) & !elem_mask)
                | (high & elem_mask);

            out.set_ignore(idx + arr_shift, high);

            let low = low & !elem_mask;

            out.set_ignore(idx + arr_shift - 1, low);
        });
        out.iter_mut().take(arr_shift - 1).for_each(|o| *o = zero);

        (out, right > left.bit_len())
    }
}

impl AssignShlAlgo for Element {
    fn overflowing<'a, L>(left: &mut L, right: usize) -> bool
    where
        L: ?Sized + BitSliceExt,
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

impl ShrAlgo for Element {
    #[cfg(feature = "alloc")]
    fn long<L>(left: &L, right: usize) -> Vec<L::Bit>
    where
        L: ?Sized + BitSliceExt,
    {
        let arr_shift = (right / L::Bit::BIT_LEN) + 1;
        let elem_shift = right % L::Bit::BIT_LEN;
        let inverse_elem_shift = (L::Bit::BIT_LEN - elem_shift) % L::Bit::BIT_LEN;
        let elem_mask: L::Bit = !(L::Bit::max_value() >> elem_shift);
        let zero = L::Bit::zero();
        let mut out = vec![L::Bit::zero(); left.len()];

        left.iter().enumerate().for_each(|(idx, val)| {
            let high = val >> elem_shift;
            let low = val << inverse_elem_shift;

            if let Some(idx) = usize::checked_sub(idx, arr_shift) {
                let low = (out.get(idx).unwrap_or(zero) & !elem_mask) | (low & elem_mask);

                out.set_ignore(idx, low);
            }

            if let Some(idx) = usize::checked_sub(idx + 1, arr_shift) {
                let high = high & !elem_mask;

                out.set_ignore(idx, high);
            }
        });

        IntSlice::shrink(out)
    }

    fn overflowing<'a, L>(left: &L, right: usize, out: &'a mut [L::Bit]) -> (&'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
    {
        extern crate std;
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
                let low = (out.get(idx).copied().unwrap_or(zero) & !elem_mask) | (low & elem_mask);

                out.set_ignore(idx, low);
            }

            if let Some(idx) = usize::checked_sub(idx + 1, arr_shift) {
                let high = high & !elem_mask;

                out.set_ignore(idx, high);
            }
        });
        let empty = left.len() - arr_shift + 1;
        out.iter_mut().skip(empty).for_each(|o| *o = zero);

        (out, right > left.bit_len())
    }
}

impl AssignShrAlgo for Element {
    fn overflowing<'a, L>(left: &mut L, right: usize) -> bool
    where
        L: ?Sized + BitSliceExt,
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
