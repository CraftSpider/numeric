use crate::algos::{AssignBitAlgo, BitAlgo, Element};
use crate::bit_slice::BitSliceExt;
use numeric_traits::identity::Zero;

impl BitAlgo for Element {
    fn and<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();

        for idx in 0..len {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

            out.set(idx, l & r);
        }

        out
    }

    fn or<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();

        for idx in 0..len {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

            out.set(idx, l | r);
        }

        out
    }

    fn xor<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();

        for idx in 0..len {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

            out.set(idx, l ^ r);
        }

        out
    }

    fn not<'a, L>(left: &L, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: ?Sized + BitSliceExt,
    {
        for idx in 0..left.len() {
            let l = left.get(idx);
            out.set(idx, !l);
        }

        out
    }
}

impl AssignBitAlgo for Element {
    fn and<L, R>(left: &mut L, right: &R)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();

        for idx in 0..len {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

            left.set(idx, l & r);
        }
    }

    fn or<L, R>(left: &mut L, right: &R)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();

        for idx in 0..len {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

            left.set(idx, l | r);
        }
    }

    fn xor<L, R>(left: &mut L, right: &R)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();

        for idx in 0..len {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

            left.set(idx, l ^ r);
        }
    }

    fn not<L>(left: &mut L)
    where
        L: ?Sized + BitSliceExt,
    {
        for idx in 0..left.len() {
            let l = left.get(idx);
            left.set(idx, !l);
        }
    }
}
