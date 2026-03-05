use super::AssignAddAlgo;
use crate::algos::{Add, Algo, BitOwned, Bitwise, Element};
use crate::bit_slice::{BitLike, BitSliceExt};
use core::hint::unreachable_unchecked;
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::overflowing::OverflowingAdd;

impl Algo<Add> for Element {
    const SATURATE_HIGH: bool = true;

    fn overflowing<L, R, O>(left: &L, right: &R) -> (O, bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();
        let one = L::Bit::one();

        let mut overflow = false;
        let mut carry = false;
        let mut out = O::zeroed(len + 1);

        for idx in 0..=len {
            let l = left.get(idx).unwrap_or(zero);
            let r = right.get(idx).unwrap_or(zero);

            let (res, new_carry) = l.overflowing_add(if carry { one } else { zero });
            carry = new_carry;

            let (res, new_carry) = res.overflowing_add(r);
            // As of Rust 1.86 nightly, this is faster than `carry |= new_carry`
            if new_carry {
                carry = true;
            }

            match out.set_opt(idx, res) {
                None if !res.is_zero() => overflow = true,
                _ => (),
            }
        }

        (out.shrink(), overflow)
    }
}

impl AssignAddAlgo for Element {
    fn overflowing<L, R>(left: &mut L, right: &R) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();
        let one = L::Bit::one();

        let mut overflow = false;
        let mut carry = false;

        for idx in 0..=len {
            let l = left.get(idx).unwrap_or(zero);
            let r = right.get(idx).unwrap_or(zero);

            let (res, new_carry) = l.overflowing_add(if carry { one } else { zero });
            carry = new_carry;

            let (res, new_carry) = res.overflowing_add(r);
            // As of Rust 1.86 nightly, this is faster than `carry |= new_carry`
            if new_carry {
                carry = true;
            }

            match left.set_opt(idx, res) {
                None if !res.is_zero() => overflow = true,
                _ => (),
            }
        }

        overflow
    }
}

impl Algo<Add> for Bitwise {
    const SATURATE_HIGH: bool = true;

    fn overflowing<L, R, O>(left: &L, right: &R) -> (O, bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
        O: BitOwned,
    {
        extern crate std;
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let mut out = O::zeroed(bit_len / L::Bit::BIT_LEN + 1);

        let mut overflow = false;
        let mut carry = false;
        for idx in 0..=bit_len {
            let l = u8::from(left.get_bit(idx).unwrap_or(false));
            let r = u8::from(right.get_bit(idx).unwrap_or(false));

            let c = if carry {
                carry = false;
                1
            } else {
                0
            };

            let new = match c + l + r {
                0 => false,
                1 => true,
                2 => {
                    carry = true;
                    false
                }
                3 => {
                    carry = true;
                    true
                }
                // SAFETY: `c`, `l`, and `r` in range 0..=1, can't be a value beyond 3
                _ => unsafe { unreachable_unchecked() },
            };

            match out.set_bit_opt(idx, new) {
                None if new => overflow = true,
                _ => (),
            }
        }

        (out.shrink(), overflow)
    }
}

impl AssignAddAlgo for Bitwise {
    fn overflowing<L, R>(left: &mut L, right: &R) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());

        let mut overflow = false;
        let mut carry = false;
        for idx in 0..=bit_len {
            let l = u8::from(left.get_bit(idx).unwrap_or(false));
            let r = u8::from(right.get_bit(idx).unwrap_or(false));

            let c = if carry {
                carry = false;
                1
            } else {
                0
            };

            let new = match c + l + r {
                0 => false,
                1 => true,
                2 => {
                    carry = true;
                    false
                }
                3 => {
                    carry = true;
                    true
                }
                // SAFETY: `c`, `l`, and `r` in range 0..=1, can't be a value beyond 3
                _ => unsafe { unreachable_unchecked() },
            };

            match left.set_bit_opt(idx, new) {
                None if new => overflow = true,
                _ => (),
            }
        }

        overflow
    }
}
