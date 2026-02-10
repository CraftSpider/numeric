use crate::algos::{
    AssignDivRemAlgo, AssignShlAlgo, AssignSubAlgo, Bitwise, CmpAlgo, DivRemAlgo, Element,
};
use crate::bit_slice::BitSliceExt;
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use core::mem;
#[cfg(feature = "alloc")]
use numeric_traits::identity::Zero;

/*
impl DivRemAlgo for Element {
    #[cfg(feature = "alloc")]
    fn long<L, R>(left: &L, right: &R) -> (Vec<L::Bit>, Vec<L::Bit>)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());

        let mut quotient = vec![L::Bit::zero(); len];
        let mut remainder = vec![L::Bit::zero(); len];
        let mut one = vec![L::Bit::zero(); len];
        one[len - 1] = L::Bit::one();

        for idx in (0..len).rev() {
            // Shift left by 1 element
            <Element as AssignShlAlgo>::wrapping(&mut remainder, L::Bit::BIT_LEN);
            // Set new element to left[idx]
            remainder.set(0, left.get(idx));

            // Mathy stuff:
            //   The remainder will *always* be no more than one digit greater than the divisor
            //   Which means the divisor will go into the remainder at most Self::Bit::MAX times
            //
            //   (99 / 100, remainder of 99, is the worst case)

            // TODO: This is the slow bit. Maybe do some mul/sub stuff instead?
            //       Can this not be a loop, maybe nested division or something?
            //       Remainder is at most Bit::MAX * right. Can we use that?
            // While remainder is greater than divisor
            while <Element as CmpAlgo>::ge(&remainder, right) {
                // Subtract remainder by divisor
                <Element as AssignSubAlgo>::wrapping(&mut remainder, right);
                // Add 1 to quotient at idx
                <Element as AssignAddAlgo>::wrapping(&mut quotient, &one);
            }

            <Element as AssignShrAlgo>::wrapping(&mut one, L::Bit::BIT_LEN);
        }

        (IntSlice::shrink(quotient), IntSlice::shrink(remainder))
    }

    fn overflowing<'a, L, R>(
        left: &L,
        right: &R,
        quotient: &'a mut [L::Bit],
        remainder: &'a mut [L::Bit],
    ) -> (&'a [L::Bit], &'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        todo!()
    }
}
 */

impl DivRemAlgo for Bitwise {
    #[cfg(feature = "alloc")]
    fn long<L, R>(left: &L, right: &R) -> (Vec<L::Bit>, Vec<L::Bit>)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let bit_len = usize::max(left.bit_len(), right.bit_len());

        let mut quotient = vec![L::Bit::zero(); len];
        let mut remainder = vec![L::Bit::zero(); len];

        for idx in (0..bit_len).rev() {
            <Element as AssignShlAlgo>::wrapping(&mut remainder, 1);
            remainder.set_bit(0, left.get_bit(idx));
            if <Element as CmpAlgo>::ge(&remainder, right) {
                // Subtract will never overflow
                <Element as AssignSubAlgo>::wrapping(&mut remainder, right);
                quotient.set_bit(idx, true);
            }
        }

        (quotient, remainder)
    }

    fn overflowing<'a, L, R>(
        left: &L,
        right: &R,
        quotient: &'a mut [L::Bit],
        remainder: &'a mut [L::Bit],
    ) -> (&'a [L::Bit], &'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        for idx in (0..bit_len).rev() {
            <Element as AssignShlAlgo>::wrapping(remainder, 1);
            remainder.set_bit(0, left.get_bit(idx));
            if <Element as CmpAlgo>::ge(remainder, right) {
                // Subtract will never overflow
                <Element as AssignSubAlgo>::wrapping(remainder, right);
                quotient.set_bit(idx, true);
            } else {
                quotient.set_bit(idx, false);
            }
        }

        (quotient, remainder, false)
    }
}

impl AssignDivRemAlgo for Bitwise {
    fn div_overflowing<L, R>(left: &mut L, right: &R, remainder: &mut [L::Bit]) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        for idx in (0..bit_len).rev() {
            <Element as AssignShlAlgo>::wrapping(remainder, 1);
            remainder.set_bit(0, left.get_bit(idx));
            if <Element as CmpAlgo>::ge(remainder, right) {
                // Subtract will never overflow
                <Element as AssignSubAlgo>::wrapping(remainder, right);
                left.set_bit(idx, true);
            } else {
                left.set_bit(idx, false);
            }
        }

        false
    }

    fn rem_overflowing<L, R>(left: &mut L, right: &R, quotient: &mut [L::Bit]) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        Self::div_overflowing(left, right, quotient);
        left.iter_mut()
            .zip(quotient.iter_mut())
            .for_each(|(l, r)| mem::swap(l, r));
        false
    }
}
