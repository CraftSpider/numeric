use std::hint::unreachable_unchecked;

use crate::bit_slice::{BitSlice, IsBit};
use super::OwnedSlice;

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]>,
    I: IsBit,
{
    /// Shift a slice left by `usize` items, implemented as a series of bitwise swaps
    pub fn shl_bitwise(left: BitSlice<S, I>, right: usize) -> OwnedSlice<I> {
        let bit_len = left.bit_len();
        let mut out = BitSlice::new(vec![I::zero(); left.len()]);
        left.iter_bits()
            .enumerate()
            .for_each(|(idx, new)| {
                if new || idx + right < bit_len {
                    #[allow(clippy::suspicious_arithmetic_impl)]
                    out.set_bit_pushing(idx + right, new);
                }
            });
        out
    }

    /// Shift a slice right by `usize` items, implemented as a series of bitwise swaps
    pub fn shr_bitwise(left: BitSlice<S, I>, right: usize) -> OwnedSlice<I> {
        let mut out = BitSlice::new(vec![I::zero(); left.len()]);
        for idx in (0..=left.bit_len()).rev() {
            let new = left.get_bit_opt(idx).unwrap_or(false);
            if let Some(idx) = idx.checked_sub(right) {
                out.set_bit_ignore(idx, new);
            }
        }
        out
    }

    /// Add two slices, implemented as a bitwise add-and-carry
    pub fn add_bitwise<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> OwnedSlice<I>
    where
        T: AsRef<[I]>,
    {
        let len = usize::max(left.len(), right.len());
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let mut out = BitSlice::new(vec![I::zero(); len]);

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

            if new {
                out.set_bit_pushing(idx, new);
            }
        }

        out
    }

    /// Subtract two slices, implemented as a bitwise sub-and-borrow
    pub fn sub_bitwise<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> (OwnedSlice<I>, bool)
    where
        T: AsRef<[I]>,
    {
        let len = usize::max(left.len(), right.len());
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let mut out = BitSlice::new(vec![I::zero(); len]);

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
    pub fn add_shift_mul_bitwise<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> OwnedSlice<I>
    where
        T: AsRef<[I]>,
    {
        let len = usize::max(left.len(), right.len());
        let mut new_self = BitSlice::shl_wrap_and_mask(left, 0);
        let mut out = BitSlice::new(vec![I::zero(); len * 2]);

        for idx in 0..right.bit_len() {
            let r = right.get_bit(idx);
            if r {
                out = BitSlice::add_element(out, new_self.clone());
            }
            new_self = BitSlice::shl_wrap_and_mask(new_self, 1);
        }

        out
    }

    /// Divide two slices, implemented as bitwise long division
    pub fn div_long_bitwise<T>(num: BitSlice<S, I>, div: BitSlice<T, I>) -> (OwnedSlice<I>, OwnedSlice<I>)
    where
        OwnedSlice<I>: PartialOrd<BitSlice<T, I>>,
        T: Clone + AsRef<[I]>,
    {
        let len = usize::max(num.len(), div.len());
        let bit_len = usize::max(num.bit_len(), div.bit_len());

        let mut quotient = BitSlice::new(vec![I::zero(); len]);
        let mut remainder: BitSlice<_, _> = BitSlice::new(vec![I::zero(); len]);

        for idx in (0..bit_len).rev() {
            remainder = BitSlice::shl_wrap_and_mask(remainder, 1);
            remainder.set_bit(0, num.get_bit(idx));
            if remainder >= div {
                // Ignore the bool - subtract will never overflow
                remainder = BitSlice::sub_element(remainder, div.clone()).0;
                quotient.set_bit(idx, true);
            }
        }

        (quotient, remainder)
    }
}

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]> + AsMut<[I]>,
    I: IsBit,
{
    /// Add two slices, implemented as a bitwise add-and-carry, storing the result in the lefthand
    /// slice.
    pub fn add_in_place_bitwise<T>(mut left: BitSlice<S, I>, right: &BitSlice<T, I>) -> BitSlice<S, I>
    where
        T: AsRef<[I]>,
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
