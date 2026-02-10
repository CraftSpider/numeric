#[cfg(feature = "alloc")]
use crate::algos::AssignBitAlgo;
use crate::algos::{AssignSubAlgo, Bitwise, Element, SubAlgo};
use crate::bit_slice::BitSliceExt;
#[cfg(feature = "alloc")]
use crate::utils::IntSlice;
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::overflowing::OverflowingSub;

impl SubAlgo for Element {
    #[cfg(feature = "alloc")]
    fn long<L, R>(left: &L, right: &R) -> (Vec<L::Bit>, bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();
        let one = L::Bit::one();
        let mut out = vec![zero; len];

        let mut carry = false;
        for idx in 0..len {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

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

        if carry {
            out.set_bit(0, !out.get_bit(0));
            <Element as AssignBitAlgo>::not(&mut out);
        }

        (IntSlice::shrink(out), carry)
    }

    fn overflowing<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> (&'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();
        let one = L::Bit::one();

        let mut carry = false;

        for idx in 0..len {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

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

        (out, carry)
    }
}

impl AssignSubAlgo for Element {
    fn overflowing<L, R>(left: &mut L, right: &R) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();
        let one = L::Bit::one();

        let mut carry = false;

        for idx in 0..len {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

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

impl SubAlgo for Bitwise {
    #[cfg(feature = "alloc")]
    fn long<L, R>(left: &L, right: &R) -> (Vec<L::Bit>, bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let mut out = vec![L::Bit::zero(); len];

        let mut carry = false;
        for idx in 0..bit_len {
            let l = left.get_bit_opt(idx).unwrap_or(false);
            let r = right.get_bit_opt(idx).unwrap_or(false);

            let c = if carry {
                carry = false;
                true
            } else {
                false
            };

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

            out.set_bit(idx, new);
        }

        if carry {
            out.set_bit(0, !out.get_bit(0));
            <Element as AssignBitAlgo>::not(&mut out);
        }

        (IntSlice::shrink(out), carry)
    }

    fn overflowing<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> (&'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());

        let mut carry = false;
        for idx in 0..bit_len {
            let l = left.get_bit_opt(idx).unwrap_or(false);
            let r = right.get_bit_opt(idx).unwrap_or(false);

            let c = if carry {
                carry = false;
                true
            } else {
                false
            };

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

        (out, carry)
    }
}
