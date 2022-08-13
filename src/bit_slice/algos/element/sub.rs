use num_traits::ops::overflowing::OverflowingSub;
use num_traits::PrimInt;

use crate::bit_slice::BitSlice;
use crate::utils::shrink_vec;
use crate::bit_slice::algos::OwnedSlice;

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]>,
    I: PrimInt + OverflowingSub,
{
    /// Subtract two slices, implemented as element-wise subtract and borrow
    pub fn sub_element<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> (OwnedSlice<I>, bool)
    where
        T: AsRef<[I]>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = I::zero();
        let one = I::one();
        let mut out = BitSlice::new(vec![zero; len]);

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

            let (res, new_carry) = l.overflowing_sub(&r);
            if new_carry {
                carry = true;
            }

            let (res, new_carry) = res.overflowing_sub(&extra);
            if new_carry {
                carry = true;
            }

            out.set_ignore(idx, res);
        }

        if carry {
            out.set_bit(0, !out.get_bit(0));
            out = -out;
        }

        (BitSlice::new(shrink_vec(out.into_inner())), carry)
    }
}

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]> + AsMut<[I]>,
    I: PrimInt + OverflowingSub,
{
    /// Subtract two slices, implemented as wrapping element-wise subtract and borrow with overflow
    /// check
    pub fn sub_element_overflowing<T>(mut left: BitSlice<S, I>, right: BitSlice<T, I>) -> (BitSlice<S, I>, bool)
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

            let (res, new_carry) = l.overflowing_sub(&r);
            if new_carry {
                carry = true;
            }

            let (res, new_carry) = res.overflowing_sub(&extra);
            if new_carry {
                carry = true;
            }

            left.set_ignore(idx, res);
        }

        (left, carry)
    }

    /// Subtract two slices, implemented as checked element-wise subtract and borrow
    pub fn sub_element_checked<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> Option<BitSlice<S, I>>
    where
        T: AsRef<[I]>,
    {
        let (out, carry) = BitSlice::sub_element_overflowing(left, right);
        if carry {
            None
        } else {
            Some(out)
        }
    }

    /// Subtract two slices, implemented as wrapping element-wise subtract and borrow
    pub fn sub_element_wrapping<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> BitSlice<S, I>
    where
        T: AsRef<[I]>,
    {
        BitSlice::sub_element_overflowing(left, right).0
    }
}
