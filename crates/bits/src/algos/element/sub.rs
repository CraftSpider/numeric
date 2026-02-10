use crate::algos::element::ElementNot;
use crate::bit_slice::BitSliceExt;
use crate::utils::IntSlice;
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::overflowing::OverflowingSub;

pub trait ElementSub: BitSliceExt {
    #[cfg(feature = "alloc")]
    /// Subtract two slices, implemented as element-wise subtract and borrow
    fn sub<T>(left: &Self, right: &T) -> (Vec<Self::Bit>, bool)
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = Self::Bit::zero();
        let one = Self::Bit::one();
        let mut out = vec![zero; len];

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

            let (res, new_carry) = l.overflowing_sub(r);
            if new_carry {
                carry = true;
            }

            let (res, new_carry) = res.overflowing_sub(extra);
            if new_carry {
                carry = true;
            }

            out.set_ignore(idx, res);
        }

        if carry {
            out.set_bit(0, !out.get_bit(0));
            ElementNot::not(&mut out);
        }

        (IntSlice::shrink(out), carry)
    }

    /// Subtract two slices, implemented as wrapping element-wise subtract and borrow with overflow
    /// check
    fn sub_overflowing<'a, T>(left: &'a mut Self, right: &T) -> (&'a mut Self, bool)
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
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

            let (res, new_carry) = l.overflowing_sub(r);
            if new_carry {
                carry = true;
            }

            let (res, new_carry) = res.overflowing_sub(extra);
            if new_carry {
                carry = true;
            }

            left.set_ignore(idx, res);
        }

        (left, carry)
    }

    /// Subtract two slices, implemented as checked element-wise subtract and borrow
    fn sub_checked<'a, T>(left: &'a mut Self, right: &T) -> Option<&'a mut Self>
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let (out, carry) = ElementSub::sub_overflowing(left, right);
        if carry {
            None
        } else {
            Some(out)
        }
    }

    /// Subtract two slices, implemented as wrapping element-wise subtract and borrow
    fn sub_wrapping<'a, T>(left: &'a mut Self, right: &T) -> &'a mut Self
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        ElementSub::sub_overflowing(left, right).0
    }
}

impl<T> ElementSub for T where T: ?Sized + BitSliceExt {}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_simple() {
        assert_eq!(ElementSub::sub(&[0u32], &[0]), (vec![0], false),);

        assert_eq!(ElementSub::sub(&[1u32], &[0]), (vec![1], false),);

        assert_eq!(ElementSub::sub(&[0u32], &[1]), (vec![1], true),);

        assert_eq!(ElementSub::sub(&[1u32], &[1]), (vec![0], false),);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_carry() {
        assert_eq!(ElementSub::sub(&[0u32, 1], &[1]), (vec![u32::MAX], false),)
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_long() {
        assert_eq!(ElementSub::sub(&[0u32, 1], &[0, 1]), (vec![0], false),);
        assert_eq!(ElementSub::sub(&[1u32, 1], &[1]), (vec![0, 1], false),);
        assert_eq!(ElementSub::sub(&[1u32, 1], &[0, 1]), (vec![1], false),);
    }
}
