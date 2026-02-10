use crate::algos::{Add, AssignBinAlg, BinAlg, Bitwise, Operation};
use crate::bit_slice::{BitSliceExt, BitVecExt};
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use core::hint::unreachable_unchecked;
use numeric_traits::identity::Zero;

impl BinAlg<Add> for Bitwise {
    #[cfg(feature = "alloc")]
    fn growing<L, R>(left: &L, right: &R) -> Vec<L::Bit>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let mut out = vec![L::Bit::zero(); len];

        let mut carry = false;
        for idx in 0..=bit_len {
            let l = u8::from(left.get_bit_opt(idx).unwrap_or(false));
            let r = u8::from(right.get_bit_opt(idx).unwrap_or(false));

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

            if new {
                out.set_bit_push(idx, new);
            }
        }

        out
    }

    fn overflowing<'a, L, R>(
        left: &L,
        right: &R,
        out: <Add as Operation>::Out<&'a mut [L::Bit]>,
    ) -> (&'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());

        let mut carry = false;
        for idx in 0..=bit_len {
            let l = u8::from(left.get_bit(idx));
            let r = u8::from(right.get_bit(idx));

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

            out.set_bit(idx, new);
        }

        (out, carry)
    }

    fn saturating<'a, L, R>(
        left: &L,
        right: &R,
        out: <Add as Operation>::Out<&'a mut [L::Bit]>,
    ) -> &'a [L::Bit]
    where
        L: BitSliceExt,
        R: BitSliceExt<Bit = L::Bit>,
    {
        todo!()
    }
}

impl AssignBinAlg<Add> for Bitwise {
    fn overflowing<L, R>(left: &mut L, right: &R, extra: ()) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());

        let mut carry = false;
        for idx in 0..=bit_len {
            let l = u8::from(left.get_bit(idx));
            let r = u8::from(right.get_bit(idx));

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

            left.set_bit(idx, new);
        }

        carry
    }

    fn saturating<L, R>(left: &L, right: &R, out: ())
    where
        L: BitSliceExt,
        R: BitSliceExt<Bit = L::Bit>,
    {
        todo!()
    }
}
