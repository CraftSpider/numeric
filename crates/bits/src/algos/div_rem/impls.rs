use super::NewtonRaphson;
use crate::algos::{
    AssignAddAlgo, AssignBitAlgo, AssignDivRemAlgo, AssignMulAlgo, AssignShlAlgo, AssignShrAlgo,
    AssignSubAlgo, Bitwise, CmpAlgo, DivRemAlgo, Element, MulAlgo, ShlAlgo,
};
use crate::bit_slice::{BitLike, BitSliceExt, BitVecExt};
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use core::mem;
use numeric_traits::cast::{FromSaturating, IntoSaturating};
use numeric_traits::class::BoundedBit;
use numeric_traits::identity::One;
#[cfg(feature = "alloc")]
use numeric_traits::identity::Zero;

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
            remainder.set_bit(0, left.get_bit(idx).unwrap());
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
            remainder.set_bit(0, left.get_bit(idx).unwrap());
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
            remainder.set_bit(0, left.get_bit(idx).unwrap());
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

/// Reciprocal initial estimates. We take the first 3 bits of the value to get this estimate
const RECIP_TABLE: &[u8] = &[0xFF, 0xE3, 0xCC, 0xBA, 0xAA, 0x9D, 0x92, 0x88];

/// Given two values, l and r, get the high bits of a widening mul between them
fn hi_mul<L, R>(l: &mut L, r: &R)
where
    L: ?Sized + BitVecExt,
    R: ?Sized + BitSliceExt<Bit = L::Bit>,
{
    let len = l.len();
    l.extend(len * 2, L::Bit::zero());
    <Element as AssignMulAlgo>::wrapping(l, r);
    <Element as AssignShrAlgo>::wrapping(l, len * L::Bit::BIT_LEN);
    l.truncate(len);
}

/// l is treated as fixed 1.N - range [1, 2)
/// r is treated as fixed 0.N - range [0.5, 1)
/// output will be fixed 0.N
fn newton_step<L, R>(est: &L, goal: &R, out: &mut Vec<L::Bit>)
where
    L: ?Sized + BitVecExt,
    R: ?Sized + BitSliceExt<Bit = L::Bit>,
{
    out.iter_mut().zip(est.iter()).for_each(|(l, r)| *l = r);

    // 1.N = 1.N * 0.N, 1 <= rl < 1.5
    hi_mul(out, goal);
    // Calculate 2 - l, given that since we are using 1.N format that's equivalent to `0 - l` in
    // modulo arithmetic.
    // 1.N = 2.N - 1.N, 0.5 < 2-rl < 1
    <Element as AssignBitAlgo>::not(out);
    <Element as AssignAddAlgo>::wrapping(out, &[L::Bit::one()]);
    // 1.N = 1.N * 0.N, 0.5 <= l(2-rl) < 1
    hi_mul(out, est);
    // 0.N = 1.N
    <Element as AssignShlAlgo>::wrapping(out, 1);
}

fn leading_zeroes<L>(l: &L) -> usize
where
    L: ?Sized + BitSliceExt,
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
        .map_or_else(|a| a, |b| b)
}

impl DivRemAlgo for NewtonRaphson {
    fn long<L, R>(left: &L, right: &R) -> (Vec<L::Bit>, Vec<L::Bit>)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let one = L::Bit::one();
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let len = usize::max(left.len(), right.len());

        let mut norm_r = vec![L::Bit::zero(); len];
        let mut est = vec![L::Bit::zero(); len];
        let mut new_est = vec![L::Bit::zero(); len];

        // Count leading zeroes
        let zeroes = leading_zeroes(right) + L::Bit::BIT_LEN * (len - right.len());
        // Normalize value to have leading 1
        <Element as ShlAlgo>::wrapping(right, zeroes, &mut norm_r);

        // Get estimate based on leading non-zero bits
        let t = norm_r.get(norm_r.len() - 1).unwrap();
        let estimate = RECIP_TABLE[(t >> L::Bit::BIT_LEN - 4).saturate() as usize - 8];
        // Put estimate in high 8 bytes of a bit...
        let estimate = L::Bit::saturate_from(estimate) << (L::Bit::BIT_LEN - 8);

        // Convert estimate to fixed 0.N by putting it in high 8 bytes
        est.set_ignore(len - 1, estimate);

        // Newton estimates to refine reciprocal
        let mut bits = 4;
        while bit_len > bits {
            new_est.fill(L::Bit::zero());
            newton_step(&est, &norm_r, &mut new_est);
            est.copy_from_slice(&new_est);
            bits *= 2;
        }

        // Calculate quotient estimate and undo normalization
        let mut quotient = est;
        hi_mul(&mut quotient, left);
        <Element as AssignShrAlgo>::wrapping(&mut quotient, len * 8 - 1 - zeroes);

        if <Element as CmpAlgo>::cmp(&quotient, &[L::Bit::zero()]).is_gt() {
            <Element as AssignSubAlgo>::wrapping(&mut quotient, &[L::Bit::one()]);
        }

        norm_r.fill(L::Bit::zero());
        let mut remainder = norm_r;
        <Element as MulAlgo>::wrapping(&quotient, right, &mut remainder);

        // Calculate left - remainder
        <Element as AssignSubAlgo>::wrapping(&mut remainder, left);
        <Element as AssignBitAlgo>::not(&mut remainder);
        <Element as AssignAddAlgo>::wrapping(&mut remainder, &[L::Bit::one()]);

        // Correct quotient to handle possible error
        if <Element as CmpAlgo>::cmp(&remainder, right).is_ge() {
            <Element as AssignAddAlgo>::wrapping(&mut quotient, &[one]);
            <Element as AssignSubAlgo>::wrapping(&mut remainder, right);
            if <Element as CmpAlgo>::cmp(&remainder, right).is_ge() {
                <Element as AssignAddAlgo>::wrapping(&mut quotient, &[one]);
                <Element as AssignSubAlgo>::wrapping(&mut remainder, right);
                if <Element as CmpAlgo>::cmp(&remainder, right).is_ge() {
                    <Element as AssignAddAlgo>::wrapping(&mut quotient, &[one]);
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
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        todo!("{:?}, {:?}, {:?}, {:?}", left, right, quotient, remainder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leading_zeros() {
        assert_eq!(leading_zeroes(&[0b01000000u8]), 1);
        assert_eq!(leading_zeroes(&[0b10000000u8]), 0);
        assert_eq!(leading_zeroes(&[0x80, 0x00u8]), 8);
        assert_eq!(leading_zeroes(&[0x00, 0x01u8]), 7);
    }
}
