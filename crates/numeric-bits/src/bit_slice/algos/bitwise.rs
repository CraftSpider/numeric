use std::hint::unreachable_unchecked;
use numeric_traits::identity::Zero;
use crate::bit_slice::{BitSliceExt, BitVecExt};
use crate::bit_slice::algos::element::{ElementAdd, ElementNot, ElementShl, ElementSub};
use crate::bit_slice::algos::ElementCmp;

pub trait BitwiseOps: BitSliceExt {
    /// Shift a slice left by `usize` items, implemented as a series of bitwise swaps
    fn shl(left: &Self, right: usize) -> Vec<Self::Bit> {
        let bit_len = left.bit_len();
        let mut out = vec![Self::Bit::zero(); left.len()];
        left.iter_bits()
            .enumerate()
            .for_each(|(idx, new)| {
                if new || idx + right < bit_len {
                    #[allow(clippy::suspicious_arithmetic_impl)]
                    out.set_bit_push(idx + right, new);
                }
            });
        out
    }

    /// Shift a slice right by `usize` items, implemented as a series of bitwise swaps
    fn shr(left: &Self, right: usize) -> Vec<Self::Bit> {
        let mut out = vec![Self::Bit::zero(); left.len()];
        for idx in (0..=left.bit_len()).rev() {
            let new = left.get_bit_opt(idx).unwrap_or(false);
            if let Some(idx) = idx.checked_sub(right) {
                out.set_bit_ignore(idx, new);
            }
        }
        out
    }

    /// Add two slices, implemented as a bitwise add-and-carry
    fn add<T>(left: &Self, right: &T) -> Vec<Self::Bit>
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>
    {
        let len = usize::max(left.len(), right.len());
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let mut out = vec![Self::Bit::zero(); len];

        let mut carry = false;
        for idx in 0..=bit_len {
            let l = left.get_bit_opt(idx).unwrap_or(false) as u8;
            let r = right.get_bit_opt(idx).unwrap_or(false) as u8;

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

    /// Subtract two slices, implemented as a bitwise sub-and-borrow
    fn sub<T>(left: &Self, right: &T) -> (Vec<Self::Bit>, bool)
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>
    {
        let len = usize::max(left.len(), right.len());
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let mut out = vec![Self::Bit::zero(); len];

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
            ElementNot::not(&mut out);
        }

        (out, carry)
    }

    /// Multiply two slices, implemented as a bitwise shift-and-add
    fn add_shift_mul<T>(left: &Self, right: &T) -> Vec<Self::Bit>
    where
        T: BitSliceExt<Bit = Self::Bit>
    {
        let len = usize::max(left.len(), right.len());
        let mut new_self =  ElementShl::shl(left, 0);
        let mut out = vec![Self::Bit::zero(); len * 2];

        for idx in 0..right.bit_len() {
            let r = right.get_bit(idx);
            if r {
                out = ElementAdd::add(&out, &new_self);
            }
            new_self = ElementShl::shl(&new_self, 1);
        }

        out
    }

    /// Divide two slices, implemented as bitwise long division
    fn div_long<T>(num: &Self, div: &T) -> (Vec<Self::Bit>, Vec<Self::Bit>)
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let len = usize::max(num.len(), div.len());
        let bit_len = usize::max(num.bit_len(), div.bit_len());

        let mut quotient = vec![Self::Bit::zero(); len];
        let mut remainder = vec![Self::Bit::zero(); len];

        for idx in (0..bit_len).rev() {
            remainder = ElementShl::shl(&remainder, 1);
            remainder.set_bit(0, num.get_bit(idx));
            if ElementCmp::cmp(&remainder, div).is_ge() {
                // Ignore the bool - subtract will never overflow
                remainder = ElementSub::sub(&remainder, div).0;
                quotient.set_bit(idx, true);
            }
        }

        (quotient, remainder)
    }

    /// Add two slices, implemented as a bitwise add-and-carry, storing the result in the lefthand
    /// slice.
    fn add_in_place<'a, T>(left: &'a mut Self, right: &T) -> &'a mut Self
    where
        T: BitSliceExt<Bit = Self::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());

        let mut carry = false;
        for idx in 0..=bit_len {
            let l = left.get_bit(idx) as u8;
            let r = right.get_bit(idx) as u8;

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

        left
    }
}

impl<T> BitwiseOps for T
where
    T: ?Sized + BitSliceExt,
{}
