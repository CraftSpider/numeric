use crate::bit_slice::BitSliceExt;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use numeric_traits::identity::Zero;

mod impls;

pub trait ShlAlgo {
    #[cfg(feature = "alloc")]
    fn long<L>(left: &L, right: usize) -> Vec<L::Bit>
    where
        L: ?Sized + BitSliceExt;

    fn overflowing<'a, L>(left: &L, right: usize, out: &'a mut [L::Bit]) -> (&'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt;

    fn wrapping<'a, L>(left: &L, right: usize, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: ?Sized + BitSliceExt,
    {
        Self::overflowing(left, right, out).0
    }

    fn checked<'a, L>(left: &L, right: usize, out: &'a mut [L::Bit]) -> Option<&'a [L::Bit]>
    where
        L: ?Sized + BitSliceExt,
    {
        let (out, overflow) = Self::overflowing(left, right, out);
        if overflow {
            None
        } else {
            Some(out)
        }
    }

    fn saturating<'a, L>(left: &L, right: usize, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: BitSliceExt,
    {
        let (val, overflow) = Self::overflowing(left, right, out);
        if overflow {
            out.fill(L::Bit::zero());
            out
        } else {
            // SAFETY: Polonius case
            unsafe { core::mem::transmute::<&[_], &[_]>(val) }
        }
    }
}

pub trait ShrAlgo {
    #[cfg(feature = "alloc")]
    fn long<L>(left: &L, right: usize) -> Vec<L::Bit>
    where
        L: ?Sized + BitSliceExt;

    fn overflowing<'a, L>(left: &L, right: usize, out: &'a mut [L::Bit]) -> (&'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt;

    fn wrapping<'a, L>(left: &L, right: usize, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: ?Sized + BitSliceExt,
    {
        Self::overflowing(left, right, out).0
    }

    fn checked<'a, L>(left: &L, right: usize, out: &'a mut [L::Bit]) -> Option<&'a [L::Bit]>
    where
        L: ?Sized + BitSliceExt,
    {
        let (out, overflow) = Self::overflowing(left, right, out);
        if overflow {
            None
        } else {
            Some(out)
        }
    }

    fn saturating<'a, L>(left: &L, right: usize, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: BitSliceExt,
    {
        let (val, overflow) = Self::overflowing(left, right, out);
        if overflow {
            out.fill(L::Bit::zero());
            out
        } else {
            // SAFETY: Polonius case
            unsafe { core::mem::transmute::<&[_], &[_]>(val) }
        }
    }
}

pub trait AssignShlAlgo {
    fn overflowing<L>(left: &mut L, right: usize) -> bool
    where
        L: ?Sized + BitSliceExt;

    fn wrapping<L>(left: &mut L, right: usize)
    where
        L: ?Sized + BitSliceExt,
    {
        Self::overflowing(left, right);
    }

    fn checked<L>(left: &mut L, right: usize) -> Option<()>
    where
        L: ?Sized + BitSliceExt,
    {
        Self::overflowing(left, right).then_some(())
    }

    fn saturating<L>(left: &mut L, right: usize)
    where
        L: BitSliceExt,
    {
        let overflow = Self::overflowing(left, right);
        if overflow {
            left.iter_mut().for_each(|l| *l = L::Bit::zero());
        }
    }
}

pub trait AssignShrAlgo {
    fn overflowing<L>(left: &mut L, right: usize) -> bool
    where
        L: ?Sized + BitSliceExt;

    fn wrapping<L>(left: &mut L, right: usize)
    where
        L: ?Sized + BitSliceExt,
    {
        Self::overflowing(left, right);
    }

    fn checked<L>(left: &mut L, right: usize) -> Option<()>
    where
        L: ?Sized + BitSliceExt,
    {
        Self::overflowing(left, right).then_some(())
    }

