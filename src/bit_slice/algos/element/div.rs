use num_traits::PrimInt;

use crate::bit_slice::BitSlice;
use crate::utils::shrink_vec;
use crate::bit_slice::algos::OwnedSlice;

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]>,
    I: PrimInt,
{
    /// Divide two slices, implemented as long division
    pub fn div_long_element<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> OwnedSlice<I>
    where
        T: AsRef<[I]>,
    {
        // let len = usize::max(left.len(), right.len());
        //
        // let mut quotient = BitSlice::new(vec![I::zero(); len]);
        // let mut remainder: BitSlice<_, _> = BitSlice::new(vec![I::zero(); len]);
        //
        // for idx in (0..len).rev() {
        //     // Shift left by 1 element
        //     // Set new element to left[idx]
        //     // If remainder is now greater than divisor
        //     if remainder >= right {
        //         // Subtract remainder by divisor
        //         // Add remainder to quotient at idx
        //     }
        // }

        todo!()
    }
}

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]> + AsMut<[I]>,
    I: PrimInt,
{
    /// Divide two slices, implemented as long division with overflow check
    pub fn div_long_element_overflowing<T>(mut left: BitSlice<S, I>, right: BitSlice<T, I>) -> (BitSlice<S, I>, bool)
    where
        T: AsRef<[I]>,
    {
        todo!()
    }

    /// divtiply two slices, implemented as checked long division
    pub fn div_long_element_checked<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> Option<BitSlice<S, I>>
    where
        T: AsRef<[I]>,
    {
        let (out, carry) = BitSlice::div_long_element_overflowing(left, right);
        if carry {
            None
        } else {
            Some(out)
        }
    }

    /// Divide two slices, implemented as wrapping long division
    pub fn div_long_element_wrapping<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> BitSlice<S, I>
    where
        T: AsRef<[I]>,
    {
        BitSlice::div_long_element_overflowing(left, right).0
    }
}
