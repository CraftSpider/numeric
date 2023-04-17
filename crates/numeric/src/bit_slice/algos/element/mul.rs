use crate::bit_slice::{BitSlice, IsBit};
use crate::utils::shrink_vec;
use crate::bit_slice::algos::OwnedSlice;
use crate::traits::WideningMul;

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]>,
    I: IsBit + WideningMul,
{
    /// Multiply two slices, implemented as shift-and-add
    pub fn mul_long_element<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> OwnedSlice<I>
    where
        T: AsRef<[I]>,
    {
        let zero = I::zero();
        let mut out = BitSlice::new(vec![zero; left.len() + right.len()]);

        left.slice()
            .iter()
            .enumerate()
            .for_each(|(idx, &l)| {
                let mut carry = zero;

                for (offset, &r) in right.slice().iter().enumerate() {
                    let (low, high) = I::widening_mul(l, r, carry);
                    carry = high;
                    out.add_item(idx + offset, low);
                }

                if carry != zero {
                    out.add_item(idx + right.slice().len(), carry);
                }
            });

        BitSlice::new(shrink_vec(out.into_inner()))
    }
}

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]> + AsMut<[I]>,
    I: IsBit + WideningMul,
{
    fn add_item(&mut self, mut idx: usize, mut val: I) -> bool {
        let slice = self.slice_mut();
        let mut carry = false;

        while let Some(loc) = slice.get_mut(idx) {
            let (new, new_carry) = loc.overflowing_add(val);
            carry = new_carry;
            *loc = new;
            idx += 1;

            if !carry {
                break;
            } else {
                val = I::one();
            }
        }

        carry
    }

    /// Multiply two slices, implemented as shift-and-add with overflow check
    pub fn mul_long_element_overflowing<T>(mut left: BitSlice<S, I>, right: BitSlice<T, I>) -> (BitSlice<S, I>, bool)
    where
        T: AsRef<[I]>,
    {
        let zero = I::zero();

        let overflow = right.slice()
            .iter()
            .enumerate()
            .rev()
            .fold(false, |overflow, (idx, &l)| {
                // From the top to bottom, add N shifted copies of M. This can be done by taking each
                // element of the left and doing a widening mul, carrying the upper, and repeating
                let mut new_overflow = false;
                let mut carry = zero;

                for (offset, &r) in right.slice().iter().enumerate() {
                    let (low, high) = I::widening_mul(l, r, carry);
                    carry = high;
                    if left.add_item(idx + offset, low) {
                        new_overflow = true;
                    }
                }

                if carry != zero && left.add_item(idx + right.slice().len(), carry) {
                    new_overflow = true;
                }

                if new_overflow || overflow {
                    true
                } else {
                    false
                }
            });

        (left, overflow)
    }

    /// Multiply two slices, implemented as checked shift-and-add
    pub fn mul_long_element_checked<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> Option<BitSlice<S, I>>
    where
        T: AsRef<[I]>,
    {
        let (out, carry) = BitSlice::mul_long_element_overflowing(left, right);
        if carry {
            None
        } else {
            Some(out)
        }
    }

    /// Multiply two slices, implemented as wrapping shift-and-add
    pub fn mul_long_element_wrapping<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> BitSlice<S, I>
    where
        T: AsRef<[I]>,
    {
        BitSlice::mul_long_element_overflowing(left, right).0
    }
}
