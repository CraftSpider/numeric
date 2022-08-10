use std::hint::unreachable_unchecked;
use num_traits::{PrimInt, Zero};

use crate::bit_slice::{BitSlice, IndexOpt, IndexOptMut, Len};
use super::OwnedSlice;

impl<S> BitSlice<S>
where
    S: IndexOpt<usize> + Len,
    S::Output: PrimInt,
{
    /// Shift a slice left by `usize` items, implemented as a series of bitwise swaps
    pub fn shl_bitwise(left: BitSlice<S>, right: usize) -> OwnedSlice<S::Output> {
        let bit_len = left.bit_len();
        let mut out = BitSlice::new(vec![<S::Output>::zero(); left.len()]);
        for (idx, new) in left.iter_bits() {
            if new || idx + right < bit_len {
                #[allow(clippy::suspicious_arithmetic_impl)]
                out.set_bit_pushing(idx + right, new);
            }
        }
        out
    }

    /// Shift a slice right by `usize` items, implemented as a series of bitwise swaps
    pub fn shr_bitwise(left: BitSlice<S>, right: usize) -> OwnedSlice<S::Output> {
        let mut out = BitSlice::new(vec![<S::Output>::zero(); left.len()]);
        for idx in (0..=left.bit_len()).rev() {
            let new = left.get_bit(idx);
            if let Some(idx) = idx.checked_sub(right) {
                out.set_bit_ignore(idx, new);
            }
        }
        out
    }

    /// Add two slices, implemented as a bitwise add-and-carry
    pub fn add_bitwise<T>(left: BitSlice<S>, right: BitSlice<T>) -> OwnedSlice<S::Output>
        where
            T: IndexOpt<usize> + Len,
            T::Output: PrimInt,
    {
        let len = usize::max(left.len(), right.len());
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let mut out = BitSlice::new(vec![<S::Output>::zero(); len]);

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
                _ => unsafe { unreachable_unchecked() },
            };

            out.set_bit_pushing(idx, new);
        }

        out
    }

    /// Subtract two slices, implemented as a bitwise sub-and-borrow
    pub fn sub_bitwise<T>(left: BitSlice<S>, right: BitSlice<T>) -> (OwnedSlice<S::Output>, bool)
        where
            T: IndexOpt<usize> + Len,
            T::Output: PrimInt,
    {
        let len = usize::max(left.len(), right.len());
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let mut out = BitSlice::new(vec![<S::Output>::zero(); len]);

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
            out = -out;
        }

        (out, carry)
    }

    /// Multiply two slices, implemented as a bitwise shift-and-add
    pub fn add_shift_mul_bitwise<T>(left: BitSlice<S>, right: BitSlice<T>) -> OwnedSlice<S::Output>
        where
            T: IndexOpt<usize> + Len,
            T::Output: PrimInt,
    {
        let len = usize::max(left.len(), right.len());
        let mut new_self = BitSlice::shl_bitwise(left, 0);
        let mut out = BitSlice::new(vec![<S::Output>::zero(); len * 2]);

        for idx in 0..right.bit_len() {
            let r = right.get_bit(idx);
            if r {
                out = BitSlice::add_bitwise(out, new_self.clone());
            }
            new_self = BitSlice::shl_bitwise(new_self, 1);
        }

        out
    }

    /// Divide two slices, implemented as bitwise long division
    pub fn long_div_bitwise<T>(num: BitSlice<S>, div: BitSlice<T>) -> (OwnedSlice<S::Output>, OwnedSlice<S::Output>)
        where
            OwnedSlice<S::Output>: PartialOrd<BitSlice<T>>,
            T: IndexOpt<usize> + Len + Clone,
            T::Output: PrimInt,
    {
        let len = usize::max(num.len(), div.len());
        let bit_len = usize::max(num.bit_len(), div.bit_len());

        let mut quotient = BitSlice::new(vec![<S::Output>::zero(); len]);
        let mut remainder: BitSlice<_> = BitSlice::new(vec![<S::Output>::zero(); len]);

        for idx in (0..bit_len).rev() {
            remainder = BitSlice::shl_bitwise(remainder, 1);
            remainder.set_bit(0, num.get_bit(idx));
            if remainder >= div {
                // Ignore the bool - subtract will never overflow
                remainder = BitSlice::sub_bitwise(remainder, div.clone()).0;
                quotient.set_bit(idx, true);
            }
        }

        (quotient, remainder)
    }
}

impl<S> BitSlice<S>
    where
        S: IndexOptMut<usize> + Len,
        S::Output: PrimInt,
{
    /// Add two slices, implemented as a bitwise add-and-carry, storing the result in the lefthand
    /// slice.
    pub fn add_in_place_bitwise<T>(mut left: BitSlice<S>, right: &BitSlice<T>) -> BitSlice<S>
        where
            T: IndexOpt<usize> + Len,
            T::Output: PrimInt,
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
                _ => unsafe { unreachable_unchecked() },
            };

            left.set_bit(idx, new);
        }

        left
    }
}
