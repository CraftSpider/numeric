#[cfg(feature = "alloc")]
use crate::algos::{AddAlgo, ShlAlgo};
use crate::algos::{AssignAddAlgo, AssignMulAlgo, AssignShlAlgo, Bitwise, Element, MulAlgo};
use crate::bit_slice::{BitLike, BitSliceExt};
#[cfg(feature = "alloc")]
use crate::utils::IntSlice;
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use numeric_traits::identity::Zero;
use numeric_traits::ops::widening::WideningMul;

impl MulAlgo for Element {
    #[cfg(feature = "alloc")]
    fn long<L, R>(left: &L, right: &R) -> Vec<L::Bit>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let zero = L::Bit::zero();
        let mut out = vec![zero; left.len() + right.len()];

        left.slice().iter().enumerate().for_each(|(idx, &l)| {
            let mut carry = zero;

            for (offset, &r) in right.slice().iter().enumerate() {
                let (low, high) = L::Bit::widening_mul(l, r, carry);
                carry = high;
                add_item(&mut out, idx + offset, low);
            }

            if carry != zero {
                add_item(&mut out, idx + right.slice().len(), carry);
            }
        });

        IntSlice::shrink(out)
    }

    fn overflowing<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> (&'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let zero = L::Bit::zero();

        let mut overflow = false;
        for idx in (0..left.len()).rev() {
            // From the top to bottom, add N shifted copies of M. This can be done by taking each
            // element of the left and doing a widening mul, carrying the upper, and repeating
            let mut new_overflow = false;
            let mut carry = zero;

            let l_mut = out.slice_mut();
            let l = l_mut[idx];
            l_mut[idx] = zero;

            for (offset, &r) in right.slice().iter().enumerate() {
                let (low, high) = L::Bit::widening_mul(l, r, carry);
                carry = high;
                if add_item(out, idx + offset, low) {
                    new_overflow = true;
                }
            }

            if carry != zero && add_item(out, idx + right.slice().len(), carry) {
                new_overflow = true;
            }

            overflow |= new_overflow;
        }

        (out, overflow)
    }
}

impl AssignMulAlgo for Element {
    fn overflowing<L, R>(left: &mut L, right: &R) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let zero = L::Bit::zero();

        let mut overflow = false;
        for idx in (0..left.len()).rev() {
            // From the top to bottom, add N shifted copies of M. This can be done by taking each
            // element of the left and doing a widening mul, carrying the upper, and repeating
            let mut new_overflow = false;
            let mut carry = zero;

            let l_mut = left.slice_mut();
            let l = l_mut[idx];
            l_mut[idx] = zero;

            for (offset, &r) in right.slice().iter().enumerate() {
                let (low, high) = L::Bit::widening_mul(l, r, carry);
                carry = high;
                if add_item(l_mut, idx + offset, low) {
                    new_overflow = true;
                }
            }

            if carry != zero && add_item(l_mut, idx + right.slice().len(), carry) {
                new_overflow = true;
            }

            overflow |= new_overflow;
        }

        overflow
    }
}

impl MulAlgo for Bitwise {
    #[cfg(feature = "alloc")]
    fn long<L, R>(left: &L, right: &R) -> Vec<L::Bit>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let mut new_self = <Element as ShlAlgo>::long(left, 0);
        let mut out = vec![L::Bit::zero(); len * 2];

        for idx in 0..right.bit_len() {
            let r = right.get_bit(idx);
            if r {
                out = <Element as AddAlgo>::long(&out, &new_self);
            }
            new_self = <Element as ShlAlgo>::long(&new_self, 1);
        }

        out
    }

    fn overflowing<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> (&'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        todo!()
    }
}

fn add_item<B: BitLike>(slice: &mut [B], mut idx: usize, mut val: B) -> bool {
    let mut carry = false;

    while let Some(loc) = slice.get_mut(idx) {
        let (new, new_carry) = loc.overflowing_add(val);
        carry = new_carry;
        *loc = new;
        idx += 1;

        if !carry {
            break;
        } else {
            val = B::one();
        }
    }

    carry
}
