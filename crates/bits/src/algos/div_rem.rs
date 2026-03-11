use crate::algos::{
    Add, Algo, AssignAlgo, AssignBitAlgo, Bitwise, CmpAlgo, Div, DivRem, Element, Mul,
    MultiBitOwned, Rem, Shl, Shr, Sub,
};
use crate::bit_slice::{BitLike, BitOwned, BitSlice};
use core::mem;
use numeric_traits::cast::{FromSaturating, IntoSaturating};
use numeric_traits::class::BoundedBit;
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::overflowing::OverflowingAdd;
use numeric_traits::ops::widening::WideningMul;

pub struct NewtonRaphson;

impl Algo<DivRem> for Bitwise {
    fn overflowing<O, L, R>(left: &L, right: &R) -> ((O, O), bool)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let mut out = (O::zeroed(len), O::zeroed(len));
        let overflow = <Self as Algo<DivRem>>::overflowing_into::<L, R, O>(left, right, &mut out);
        (out, overflow)
    }

    fn overflowing_into<L, R, O>(left: &L, right: &R, (quotient, remainder): &mut (O, O)) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        for idx in (0..bit_len).rev() {
            <Element as AssignAlgo<Shl>>::wrapping::<O, _, [_]>(remainder, 1);
            remainder.set_bit(0, left.get_bit(idx).unwrap_or(false));
            if <Element as CmpAlgo>::ge(remainder, right) {
                // Subtract will never overflow
                <Element as AssignAlgo<Sub>>::wrapping::<[L::Bit; 0], _, _>(remainder, right);
                quotient.set_bit(idx, true);
            } else {
                quotient.set_bit(idx, false);
            }
        }
        false
    }
}

impl AssignAlgo<Div> for Bitwise {
    fn overflowing<O, L, R>(left: &mut L, right: &R) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let len = usize::max(left.len(), right.len());

        let mut remainder = O::zeroed(len);

        for idx in (0..bit_len).rev() {
            <Element as AssignAlgo<Shl>>::wrapping::<O, _, [_]>(&mut remainder, 1);
            remainder.set_bit(0, left.get_bit(idx).unwrap_or(false));
            if <Element as CmpAlgo>::ge(&remainder, right) {
                // Subtract will never overflow
                <Element as AssignAlgo<Sub>>::wrapping::<[L::Bit; 0], _, _>(&mut remainder, right);
                left.set_bit(idx, true);
            } else {
                left.set_bit(idx, false);
            }
        }

        false
    }
}

impl AssignAlgo<Rem> for Bitwise {
    fn overflowing<O, L, R>(left: &mut L, right: &R) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let len = usize::max(left.len(), right.len());

        let mut remainder = O::zeroed(len);

        for idx in (0..bit_len).rev() {
            <Element as AssignAlgo<Shl>>::wrapping::<O, _, [_]>(&mut remainder, 1);
            remainder.set_bit(0, left.get_bit(idx).unwrap_or(false));
            if <Element as CmpAlgo>::ge(&remainder, right) {
                // Subtract will never overflow
                <Element as AssignAlgo<Sub>>::wrapping::<[L::Bit; 0], _, _>(&mut remainder, right);
                left.set_bit(idx, true);
            } else {
                left.set_bit(idx, false);
            }
        }

