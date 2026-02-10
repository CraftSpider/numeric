use crate::bit_slice::BitSliceExt;
use crate::utils::IntSlice;
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::overflowing::OverflowingAdd;
use numeric_traits::ops::widening::WideningMul;

pub trait ElementMul: BitSliceExt {
    #[cfg(feature = "alloc")]
    /// Multiply two slices, implemented as shift-and-add
    fn mul<T>(left: &Self, right: &T) -> Vec<Self::Bit>
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let zero = Self::Bit::zero();
        let mut out = vec![zero; left.len() + right.len()];

        left.slice().iter().enumerate().for_each(|(idx, &l)| {
            let mut carry = zero;

            for (offset, &r) in right.slice().iter().enumerate() {
                let (low, high) = Self::Bit::widening_mul(l, r, carry);
                carry = high;
                out.add_item(idx + offset, low);
            }

            if carry != zero {
                out.add_item(idx + right.slice().len(), carry);
            }
        });

        IntSlice::shrink(out)
    }

    fn add_item(&mut self, mut idx: usize, mut val: Self::Bit) -> bool {
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
                val = Self::Bit::one();
            }
        }

        carry
    }

    /// Multiply two slices, implemented as shift-and-add with overflow check
    fn mul_overflowing<'a, T>(left: &'a mut Self, right: &T) -> (&'a mut Self, bool)
    where
        T: BitSliceExt<Bit = Self::Bit>,
    {
        let zero = Self::Bit::zero();

        let mut overflow = false;
        for idx in (0..left.len()).rev() {
            // From the top to bottom, add N shifted copies of M. This can be done by taking each
            // element of the left and doing a widening mul, carrying the upper, and repeating
            let mut new_overflow = false;
            let mut carry = zero;

            let l_mut = left.slice_mut();
            let l = l_mut[idx];
            l_mut[idx] = zero;

            for (offset, &r) in right.slice().iter().enumerate() {
                let (low, high) = Self::Bit::widening_mul(l, r, carry);
                carry = high;
                if left.add_item(idx + offset, low) {
                    new_overflow = true;
                }
            }

            if carry != zero && left.add_item(idx + right.slice().len(), carry) {
                new_overflow = true;
            }

            overflow |= new_overflow;
        }

        (left, overflow)
    }

    /// Multiply two slices, implemented as checked shift-and-add
    fn mul_checked<'a, T>(left: &'a mut Self, right: &T) -> Option<&'a mut Self>
    where
        T: BitSliceExt<Bit = Self::Bit>,
    {
        let (out, carry) = ElementMul::mul_overflowing(left, right);
        if carry {
            None
        } else {
            Some(out)
        }
    }

    /// Multiply two slices, implemented as wrapping shift-and-add
    fn mul_wrapping<'a, T>(left: &'a mut Self, right: &T) -> &'a mut Self
    where
        T: BitSliceExt<Bit = Self::Bit>,
    {
        ElementMul::mul_overflowing(left, right).0
    }
}

impl<T> ElementMul for T where T: ?Sized + BitSliceExt {}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_mul() {
        let slice1: &[u8] = &[0b00000000];
        let slice2 = &[0b00000001];

        assert_eq!(ElementMul::mul(slice1, slice2), &[0b0]);

        let slice3: &[u8] = &[0b00000001];
        let slice4 = &[0b00000001];

        assert_eq!(ElementMul::mul(slice3, slice4), &[0b1]);

        let slice5: &[u8] = &[0b00000001];
        let slice6 = &[0b00000010];

        assert_eq!(ElementMul::mul(slice5, slice6), &[0b10]);

        let slice7: &[u8] = &[0b00000010];
        let slice8 = &[0b00000010];

        assert_eq!(ElementMul::mul(slice7, slice8), &[0b100]);
    }

    #[test]
    fn test_mul_wrapping() {
        let slice1: &mut [u8] = &mut [0b00000000];
        let slice2 = &[0b00000001];

        assert_eq!(ElementMul::mul_wrapping(slice1, slice2), &[0b0]);

        let slice3: &mut [u8] = &mut [0b00000001];
        let slice4 = &[0b00000001];

        assert_eq!(ElementMul::mul_wrapping(slice3, slice4), &[0b1]);

        let slice5: &mut [u8] = &mut [0b00000001];
        let slice6 = &[0b00000010];

        assert_eq!(ElementMul::mul_wrapping(slice5, slice6), &[0b10]);

        let slice7: &mut [u8] = &mut [0b00000010];
        let slice8 = &[0b00000010];

        assert_eq!(ElementMul::mul_wrapping(slice7, slice8), &[0b100]);
    }
}
