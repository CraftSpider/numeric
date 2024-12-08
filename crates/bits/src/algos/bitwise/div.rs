use crate::algos::{ElementCmp, ElementShl, ElementSub};
use crate::bit_slice::BitSliceExt;
#[cfg(feature = "std")]
use alloc::{vec, vec::Vec};
use numeric_traits::identity::Zero;

pub trait BitwiseDiv: BitSliceExt {
    #[cfg(feature = "std")]
    /// Divide two slices, implemented as bitwise long division
    fn div_long<T>(num: &Self, div: &T) -> (Vec<Self::Bit>, Vec<Self::Bit>)
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let len = usize::max(num.len(), div.len());
        let bit_len = usize::max(num.bit_len(), div.bit_len());

        let mut quotient = vec![Self::Bit::zero(); len];
        let mut remainder = vec![Self::Bit::zero(); len];

        for idx in (0..bit_len).rev() {
            ElementShl::shl_wrapping(&mut remainder, 1);
            remainder.set_bit(0, num.get_bit(idx));
            if ElementCmp::cmp(&remainder, div).is_ge() {
                // Subtract will never overflow
                ElementSub::sub_wrapping(&mut remainder, div);
                quotient.set_bit(idx, true);
            }
        }

        (quotient, remainder)
    }

    /// Divide two slices, implemented as long division with overflow check
    fn div_long_overflowing<'a, T>(
        num: &'a mut Self,
        div: &T,
        remainder: &mut [Self::Bit],
    ) -> (&'a mut Self, bool)
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let bit_len = usize::max(num.bit_len(), div.bit_len());
        for idx in (0..bit_len).rev() {
            ElementShl::shl_wrapping(remainder, 1);
            remainder.set_bit(0, num.get_bit(idx));
            if ElementCmp::cmp(remainder, div).is_ge() {
                // Subtract will never overflow
                ElementSub::sub_wrapping(remainder, div);
                num.set_bit(idx, true);
            } else {
                num.set_bit(idx, false);
            }
        }

        (num, false)
    }

    /// divide two slices, implemented as checked long division
    fn div_long_checked<'a, T>(
        left: &'a mut Self,
        right: &T,
        scratch: &mut [Self::Bit],
    ) -> Option<&'a mut Self>
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let (out, carry) = BitwiseDiv::div_long_overflowing(left, right, scratch);
        if carry {
            None
        } else {
            Some(out)
        }
    }

    /// Divide two slices, implemented as wrapping long division
    fn div_long_wrapping<'a, T>(
        left: &'a mut Self,
        right: &T,
        scratch: &mut [Self::Bit],
    ) -> &'a mut Self
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        BitwiseDiv::div_long_overflowing(left, right, scratch).0
    }

    /// Divide two slices, implemented as long division with overflow check
    fn rem_long_overflowing<'a, T>(
        num: &'a mut Self,
        div: &T,
        remainder: &mut [Self::Bit],
    ) -> (&'a mut Self, bool)
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let bit_len = usize::max(num.bit_len(), div.bit_len());

        for idx in (0..bit_len).rev() {
            ElementShl::shl_wrapping(remainder, 1);
            remainder.set_bit(0, num.get_bit(idx));
            if ElementCmp::cmp(remainder, div).is_ge() {
                // Subtract will never overflow
                ElementSub::sub_wrapping(remainder, div);
            }
        }
        num.slice_mut().copy_from_slice(remainder);

        (num, false)
    }

    /// divide two slices, implemented as checked long division
    fn rem_long_checked<'a, T>(
        left: &'a mut Self,
        right: &T,
        scratch: &mut [Self::Bit],
    ) -> Option<&'a mut Self>
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let (out, carry) = BitwiseDiv::rem_long_overflowing(left, right, scratch);
        if carry {
            None
        } else {
            Some(out)
        }
    }

    /// Divide two slices, implemented as wrapping long division
    fn rem_long_wrapping<'a, T>(
        left: &'a mut Self,
        right: &T,
        scratch: &mut [Self::Bit],
    ) -> &'a mut Self
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        BitwiseDiv::rem_long_overflowing(left, right, scratch).0
    }
}

impl<T> BitwiseDiv for T where T: ?Sized + BitSliceExt {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div() {
        let slice1: &[u8] = &[0b10];
        let slice2: &[u8] = &[0b01];

        assert_eq!(BitwiseDiv::div_long(slice1, slice2).0, &[0b10]);

        let slice3: &[u8] = &[0b10];
        let slice4: &[u8] = &[0b10];

        assert_eq!(BitwiseDiv::div_long(slice3, slice4).0, &[0b01]);

        let slice5: &[u8] = &[0b00000000, 0b1];
        let slice6: &[u8] = &[0b00000010];

        assert_eq!(BitwiseDiv::div_long(slice5, slice6).0, &[0b10000000, 0b0]);

        let slice7: &[u8] = &[0b0, 0b0, 0b0, 0b1];
        let slice8: &[u8] = &[0b10];

        assert_eq!(
            BitwiseDiv::div_long(slice7, slice8).0,
            &[0b0, 0b0, 0b10000000, 0b0]
        );
    }

    #[test]
    fn test_rem() {
        for i in 0..4 {
            let slice1: &[u8] = &[i];
            let slice2 = &[0b10];

            assert_eq!(BitwiseDiv::div_long(slice1, slice2).1, &[i % 2]);
        }

        for i in 0..6 {
            let slice3: &[u8] = &[i];
            let slice4 = &[0b11];

            assert_eq!(BitwiseDiv::div_long(slice3, slice4).1, &[i % 3]);
        }

        let slice5: &[u8] = &[0b00000001, 0b111];
        let slice6 = &[0b00000010];

        assert_eq!(BitwiseDiv::div_long(slice5, slice6).1, &[0b01, 0b0]);
    }

    #[test]
    fn test_div_wrapping() {
        let mut data = [0b10u8];
        let slice2: &[u8] = &[0b01];

        assert_eq!(
            BitwiseDiv::div_long_wrapping(&mut data, slice2, &mut [0; 1]),
            &[0b10]
        );

        let mut data = [0b10u8];
        let slice4: &[u8] = &[0b10];

        assert_eq!(
            BitwiseDiv::div_long_wrapping(&mut data, slice4, &mut [0]),
            &[0b01]
        );

        let mut data = [0b00000000u8, 0b1];
        let slice6: &[u8] = &[0b00000010];

        assert_eq!(
            BitwiseDiv::div_long_wrapping(&mut data, slice6, &mut [0; 2]),
            &[0b10000000, 0b0]
        );

        let mut data = [0b0u8, 0b0, 0b0, 0b1];
        let slice8: &[u8] = &[0b10];

        assert_eq!(
            BitwiseDiv::div_long_wrapping(&mut data, slice8, &mut [0; 4]),
            &[0b0, 0b0, 0b10000000, 0b0]
        );
    }
}
