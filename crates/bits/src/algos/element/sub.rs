use crate::algos::element::ElementNot;
use crate::algos::{AssignBinAlg, BinAlg, Element, Operation, Sub};
use crate::bit_slice::BitSliceExt;
use crate::utils::IntSlice;
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::overflowing::OverflowingSub;

impl BinAlg<Sub> for Element {
    #[cfg(feature = "alloc")]
    fn growing<L, R>(left: &L, right: &R) -> (Vec<L::Bit>, bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();
        let one = L::Bit::one();
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

    fn overflowing<'a, L, R>(
        left: &L,
        right: &R,
        out: <Sub as Operation>::Out<&'a mut [L::Bit]>,
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

        (out, carry)
    }

    fn saturating<'a, L, R>(
        left: &L,
        right: &R,
        out: <Sub as Operation>::Out<&'a mut [L::Bit]>,
    ) -> &'a [L::Bit]
    where
        L: BitSliceExt,
        R: BitSliceExt<Bit = L::Bit>,
    {
        todo!()
    }
}

impl AssignBinAlg<Sub> for Element {
    fn overflowing<L, R>(left: &mut L, right: &R, extra: ()) -> bool
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

        carry
    }

    fn saturating<L, R>(left: &L, right: &R, out: ())
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
        assert_eq!(
            <Element as BinAlg<Sub>>::growing(&[0u32], &[0]),
            (vec![0], false),
        );

        assert_eq!(
            <Element as BinAlg<Sub>>::growing(&[1u32], &[0]),
            (vec![1], false),
        );

        assert_eq!(
            <Element as BinAlg<Sub>>::growing(&[0u32], &[1]),
            (vec![1], true),
        );

        assert_eq!(
            <Element as BinAlg<Sub>>::growing(&[1u32], &[1]),
            (vec![0], false),
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_carry() {
        assert_eq!(
            <Element as BinAlg<Sub>>::growing(&[0u32, 1], &[1]),
            (vec![u32::MAX], false),
        )
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_long() {
        assert_eq!(
            <Element as BinAlg<Sub>>::growing(&[0u32, 1], &[0, 1]),
            (vec![0], false),
        );
        assert_eq!(
            <Element as BinAlg<Sub>>::growing(&[1u32, 1], &[1]),
            (vec![0, 1], false),
        );
        assert_eq!(
            <Element as BinAlg<Sub>>::growing(&[1u32, 1], &[0, 1]),
            (vec![1], false),
        );
    }
}