        left.iter_mut()
            .zip(remainder.iter())
            .for_each(|(l, r)| *l = r);

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
fn hi_mul<L, R, O>(left: &L, right: &R, out: &mut O)
where
    L: ?Sized + BitSlice,
    R: ?Sized + BitSlice<Bit = L::Bit>,
    O: ?Sized + BitSlice<Bit = L::Bit>,
{
    // We multiply into out, wrapping around our overflow value for each iteration
    let zero = L::Bit::zero();
    out.iter_mut().for_each(|v| *v = zero);

    for (idx, l) in left.iter().enumerate() {
        let mut carry = out.get(idx).unwrap();

        // We clear the current index - it will be overwritten by the new high value
        out.set(idx, zero);

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
}

/// l is treated as fixed 1.N - range [1, 2)
/// r is treated as fixed 0.N - range [0.5, 1)
/// output will be fixed 0.N
fn newton_step<L, R, O>(est: &L, goal: &R, out: &mut O, scratch: &mut O)
where
    L: ?Sized + BitSlice,
    R: ?Sized + BitSlice<Bit = L::Bit>,
    O: ?Sized + BitSlice<Bit = L::Bit>,
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
    <Element as AssignAlgo<Shl>>::wrapping::<[L::Bit; 0], _, [_]>(out, 1);
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

impl Algo<DivRem> for NewtonRaphson {
    fn overflowing<O, L, R>(left: &L, right: &R) -> ((O, O), bool)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let mut out = (O::zeroed(len), O::zeroed(len));
        let overflow = <Self as Algo<DivRem>>::overflowing_into(left, right, &mut out);
        (out, overflow)
    }

    fn overflowing_into<L, R, O>(left: &L, right: &R, (quotient, remainder): &mut (O, O)) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let one = L::Bit::one();
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let len = usize::max(left.len(), right.len());

        let norm_r = &mut *remainder;
        let mut est = &mut O::zeroed(len);
        let mut new_est = &mut O::zeroed(len);
        let scratch = &mut *quotient;

        // Count leading zeroes
        let zeroes = leading_zeroes(right) + L::Bit::BIT_LEN * (len - right.len());
        // Normalize value to have leading 1
        <Element as Algo<Shl>>::wrapping_into::<_, R, _>(right, zeroes, norm_r);

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
            newton_step(est, norm_r, new_est, scratch);
            mem::swap(&mut est, &mut new_est);
            bits *= 2;
        }

        // Calculate quotient estimate and undo normalization
        hi_mul(est, left, scratch);
        <Element as AssignAlgo<Shr>>::wrapping::<O, _, [_]>(
            quotient,
            len * L::Bit::BIT_LEN - 1 - zeroes,
        );

        if quotient.iter().any(|v| v != L::Bit::zero()) {
            <Element as AssignAlgo<Sub>>::wrapping::<O, _, _>(quotient, &[one]);
        }

        remainder.fill(L::Bit::zero());
        <Element as Algo<Mul>>::wrapping_into(quotient, right, remainder);

        // Calculate left - remainder
        <Element as AssignAlgo<Sub>>::wrapping::<O, _, _>(remainder, left);
        <Element as AssignBitAlgo>::not(remainder);
        <Element as AssignAlgo<Add>>::wrapping::<O, _, _>(remainder, &[one]);

        // Correct quotient to handle possible error
        if <Element as CmpAlgo>::cmp(remainder, right).is_ge() {
            <Element as AssignAlgo<Add>>::wrapping::<O, _, _>(quotient, &[one]);
            <Element as AssignAlgo<Sub>>::wrapping::<O, _, _>(remainder, right);
            if <Element as CmpAlgo>::cmp(remainder, right).is_ge() {
                <Element as AssignAlgo<Add>>::wrapping::<O, _, _>(quotient, &[one]);
                <Element as AssignAlgo<Sub>>::wrapping::<O, _, _>(remainder, right);
                if <Element as CmpAlgo>::cmp(remainder, right).is_ge() {
                    <Element as AssignAlgo<Add>>::wrapping::<O, _, _>(quotient, &[one]);
                }
            }
        }

        false
    }
}

impl AssignAlgo<Div> for NewtonRaphson {
    fn overflowing<O, L, R>(left: &mut L, right: &R) -> bool
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let ((q, _), overflow) = <Self as Algo<DivRem>>::overflowing::<O, L, R>(left, right);
        left.iter_mut().zip(q.iter()).for_each(|(l, r)| *l = r);
        overflow
    }
}

impl AssignAlgo<Rem> for NewtonRaphson {
    fn overflowing<O, L, R>(left: &mut L, right: &R) -> bool
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let ((_, r), overflow) = <Self as Algo<DivRem>>::overflowing::<O, L, R>(left, right);
        left.iter_mut().zip(r.iter()).for_each(|(l, r)| *l = r);
        overflow
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::{Algo, Bitwise, DivRem};

