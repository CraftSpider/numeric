use crate::bit_slice::BitSliceExt;
use numeric_traits::class::Bounded;

mod impls;

pub trait AddAlgo {
    #[cfg(feature = "std")]
    fn long<L, R>(left: &L, right: &R) -> alloc::vec::Vec<L::Bit>
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
        let (val, overflow) = Self::overflowing(left, right, out);
        if overflow {
            out.fill(L::Bit::max_value());
            out
        } else {
            // SAFETY: Polonius case
            unsafe { core::mem::transmute::<&[_], &[_]>(val) }
        }
    }
}

pub trait AssignAddAlgo {
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
            left.slice_mut().fill(L::Bit::max_value());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::{Bitwise, Element};

    #[cfg(feature = "std")]
    fn test_long<B: AddAlgo>() {
        // Simple addition
        assert_eq!(B::long(&[0u32], &[0]), &[0]);
        assert_eq!(B::long(&[0u32], &[1]), &[1]);
        assert_eq!(B::long(&[1u32], &[0]), &[1]);
        assert_eq!(B::long(&[1u32], &[1]), &[2]);

        // Long addition handled correctly
        assert_eq!(B::long(&[0u32], &[0, 1]), &[0, 1]);
        assert_eq!(B::long(&[1u32], &[0, 1]), &[1, 1]);
        assert_eq!(B::long(&[u32::MAX], &[1]), &[0, 1]);
    }

    #[test]
    fn test_element() {
        #[cfg(feature = "std")]
        test_long::<Element>();
    }

    #[test]
    fn test_bitwise() {
        #[cfg(feature = "std")]
        test_long::<Bitwise>();
    }
}
