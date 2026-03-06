use crate::algos::{Algo, AssignAlgo, Bitwise, Element, Sub};
use crate::bit_slice::{BitLike, BitOwned, BitSlice};
use core::mem;
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::overflowing::OverflowingSub;

impl Algo<Sub> for Element {
    fn overflowing<O, L, R>(left: &L, right: &R) -> (O, bool)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let mut out = O::zeroed(len);
        let overflow = <Self as Algo<Sub>>::overflowing_into::<L, R, O>(left, right, &mut out);
        (out, overflow)
    }

    fn overflowing_into<L, R, O>(left: &L, right: &R, out: &mut O) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();
        let one = L::Bit::one();

        let mut carry = false;

        for idx in 0..len {
            let l = left.get(idx).unwrap_or(zero);
            let r = right.get(idx).unwrap_or(zero);

            let extra = if carry {
                carry = false;
                one
            } else {
                zero
            };

            let (res, new_carry) = l.overflowing_sub(r);
            if new_carry {
                carry = true;
            }

            let (res, new_carry) = res.overflowing_sub(extra);
            if new_carry {
                carry = true;
            }

            out.set_ignore(idx, res);
        }

        out.shrink();
        carry
    }
}

impl AssignAlgo<Sub> for Element {
    fn overflowing<O, L, R>(left: &mut L, right: &R) -> bool
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();
        let one = L::Bit::one();

        let mut carry = false;

        for idx in 0..len {
            let l = left.get(idx).unwrap_or(zero);
            let r = right.get(idx).unwrap_or(zero);

            let extra = if carry {
                carry = false;
                one
            } else {
                zero
            };

            let (res, new_carry) = l.overflowing_sub(r);
            if new_carry {
                carry = true;
            }

            let (res, new_carry) = res.overflowing_sub(extra);
            if new_carry {
                carry = true;
            }

            left.set_ignore(idx, res);
        }

        carry
    }
}

impl Algo<Sub> for Bitwise {
    fn overflowing<O, L, R>(left: &L, right: &R) -> (O, bool)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let mut out = O::zeroed(bit_len / L::Bit::BIT_LEN);
        let overflow = <Self as Algo<Sub>>::overflowing_into::<L, R, O>(left, right, &mut out);
        (out, overflow)
    }

    fn overflowing_into<L, R, O>(left: &L, right: &R, out: &mut O) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());

        let mut carry = false;
        for idx in 0..bit_len {
            let l = left.get_bit(idx).unwrap_or(false);
            let r = right.get_bit(idx).unwrap_or(false);

            let c = mem::take(&mut carry);

            let new = match (l, r, c) {
                (true, false, false) => true,
                (true, true, false) | (true, false, true) | (false, false, false) => false,
                (false, true, false) | (false, false, true) | (true, true, true) => {
                    carry = true;
                    true
                }
                (false, true, true) => {
                    carry = true;
                    false
                }
            };

            out.set_bit_ignore(idx, new);
        }

        out.shrink();
        carry
    }
}

impl AssignAlgo<Sub> for Bitwise {
    fn overflowing<O, L, R>(left: &mut L, right: &R) -> bool
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());

        let mut carry = false;
        for idx in 0..bit_len {
            let l = left.get_bit(idx).unwrap_or(false);
            let r = right.get_bit(idx).unwrap_or(false);

            let c = mem::take(&mut carry);

            let new = match (l, r, c) {
                (true, false, false) => true,
                (true, true, false) | (true, false, true) | (false, false, false) => false,
                (false, true, false) | (false, false, true) | (true, true, true) => {
                    carry = true;
                    true
                }
                (false, true, true) => {
                    carry = true;
                    false
                }
            };

            left.set_bit_ignore(idx, new);
        }

        carry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::{Bitwise, Element};
    #[cfg(feature = "alloc")]
    use alloc::{vec, vec::Vec};

    #[cfg(feature = "alloc")]
    fn test_long<B: Algo<Sub>>() {
        // Simple subtraction
        assert_eq!(
            B::overflowing::<Vec<_>, _, _>(&[0u32], &[0]),
            (vec![0], false)
        );
        assert_eq!(
            B::overflowing::<Vec<_>, _, _>(&[1u32], &[0]),
            (vec![1], false)
        );
        assert_eq!(
            B::overflowing::<Vec<_>, _, _>(&[0u32], &[1]),
            (vec![4294967295], true)
        );
        assert_eq!(
            B::overflowing::<Vec<_>, _, _>(&[1u32], &[1]),
            (vec![0], false)
        );

        // Long subtraction handled correctly
        assert_eq!(
            B::overflowing::<Vec<_>, _, _>(&[0u32, 1], &[0, 1]),
            (vec![0], false)
        );
        assert_eq!(
            B::overflowing::<Vec<_>, _, _>(&[1u32, 1], &[1]),
            (vec![0, 1], false)
        );
        assert_eq!(
            B::overflowing::<Vec<_>, _, _>(&[1u32, 1], &[0, 1]),
            (vec![1], false)
        );
        assert_eq!(
            B::overflowing::<Vec<_>, _, _>(&[0u32, 1], &[1]),
            (vec![u32::MAX], false)
        );
    }

    fn test_wrapping<B: Algo<Sub>>() {
        // Simple subtraction
        assert_eq!(B::wrapping::<[u32; 1], _, _>(&[0u32], &[0]), [0]);
        assert_eq!(B::wrapping::<[u32; 1], _, _>(&[1u32], &[0]), [1]);
        assert_eq!(B::wrapping::<[u32; 1], _, _>(&[0u32], &[1]), [u32::MAX]);
        assert_eq!(B::wrapping::<[u32; 1], _, _>(&[1u32], &[1]), [0]);

        // Long subtraction handled correctly
        assert_eq!(B::wrapping::<[u32; 2], _, _>(&[0u32, 1], &[0, 1]), [0, 0]);
        assert_eq!(B::wrapping::<[u32; 2], _, _>(&[1u32, 1], &[1]), [0, 1]);
        assert_eq!(B::wrapping::<[u32; 2], _, _>(&[1u32, 1], &[0, 1]), [1, 0]);
        assert_eq!(
            B::wrapping::<[u32; 2], _, _>(&[0u32, 1], &[1]),
            [u32::MAX, 0]
        );
    }

    fn test_saturating<B: Algo<Sub>>() {
        assert_eq!(B::saturating::<[u32; 1], _, _>(&[0], &[0]), [0]);
        assert_eq!(B::saturating::<[u32; 1], _, _>(&[1], &[1]), [0]);
        assert_eq!(B::saturating::<[u32; 1], _, _>(&[1], &[u32::MAX]), [0]);
        assert_eq!(B::saturating::<[u32; 1], _, _>(&[0], &[1]), [0]);
        assert_eq!(
            B::saturating::<[u32; 1], _, _>(&[u32::MAX], &[1]),
            [u32::MAX - 1]
        );
    }

    #[test]
    fn test_element() {
        #[cfg(feature = "alloc")]
        test_long::<Element>();
        test_wrapping::<Element>();
        test_saturating::<Element>();
    }

    #[test]
    fn test_bitwise() {
        #[cfg(feature = "alloc")]
        test_long::<Bitwise>();
        test_wrapping::<Bitwise>();
        test_saturating::<Bitwise>();
    }
}
