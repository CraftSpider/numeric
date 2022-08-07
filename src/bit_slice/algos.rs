use std::hint::unreachable_unchecked;
use num_traits::{PrimInt, Zero};

use super::{BitSlice, IndexOpt, Len};

type OwnedSlice<T> = BitSlice<Vec<T>>;

impl<S> BitSlice<S>
where
    S: IndexOpt<usize> + Len,
    S::Output: PrimInt,
{
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

            out.set_bit_pushing(idx, new);
        }

        out
    }

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
            remainder = remainder << 1;
            remainder.set_bit(0, num.get_bit(idx));
            if remainder >= div {
                // Ignore the bool - subtract will never overflow
                remainder = (remainder - div.clone()).0;
                quotient.set_bit(idx, true);
            }
        }

        (quotient, remainder)
    }
}
