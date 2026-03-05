use crate::bit_slice::BitSliceExt;
use numeric_traits::class::Bounded;

mod impls;

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
            left.iter_mut().for_each(|v| *v = L::Bit::max_value());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::{Add, Algo, Bitwise};

    #[cfg(feature = "alloc")]
    fn test_long<B: Algo<Add>>() {
        use alloc::vec::Vec;
        // Simple addition
        assert_eq!(B::wrapping::<_, _, Vec<_>>(&[0u32], &[0]), &[0]);
        assert_eq!(B::wrapping::<_, _, Vec<_>>(&[0u32], &[1]), &[1]);
        assert_eq!(B::wrapping::<_, _, Vec<_>>(&[1u32], &[0]), &[1]);
        assert_eq!(B::wrapping::<_, _, Vec<_>>(&[1u32], &[1]), &[2]);

        // Long addition handled correctly
        assert_eq!(B::wrapping::<_, _, Vec<_>>(&[0u32], &[0, 1]), &[0, 1]);
        assert_eq!(B::wrapping::<_, _, Vec<_>>(&[1u32], &[0, 1]), &[1, 1]);
        assert_eq!(B::wrapping::<_, _, Vec<_>>(&[u32::MAX], &[1]), &[0, 1]);
    }

    fn test_wrapping<B: Algo<Add>>() {
        // Simple addition
        assert_eq!(B::wrapping::<_, _, [_; _]>(&[0u32], &[0]), [0]);
        assert_eq!(B::wrapping::<_, _, [_; _]>(&[0u32], &[1]), [1]);
        assert_eq!(B::wrapping::<_, _, [_; _]>(&[1u32], &[0]), [1]);
        assert_eq!(B::wrapping::<_, _, [_; _]>(&[1u32], &[1]), [2]);

        // Long addition handled correctly
        assert_eq!(B::wrapping::<_, _, [_; _]>(&[0u32], &[0, 1]), [0, 1]);
        assert_eq!(B::wrapping::<_, _, [_; _]>(&[1u32], &[0, 1]), [1, 1]);
        assert_eq!(B::wrapping::<_, _, [_; _]>(&[u32::MAX], &[1]), [0, 1]);
    }

    fn test_saturating<B: Algo<Add>>() {
        assert_eq!(B::saturating::<_, _, [_; _]>(&[0u32], &[0]), [0]);
        assert_eq!(B::saturating::<_, _, [_; _]>(&[1u32], &[1]), [2]);
        assert_eq!(B::saturating::<_, _, [_; _]>(&[1], &[u32::MAX]), [u32::MAX]);
        assert_eq!(B::saturating::<_, _, [_; _]>(&[u32::MAX], &[1]), [u32::MAX]);
        assert_eq!(
            B::saturating::<_, _, [_; _]>(&[u32::MAX], &[u32::MAX]),
            [u32::MAX]
        );
    }

    /// Test some edge cases of the desired AddAlgo API
    /// - Outputs can be non-zero
    /// - Inputs can be of different lengths
    fn test_edges<B: Algo<Add>>() {
        let a = &[1u8, 1];
        let b = &[1];

        assert_eq!(B::wrapping::<_, _, [_; _]>(a, b), [2]);
        assert_eq!(B::wrapping::<_, _, [_; _]>(a, b), [2, 1]);
        assert_eq!(B::wrapping::<_, _, [_; _]>(b, a), [2]);
        assert_eq!(B::wrapping::<_, _, [_; _]>(b, a), [2, 1]);
    }

    #[test]
    fn test_element() {
        // #[cfg(feature = "alloc")]
        // test_long::<Element>();
        // test_wrapping::<Element>();
        // test_saturating::<Element>();
        // test_edges::<Element>();
    }

    #[test]
    fn test_bitwise() {
        #[cfg(feature = "alloc")]
        test_long::<Bitwise>();
        test_wrapping::<Bitwise>();
        test_saturating::<Bitwise>();
        test_edges::<Bitwise>();
    }
}