    #[cfg(feature = "alloc")]
    fn test_div_long<B: Algo<DivRem>>() {
        use alloc::vec::Vec;
        let slice1: &[u8] = &[0b10];
        let slice2: &[u8] = &[0b01];

        assert_eq!(B::wrapping::<Vec<_>, _, _>(slice1, slice2).0, [0b10]);

        let slice3: &[u8] = &[0b10];
        let slice4: &[u8] = &[0b10];

        assert_eq!(B::wrapping::<Vec<_>, _, _>(slice3, slice4).0, [0b01]);

        let slice5: &[u8] = &[0b0000_0000, 0b1];
        let slice6: &[u8] = &[0b0000_0010];

        assert_eq!(
            B::wrapping::<Vec<_>, _, _>(slice5, slice6).0,
            [0b1000_0000, 0b0]
        );

        let slice7: &[u8] = &[0b0, 0b0, 0b0, 0b1];
        let slice8: &[u8] = &[0b10];

        assert_eq!(
            B::wrapping::<Vec<_>, _, _>(slice7, slice8).0,
            [0b0, 0b0, 0b1000_0000, 0b0]
        );
    }

    #[cfg(feature = "alloc")]
    fn test_rem_long<B: Algo<DivRem>>() {
        use alloc::vec::Vec;
        for i in 0..4 {
            let slice1: &[u8] = &[i];
            let slice2 = &[0b10];

            assert_eq!(B::wrapping::<Vec<_>, _, _>(slice1, slice2).1, &[i % 2]);
        }

        for i in 0..6 {
            let slice3: &[u8] = &[i];
            let slice4 = &[0b11];

            assert_eq!(B::wrapping::<Vec<_>, _, _>(slice3, slice4).1, &[i % 3]);
        }

        let slice5: &[u8] = &[0b0000_0001, 0b111];
        let slice6 = &[0b0000_0010];

        assert_eq!(B::wrapping::<Vec<_>, _, _>(slice5, slice6).1, &[0b01, 0b0]);
    }

    fn test_div_wrapping<B: Algo<DivRem>>() {
        let data = &[0b10u8];
        let slice2: &[u8] = &[0b01];

        assert_eq!(B::wrapping::<[u8; 1], _, _>(data, slice2).0, [0b10]);

        let data = &[0b10u8];
        let slice4: &[u8] = &[0b10];

        assert_eq!(B::wrapping::<[u8; 1], _, _>(data, slice4).0, [0b01]);

        let data = &[0b0000_0000u8, 0b1];
        let slice6: &[u8] = &[0b0000_0010];

        assert_eq!(
            B::wrapping::<[u8; 2], _, _>(data, slice6).0,
            [0b1000_0000, 0b0]
        );

        let data = &[0b0u8, 0b0, 0b0, 0b1];
        let slice8: &[u8] = &[0b10];

        assert_eq!(
            B::wrapping::<[u8; 4], _, _>(data, slice8).0,
            [0b0, 0b0, 0b1000_0000, 0b0]
        );
    }

    #[test]
    fn test_bitwise() {
        #[cfg(feature = "alloc")]
        test_div_long::<Bitwise>();
        #[cfg(feature = "alloc")]
        test_rem_long::<Bitwise>();

        test_div_wrapping::<Bitwise>();
    }

    #[test]
    fn test_nr() {
        #[cfg(feature = "alloc")]
        test_div_long::<NewtonRaphson>();
        #[cfg(feature = "alloc")]
        test_rem_long::<NewtonRaphson>();

        test_div_wrapping::<NewtonRaphson>();
    }

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
        assert_eq!(out, &[0b1111_1110, 0b1111_1111]);
    }

    #[test]
    fn test_leading_zeros() {
        assert_eq!(leading_zeroes(&[0b0100_0000u8]), 1);
        assert_eq!(leading_zeroes(&[0b1000_0000u8]), 0);
        assert_eq!(leading_zeroes(&[0x80, 0x00u8]), 8);
        assert_eq!(leading_zeroes(&[0x00, 0x01u8]), 7);
    }
}