    fn saturating<L>(left: &mut L, right: usize)
    where
        L: BitSliceExt,
    {
        let overflow = Self::overflowing(left, right);
        if overflow {
            left.iter_mut().for_each(|l| *l = L::Bit::zero());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::{Bitwise, Element};

    #[cfg(feature = "alloc")]
    fn test_shl<B: ShlAlgo>() {
        let slice: &[u16] = &[0b1010_1010_1010_1010, 0b1010_1010_1010_1010];
        assert_eq!(
            B::long(slice, 1),
            &[0b0101_0101_0101_0100, 0b0101_0101_0101_0101, 0b1]
        );

        let slice: &[u8] = &[0b1111_1111];
        assert_eq!(B::long(slice, 8), &[0b0, 0b1111_1111]);

        assert_eq!(B::long(&[0b0000_0000u8], 1), &[0]);
        assert_eq!(B::long(&[0b01u8], 1), &[0b10]);
        assert_eq!(B::long(&[0b0101u8], 1), &[0b1010]);

        let slice = &[0b1010_1010_1010_1010u16, 0b1010_1010_1010_1010];
        let res = B::long(slice, 1);
        assert_eq!(res, &[0b0101_0101_0101_0100, 0b0101_0101_0101_0101, 0b1]);
        assert_eq!(B::long(&[0b1u8], 8), &[0b0, 0b1])
    }

    fn test_shl_wrapping<B: ShlAlgo>() {
        let data = [0u8];
        assert_eq!(B::wrapping(&data, 1, &mut [0]), &[0]);
        let data = [0b01u8];
        assert_eq!(B::wrapping(&data, 1, &mut [0]), &[0b10]);
        let data = [0b0101u8];
        assert_eq!(B::wrapping(&data, 1, &mut [0]), &[0b1010]);

        let data = [0b1010_1010_1010_1010u16, 0b1010_1010_1010_1010];
        assert_eq!(
            B::wrapping(&data, 1, &mut [0; 2]),
            &[0b0101_0101_0101_0100, 0b0101_0101_0101_0101],
        );
        let data = [0b1000_0000u8, 0b1000_0000];
        assert_eq!(B::wrapping(&data, 1, &mut [0; 2]), &[0b0, 0b1]);
        let data = [0b1u8, 0b0];
        assert_eq!(B::wrapping(&data, 8, &mut [0; 2]), &[0b0, 0b1])
    }

    #[cfg(feature = "alloc")]
    fn test_shr<B: ShrAlgo>() {
        let slice: &[u16] = &[0b1010_1010_1010_1010, 0b1010_1010_1010_1010];
        assert_eq!(
            B::long(slice, 1),
            &[0b0101_0101_0101_0101, 0b0101_0101_0101_0101]
        );

        let slice: &[u8] = &[0b0, 0b1111_1111];
        assert_eq!(B::long(slice, 8), &[0b1111_1111]);
    }

    fn test_shr_wrapping<B: ShrAlgo>() {
        let data = [0u8];
        assert_eq!(B::wrapping(&data, 1, &mut [0]), &[0]);
        let data = [0b10u8];
        assert_eq!(B::wrapping(&data, 1, &mut [0]), &[0b01]);
        let data = [0b1010u8];
        assert_eq!(B::wrapping(&data, 1, &mut [0]), &[0b0101]);

        let data = [0b1010_1010_1010_1010u16, 0b1010_1010_1010_1010];
        assert_eq!(
            B::wrapping(&data, 1, &mut [0; 2]),
            &[0b0101_0101_0101_0101, 0b0101_0101_0101_0101],
        );
        let data = [0b0000_0001u8, 0b0000_0001];
        assert_eq!(B::wrapping(&data, 1, &mut [0; 2]), &[0b1000_0000, 0b0]);
        let data = [0b0u8, 0b1];
        assert_eq!(B::wrapping(&data, 8, &mut [0; 2]), &[0b1, 0b0])
    }

    #[test]
    fn test_bitwise() {
        #[cfg(feature = "alloc")]
        test_shl::<Bitwise>();
        #[cfg(feature = "alloc")]
        test_shr::<Bitwise>();

        test_shl_wrapping::<Bitwise>();
        test_shr_wrapping::<Bitwise>();
    }

    #[test]
    fn test_element() {
        #[cfg(feature = "alloc")]
        test_shl::<Element>();
        #[cfg(feature = "alloc")]
        test_shr::<Element>();

        test_shl_wrapping::<Element>();
        test_shr_wrapping::<Element>();
    }
}
