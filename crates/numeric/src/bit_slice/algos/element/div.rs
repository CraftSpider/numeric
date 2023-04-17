use crate::bit_slice::BitSlice;
use crate::utils::{int_to_arr, shrink_vec};
use crate::bit_slice::algos::OwnedSlice;
use crate::bit_slice::sealed::IsBit;
use crate::traits::WideningMul;

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]>,
    I: IsBit,
{
    /// Divide two slices, implemented as long division
    pub fn div_long_element<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> (OwnedSlice<I>, OwnedSlice<I>)
    where
        T: AsRef<[I]>,
    {
        let len = usize::max(left.len(), right.len());

        let mut quotient = BitSlice::new(vec![I::zero(); len]);
        let mut remainder: BitSlice<_, _> = BitSlice::new(vec![I::zero(); len]);

        for idx in (0..len).rev() {
            todo!();
            // Shift left by 1 element
            // Set new element to left[idx]
            // If remainder is now greater than divisor
            if remainder >= right.as_slice() {
                // Subtract remainder by divisor
                // Add remainder to quotient at idx
            }
        }

        (quotient, remainder)
    }
}

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]> + AsMut<[I]> + Clone,
    I: IsBit + WideningMul,
{
    /// Divide two slices, implemented as long division with overflow check
    pub fn div_long_element_overflowing<T>(mut left: BitSlice<S, I>, right: BitSlice<T, I>) -> (BitSlice<S, I>, bool)
    where
        T: AsRef<[I]> + Clone,
    {
        todo!()
    }

    /*
        // Make sure left * right <= bits
        let k = usize::max(left.bit_len(), right.bit_len());
        println!("k: {k}");
        let pow = 2usize.pow((k+1) as u32);
        let bits = BitSlice::new(int_to_arr(pow));
        println!("bits: {bits:?}");

        // Get an initial guess. For now this can be whatever
        // This should be able to fit at least `bits`
        let mut x = left.clone();
        println!("x: {x:?}");

        for _ in 0..k {
            // x+1 = x * (2^(k+1) - x * B) >> k
            let b = right.as_slice();
            println!("b: {b:?}");
            let bx = BitSlice::mul_long_element_wrapping(x.clone(), b);
            println!("bx: {bx:?}");
            let sub = BitSlice::sub_element_wrapping(bits.clone(), bx);
            println!("sub: {sub:?}");
            x = BitSlice::shr_wrap_and_mask_wrapping(BitSlice::mul_long_element_wrapping(x, sub), k);
            println!("x: {x:?}");
        }

        (BitSlice::shr_wrap_and_mask_wrapping(BitSlice::mul_long_element_wrapping(left, x), k), false)
     */

    /// divtiply two slices, implemented as checked long division
    pub fn div_long_element_checked<T>(left: BitSlice<S, I>, right: BitSlice<T, I>) -> Option<BitSlice<S, I>>
    where
        T: AsRef<[I]> + Clone,
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
        T: AsRef<[I]> + Clone,
    {
        BitSlice::div_long_element_overflowing(left, right).0
    }
}
