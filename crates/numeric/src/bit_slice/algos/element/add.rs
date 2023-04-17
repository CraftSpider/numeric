use crate::bit_slice::{BitSlice, IsBit};
use crate::utils::shrink_vec;
use crate::bit_slice::algos::OwnedSlice;

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]>,
    I: IsBit,
{
    /// Add two slices, implemented as element-wise add and carry
    pub fn add_element<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> OwnedSlice<I>
    where
        T: AsRef<[I]>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = I::zero();
        let one = I::one();
        let mut out = BitSlice::new(vec![zero; len + 1]);

        let mut carry = false;

        for idx in 0..(len + 1) {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

            let extra = if carry {
                carry = false;
                one
            } else {
                zero
            };

            let (res, new_carry) = l.overflowing_add(r);
            if new_carry {
                carry = true;
            }

            let (res, new_carry) = res.overflowing_add(extra);
            if new_carry {
                carry = true;
            }

            out.set_ignore(idx, res);
        }

        BitSlice::new(shrink_vec(out.into_inner()))
    }
}

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]> + AsMut<[I]>,
    I: IsBit,
{
    /// Add two slices, implemented as wrapping element-wise add and carry with overflow check
    pub fn add_element_overflowing<T>(mut left: BitSlice<S, I>, right: BitSlice<T, I>) -> (BitSlice<S, I>, bool)
    where
        T: AsRef<[I]>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = I::zero();
        let one = I::one();

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

            let (res, new_carry) = l.overflowing_add(r);
            if new_carry {
                carry = true;
            }

            let (res, new_carry) = res.overflowing_add(extra);
            if new_carry {
                carry = true;
            }

            left.set_ignore(idx, res);
        }

        (left, carry)
    }

    /// Add two slices, implemented as checked element-wise add and carry
    pub fn add_element_checked<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> Option<BitSlice<S, I>>
    where
        T: AsRef<[I]>,
    {
        let (out, carry) = BitSlice::add_element_overflowing(left, right);
        if carry {
            None
        } else {
            Some(out)
        }
    }

    /// Add two slices, implemented as wrapping element-wise add and carry
    pub fn add_element_wrapping<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> BitSlice<S, I>
    where
        T: AsRef<[I]>,
    {
        BitSlice::add_element_overflowing(left, right).0
    }
}
