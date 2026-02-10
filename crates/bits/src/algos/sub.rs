use crate::bit_slice::BitSliceExt;
use numeric_traits::class::Bounded;

mod impls;

pub trait SubAlgo {
    #[cfg(feature = "std")]
    fn long<L, R>(left: &L, right: &R) -> (alloc::vec::Vec<L::Bit>, bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn overflowing<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> (&'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn wrapping<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        Self::overflowing(left, right, out).0
    }

    fn checked<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> Option<&'a [L::Bit]>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let (out, overflow) = Self::overflowing(left, right, out);
        if overflow {
            None
        } else {
            Some(out)
        }
    }

    fn saturating<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: BitSliceExt,
        R: BitSliceExt<Bit = L::Bit>,
    {
        {
            let (val, overflow) = Self::overflowing(left, right, out);
            if overflow {
                out.fill(L::Bit::min_value());
                out
            } else {
                // SAFETY: Polonius case
                unsafe { core::mem::transmute::<&[_], &[_]>(val) }
            }
        }
    }
}

pub trait AssignSubAlgo {
    fn overflowing<L, R>(left: &mut L, right: &R) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn wrapping<L, R>(left: &mut L, right: &R)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        Self::overflowing(left, right);
    }

    fn checked<L, R>(left: &mut L, right: &R) -> Option<()>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        if Self::overflowing(left, right) {
            None
        } else {
            Some(())
        }
    }

    fn saturating<L, R>(left: &mut L, right: &R)
    where
        L: BitSliceExt,
        R: BitSliceExt<Bit = L::Bit>,
    {
        let overflow = Self::overflowing(left, right);
        if overflow {
            left.slice_mut().fill(L::Bit::min_value());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::{Bitwise, Element};
    use alloc::vec;

    fn test_long<B: SubAlgo>() {
        // Simple subtraction
        assert_eq!(B::long(&[0u32], &[0]), (vec![0], false));
        assert_eq!(B::long(&[1u32], &[0]), (vec![1], false));
        assert_eq!(B::long(&[0u32], &[1]), (vec![1], true));
        assert_eq!(B::long(&[1u32], &[1]), (vec![0], false));

        // Long subtraction handled correctly
        assert_eq!(B::long(&[0u32, 1], &[0, 1]), (vec![0], false));
        assert_eq!(B::long(&[1u32, 1], &[1]), (vec![0, 1], false));
        assert_eq!(B::long(&[1u32, 1], &[0, 1]), (vec![1], false));
        assert_eq!(B::long(&[0u32, 1], &[1]), (vec![u32::MAX], false));
    }

    #[test]
    fn test_element() {
        test_long::<Element>();
    }

    #[test]
    fn test_bitwise() {
        test_long::<Bitwise>();
    }
}
