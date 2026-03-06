use super::NewtonRaphson;
use crate::algos::{
    Add, AssignAlgo, AssignBitAlgo, AssignDivRemAlgo, AssignShlAlgo, AssignShrAlgo, AssignSubAlgo,
    Bitwise, CmpAlgo, DivRemAlgo, Element, MulAlgo, ShlAlgo,
};
use crate::bit_slice::{BitLike, BitSlice};
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use core::mem;
use numeric_traits::cast::{FromSaturating, IntoSaturating};
use numeric_traits::class::BoundedBit;
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::overflowing::OverflowingAdd;
use numeric_traits::ops::widening::WideningMul;

impl DivRemAlgo for Bitwise {
    #[cfg(feature = "alloc")]
    fn long<L, R>(left: &L, right: &R) -> (Vec<L::Bit>, Vec<L::Bit>)
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let bit_len = usize::max(left.bit_len(), right.bit_len());

        let mut quotient = vec![L::Bit::zero(); len];
        let mut remainder = vec![L::Bit::zero(); len];

        for idx in (0..bit_len).rev() {
            <Element as AssignShlAlgo>::wrapping(&mut remainder, 1);
            remainder.set_bit(0, left.get_bit(idx).unwrap_or(false));
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
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        for idx in (0..bit_len).rev() {
            <Element as AssignShlAlgo>::wrapping(remainder, 1);
            remainder.set_bit(0, left.get_bit(idx).unwrap_or(false));
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
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        for idx in (0..bit_len).rev() {
            <Element as AssignShlAlgo>::wrapping(remainder, 1);
            remainder.set_bit(0, left.get_bit(idx).unwrap_or(false));
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
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        Self::div_overflowing(left, right, quotient);
        left.iter_mut()
            .zip(quotient.iter_mut())
            .for_each(|(l, r)| mem::swap(l, r));
        false
    }
}

/// Reciprocal initial estimates. We take the first 3 bits of the value to get this estimate
const RECIP_TABLE: &[u8] = &[0xFF, 0xE3, 0xCC, 0xBA, 0xAA, 0x9D, 0x92, 0x88];

fn add_item_loop<B: ?Sized + BitSlice>(slice: &mut B, mut idx: usize, mut val: B::Bit) {
    let len = slice.len();

    while let Some(loc) = slice.get_mut(idx % len) {
        let (new, new_carry) = loc.overflowing_add(val);
        *loc = new;
        idx += 1;

        if !new_carry {
            break;
        } else {
            val = B::Bit::one();
        }
    }
}

/// Given two values, l and r, get the high bits of a widening mul between them
fn hi_mul<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> &'a [L::Bit]
where
    L: ?Sized + BitSlice,
    R: ?Sized + BitSlice<Bit = L::Bit>,
{
    // We multiply into out, wrapping around our overflow value for each iteration
    let zero = L::Bit::zero();
    out.fill(zero);

    for (idx, l) in left.iter().enumerate() {
        let mut carry = out[idx];

        // We clear the current index - it will be overwritten by the new high value
        out[idx] = zero;

        for (offset, r) in right.iter().enumerate() {
            let (low, high) = L::Bit::widening_mul(l, r, carry);
            carry = high;

            // We always discard first low value, since it won't be useful in future loops
            if offset != 0 {
                add_item_loop(out, idx + offset, low);
            }
        }

        if carry != zero {
            add_item_loop(out, idx + right.len(), carry);
        }
    }

    out
}

/// l is treated as fixed 1.N - range [1, 2)
/// r is treated as fixed 0.N - range [0.5, 1)
/// output will be fixed 0.N
fn newton_step<L, R>(est: &L, goal: &R, out: &mut [L::Bit], scratch: &mut [L::Bit])
where
    L: ?Sized + BitSlice,
    R: ?Sized + BitSlice<Bit = L::Bit>,
{
    // 1.N = 1.N * 0.N, 1 <= rl < 1.5
    hi_mul(est, goal, scratch);
    // Calculate 2 - l, given that since we are using 1.N format that's equivalent to `0 - l` in
    // modulo arithmetic.
    // 1.N = 2.N - 1.N, 0.5 < 2-rl < 1
    <Element as AssignBitAlgo>::not(scratch);
    <Element as AssignAlgo<Add>>::wrapping::<[L::Bit; 0], _, _>(scratch, &[L::Bit::one()]);
    // 1.N = 1.N * 0.N, 0.5 <= l(2-rl) < 1
    hi_mul(scratch, est, out);
    // 0.N = 1.N
    <Element as AssignShlAlgo>::wrapping(out, 1);
}

fn leading_zeroes<L>(l: &L) -> usize
where
    L: ?Sized + BitSlice,
{
    l.iter()
        .rev()
        .try_fold(0, |acc, val| {
            let z = val.leading_zeros().saturate() as usize;
            if z != L::Bit::BIT_LEN {
                Err(acc + z)
            } else {
                Ok(acc + L::Bit::BIT_LEN)
            }
        })
        .unwrap_or_else(|a| a)
}

impl DivRemAlgo for NewtonRaphson {
    #[cfg(feature = "alloc")]
    fn long<L, R>(left: &L, right: &R) -> (Vec<L::Bit>, Vec<L::Bit>)
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let one = L::Bit::one();
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let len = usize::max(left.len(), right.len());

        let mut norm_r = vec![L::Bit::zero(); len];
        let mut est = vec![L::Bit::zero(); len];
        let mut new_est = vec![L::Bit::zero(); len];
        let mut scratch = vec![L::Bit::zero(); len];

        // Count leading zeroes
        let zeroes = leading_zeroes(right) + L::Bit::BIT_LEN * (len - right.len());
        // Normalize value to have leading 1
        <Element as ShlAlgo>::wrapping(right, zeroes, &mut norm_r);

        // Get estimate based on leading non-zero bits
        let t = norm_r.get(norm_r.len() - 1).unwrap();
        let estimate = RECIP_TABLE[(t >> (L::Bit::BIT_LEN - 4)).saturate() as usize - 8];
        // Put estimate in high 8 bytes of a bit...
        let estimate = L::Bit::saturate_from(estimate) << (L::Bit::BIT_LEN - 8);

        // Convert estimate to fixed 0.N by putting it in high 8 bytes
        est.set_ignore(len - 1, estimate);

        // Newton estimates to refine reciprocal
        let mut bits = 4;
        while bit_len > bits {
            newton_step(&est, &norm_r, &mut new_est, &mut scratch);
            mem::swap(&mut est, &mut new_est);
            bits *= 2;
        }

        // Calculate quotient estimate and undo normalization
        hi_mul(&est, left, &mut new_est);
        let mut quotient = new_est;
        <Element as AssignShrAlgo>::wrapping(&mut quotient, len * L::Bit::BIT_LEN - 1 - zeroes);

        if quotient.iter().any(|v| v != L::Bit::zero()) {
            <Element as AssignSubAlgo>::wrapping(&mut quotient, &[L::Bit::one()]);
        }

        let mut remainder = norm_r;
        <Element as MulAlgo>::wrapping(&quotient, right, &mut remainder);

        // Calculate left - remainder
        <Element as AssignSubAlgo>::wrapping(&mut remainder, left);
        <Element as AssignBitAlgo>::not(&mut remainder);
        <Element as AssignAlgo<Add>>::wrapping::<[L::Bit; 0], _, _>(
            &mut remainder,
            &[L::Bit::one()],
        );

        // Correct quotient to handle possible error
        if <Element as CmpAlgo>::cmp(&remainder, right).is_ge() {
            <Element as AssignAlgo<Add>>::wrapping::<[L::Bit; 0], _, _>(&mut quotient, &[one]);
            <Element as AssignSubAlgo>::wrapping(&mut remainder, right);
            if <Element as CmpAlgo>::cmp(&remainder, right).is_ge() {
                <Element as AssignAlgo<Add>>::wrapping::<[L::Bit; 0], _, _>(&mut quotient, &[one]);
                <Element as AssignSubAlgo>::wrapping(&mut remainder, right);
                if <Element as CmpAlgo>::cmp(&remainder, right).is_ge() {
                    <Element as AssignAlgo<Add>>::wrapping::<[L::Bit; 0], _, _>(
                        &mut quotient,
                        &[one],
                    );
                }
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
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        todo!("{:?}, {:?}, {:?}, {:?}", left, right, quotient, remainder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high() {
        let slice1 = &[0b1000_0000u8];
        let slice2 = &[0b0000_0010];
        let out = &mut [0];
        hi_mul(slice1, slice2, out);
        assert_eq!(out, &[0b0000_0001]);

        let slice3 = &[0b1111_1111u8];
        let slice4 = &[2];
        let out = &mut [0];
        hi_mul(slice3, slice4, out);
        assert_eq!(out, &[0b1]);

        let slice5 = &[0b1111_1111u8];
        let slice6 = &[0x10u8];
        let out = &mut [0];
        hi_mul(slice5, slice6, out);
        assert_eq!(out, &[0b0000_1111]);

        let slice7 = &[0b1000_0000u8, 0b0000_1000];
        let slice8 = &[0b0000_0000, 0b1000_0000];
        let out = &mut [0; 2];
        hi_mul(slice7, slice8, out);
        assert_eq!(out, &[0b0100_0000, 0b0000_0100]);

        let l = &[0b1111_1111u8, 0b1111_1111];
        let r = &[0b1111_1111, 0b1111_1111];
        let out = &mut [0; 2];
        hi_mul(l, r, out);
        assert_eq!(out, &[0b11111110, 0b11111111]);
    }

    #[test]
    fn test_leading_zeros() {
        assert_eq!(leading_zeroes(&[0b0100_0000u8]), 1);
        assert_eq!(leading_zeroes(&[0b1000_0000u8]), 0);
        assert_eq!(leading_zeroes(&[0x80, 0x00u8]), 8);
        assert_eq!(leading_zeroes(&[0x00, 0x01u8]), 7);
    }
}
