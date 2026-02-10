use super::super::*;
use crate::bit_slice::BitSliceExt;
use crate::utils::IntSlice;
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::overflowing::OverflowingAdd;

impl BinAlg<Add> for Element {
    #[cfg(feature = "alloc")]
    fn growing<L, R>(left: &L, right: &R) -> Vec<L::Bit>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();
        let one = L::Bit::one();
        let mut out = vec![zero; len + 1];

        let mut carry = false;

        for idx in 0..=len {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

            let (res, new_carry) = l.overflowing_add(if carry { one } else { zero });
            carry = new_carry;

            let (res, new_carry) = res.overflowing_add(r);
            if new_carry {
                carry = true;
            }

            out.set_ignore(idx, res);
        }

        IntSlice::shrink(out)
    }

    fn overflowing<'a, L, R>(
        left: &L,
        right: &R,
        out: <Add as Operation>::Out<&'a mut [L::Bit]>,
    ) -> (&'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();
        let one = L::Bit::one();

        let mut carry = false;

        for idx in 0..len {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

            let (res, new_carry) = l.overflowing_add(if carry { one } else { zero });
            carry = new_carry;

            let (res, new_carry) = res.overflowing_add(r);
            // As of Rust 1.86 nightly, this is faster than `carry |= new_carry`
            if new_carry {
                carry = true;
            }

            out.set_ignore(idx, res);
        }

        (out, carry)
    }

    fn saturating<'a, L, R>(
        left: &L,
        right: &R,
        out: <Add as Operation>::Out<&'a mut [L::Bit]>,
    ) -> &'a [L::Bit]
    where
        L: BitSliceExt,
        R: BitSliceExt<Bit = L::Bit>,
    {
        todo!()
    }
}

impl AssignBinAlg<Add> for Element {
    fn overflowing<L, R>(left: &mut L, right: &R, _: ()) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();
        let one = L::Bit::one();

        let mut carry = false;

        for idx in 0..len {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

            let (res, new_carry) = l.overflowing_add(if carry { one } else { zero });
            carry = new_carry;

            let (res, new_carry) = res.overflowing_add(r);
            // As of Rust 1.86 nightly, this is faster than `carry |= new_carry`
            if new_carry {
                carry = true;
            }

            left.set_ignore(idx, res);
        }

        carry
    }

    fn saturating<L, R>(left: &L, right: &R, _: ())
    where
        L: BitSliceExt,
        R: BitSliceExt<Bit = L::Bit>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_simple() {
        assert_eq!(<Element as BinAlg<Add>>::growing(&[0u32], &[0]), &[0],);

        assert_eq!(<Element as BinAlg<Add>>::growing(&[0u32], &[1]), &[1],);

        assert_eq!(<Element as BinAlg<Add>>::growing(&[1u32], &[0]), &[1],);

        assert_eq!(<Element as BinAlg<Add>>::growing(&[1u32], &[1]), &[2],);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_long() {
        assert_eq!(<Element as BinAlg<Add>>::growing(&[0u32], &[0, 1]), &[0, 1],);

        assert_eq!(<Element as BinAlg<Add>>::growing(&[1u32], &[0, 1]), &[1, 1],);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_carry() {
        assert_eq!(
            <Element as BinAlg<Add>>::growing(&[u32::MAX], &[1]),
            &[0, 1],
        );
    }
}
