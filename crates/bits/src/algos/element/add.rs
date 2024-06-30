use crate::bit_slice::BitSliceExt;
use crate::utils::IntSlice;
#[cfg(feature = "std")]
use alloc::{vec, vec::Vec};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::overflowing::OverflowingAdd;

pub trait ElementAdd: BitSliceExt {
    #[cfg(feature = "std")]
    /// Add two slices, implemented as element-wise add and carry
    fn add<T>(left: &Self, right: &T) -> Vec<Self::Bit>
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = Self::Bit::zero();
        let one = Self::Bit::one();
        let mut out = vec![zero; len + 1];

        let mut carry = false;

        for idx in 0..=len {
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

        IntSlice::shrink(out)
    }

    /// Add two slices, implemented as wrapping element-wise add and carry with overflow check
    fn add_overflowing<'a, T>(left: &'a mut Self, right: &T) -> (&'a mut Self, bool)
    where
        T: BitSliceExt<Bit = Self::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = Self::Bit::zero();
        let one = Self::Bit::one();

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
    fn add_checked<'a, T>(left: &'a mut Self, right: &T) -> Option<&'a mut Self>
    where
        T: BitSliceExt<Bit = Self::Bit>,
    {
        let (out, carry) = ElementAdd::add_overflowing(left, right);
        if carry {
            None
        } else {
            Some(out)
        }
    }

    /// Add two slices, implemented as wrapping element-wise add and carry
    fn add_wrapping<'a, T>(left: &'a mut Self, right: &T) -> &'a mut Self
    where
        T: BitSliceExt<Bit = Self::Bit>,
    {
        ElementAdd::add_overflowing(left, right).0
    }
}

impl<T: ?Sized + BitSliceExt> ElementAdd for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(ElementAdd::add(&[0u32], &[0]), &[0],);

        assert_eq!(ElementAdd::add(&[0u32], &[1]), &[1],);

        assert_eq!(ElementAdd::add(&[1u32], &[0]), &[1],);

        assert_eq!(ElementAdd::add(&[1u32], &[1]), &[2],);
    }

    #[test]
    fn test_long() {
        assert_eq!(ElementAdd::add(&[0u32], &[0, 1]), &[0, 1],);

        assert_eq!(ElementAdd::add(&[1u32], &[0, 1]), &[1, 1],);
    }

    #[test]
    fn test_carry() {
        assert_eq!(ElementAdd::add(&[u32::MAX], &[1]), &[0, 1],);
    }
}
