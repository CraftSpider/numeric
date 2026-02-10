use crate::bit_slice::BitSliceExt;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use numeric_traits::class::Bounded;

mod impls;

pub trait MulAlgo {
    #[cfg(feature = "alloc")]
    fn long<L, R>(left: &L, right: &R) -> Vec<L::Bit>
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

pub trait AssignMulAlgo {
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
            left.iter_mut().for_each(|l| *l = L::Bit::max_value());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::{Bitwise, Element};

    #[cfg(feature = "alloc")]
    fn test_long<B: MulAlgo>() {
        let slice1: &[u8] = &[0b00000000];
        let slice2 = &[0b00000001];

        assert_eq!(B::long(slice1, slice2), &[0b0]);

        let slice3: &[u8] = &[0b00000001];
        let slice4 = &[0b00000001];

        assert_eq!(B::long(slice3, slice4), &[0b1]);

        let slice5: &[u8] = &[0b00000001];
        let slice6 = &[0b00000010];

        assert_eq!(B::long(slice5, slice6), &[0b10]);

        let slice7: &[u8] = &[0b00000010];
        let slice8 = &[0b00000010];

        assert_eq!(B::long(slice7, slice8), &[0b100]);
    }

    fn test_wrapping_assign<B: AssignMulAlgo>() {
        let slice1: &mut [u8] = &mut [0b00000000];
        let slice2 = &[0b00000001];

        B::wrapping(slice1, slice2);
        assert_eq!(slice1, &[0b0]);

        let slice3: &mut [u8] = &mut [0b00000001];
        let slice4 = &[0b00000001];

        B::wrapping(slice3, slice4);
        assert_eq!(slice3, &[0b1]);

        let slice5: &mut [u8] = &mut [0b00000001];
        let slice6 = &[0b00000010];

        B::wrapping(slice5, slice6);
        assert_eq!(slice5, &[0b10]);

        let slice7: &mut [u8] = &mut [0b00000010];
        let slice8 = &[0b00000010];

        B::wrapping(slice7, slice8);
        assert_eq!(slice7, &[0b100]);
    }

    #[test]
    fn test_element() {
        #[cfg(feature = "alloc")]
        test_long::<Element>();
        test_wrapping_assign::<Element>();
    }

    #[test]
    fn test_bitwise() {
        #[cfg(feature = "alloc")]
        test_long::<Bitwise>();
        // test_wrapping_assign::<Bitwise>();
    }
}
