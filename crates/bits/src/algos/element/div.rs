use crate::algos::{ElementAdd, ElementCmp, ElementShl, ElementShr, ElementSub};
use crate::bit_slice::{BitLike, BitSliceExt};
use crate::utils::IntSlice;
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use numeric_traits::identity::{One, Zero};

pub trait ElementDiv: BitSliceExt {
    #[cfg(feature = "alloc")]
    /// Divide two slices, implemented as long division.
    ///
    /// <div class="warning">
    /// This method is, unlike most element methods, *much slower* than the equivalent Bitwise
    /// method. Prefer `BitwiseDiv::div_long` for non-academic use-cases.
    /// </div>
    fn div_long<T>(left: &Self, right: &T) -> (Vec<Self::Bit>, Vec<Self::Bit>)
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let len = usize::max(left.len(), right.len());

        let mut quotient = vec![Self::Bit::zero(); len];
        let mut remainder = vec![Self::Bit::zero(); len];
        let mut one = vec![Self::Bit::zero(); len];
        one[len - 1] = Self::Bit::one();

        for idx in (0..len).rev() {
            // Shift left by 1 element
            ElementShl::shl_wrapping(&mut remainder, Self::Bit::BIT_LEN);
            // Set new element to left[idx]
            remainder.set(0, left.get(idx));

            // Mathy stuff:
            //   The remainder will *always* be no more than one digit greater than the divisor
            //   Which means the divisor will go into the remainder at most Self::Bit::MAX times
            //
            //   (99 / 100, remainder of 99, is the worst case)

            // TODO: This is the slow bit. Maybe do some mul/sub stuff instead?
            //       Can this not be a loop, maybe nested division or something?
            //       Remainder is at most Bit::MAX * right. Can we use that?
            // While remainder is greater than divisor
            while ElementCmp::cmp(&remainder, right).is_ge() {
                // Subtract remainder by divisor
                ElementSub::sub_wrapping(&mut remainder, right);
                // Add 1 to quotient at idx
                ElementAdd::add_wrapping(&mut quotient, &one);
            }

            ElementShr::shr_wrapping(&mut one, Self::Bit::BIT_LEN);
        }

        (IntSlice::shrink(quotient), IntSlice::shrink(remainder))
    }

    /// Divide two slices, implemented as long division with overflow check
    fn div_long_overflowing<'a, T>(_left: &'a mut Self, _right: &T) -> (&'a mut Self, bool)
    where
        T: ?Sized + BitSliceExt,
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

    /// divide two slices, implemented as checked long division
    fn div_long_checked<'a, T>(left: &'a mut Self, right: &T) -> Option<&'a mut Self>
    where
        T: ?Sized + BitSliceExt,
    {
        let (out, carry) = ElementDiv::div_long_overflowing(left, right);
        if carry {
            None
        } else {
            Some(out)
        }
    }

    /// Divide two slices, implemented as wrapping long division
    fn div_long_wrapping<'a, T>(left: &'a mut Self, right: &T) -> &'a mut Self
    where
        T: ?Sized + BitSliceExt,
    {
        ElementDiv::div_long_overflowing(left, right).0
    }
}

impl<T> ElementDiv for T where T: ?Sized + BitSliceExt {}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_rem() {
        assert_eq!(ElementDiv::div_long(&[10u32], &[2]).1, &[0],);
        assert_eq!(ElementDiv::div_long(&[17u32], &[11]).1, &[6],);
        // TODO: Unworkably slow
        // assert_eq!(ElementDiv::div_long(&[0u32, 1], &[7]).1, &[2],);
    }

    // #[test]
    // fn test_div() {
    //     let slice1: &[u8] = &[0b10];
    //     let slice2: &[u8] = &[0b01];
    //
    //     assert_eq!(ElementDiv::div_long(slice1, slice2).0, &[0b10]);
    //
    //     let slice3: &[u8] = &[0b10];
    //     let slice4: &[u8] = &[0b10];
    //
    //     assert_eq!(ElementDiv::div_long(slice3, slice4).0, &[0b01]);
    //
    //     let slice5: &[u8] = &[0b00000000, 0b1];
    //     let slice6: &[u8] = &[0b00000010];
    //
    //     assert_eq!(ElementDiv::div_long(slice5, slice6).0, &[0b10000000]);
    //
    //     let slice7: &[u8] = &[0b0, 0b0, 0b0, 0b1];
    //     let slice8: &[u8] = &[0b10];
    //
    //     assert_eq!(
    //         ElementDiv::div_long(slice7, slice8).0,
    //         &[0b0, 0b0, 0b10000000]
    //     );
    // }
}
