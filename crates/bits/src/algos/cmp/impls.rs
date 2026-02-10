use crate::algos::cmp::CmpAlgo;
use crate::algos::Element;
use crate::bit_slice::BitSliceExt;
use core::cmp::Ordering;
use numeric_traits::identity::Zero;

impl CmpAlgo for Element {
    fn cmp<L, R>(left: &L, right: &R) -> Ordering
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let zero = L::Bit::zero();
        let len = usize::max(left.len(), right.len());
        for idx in 0..len {
            match Ord::cmp(
                &left.get_opt(idx).unwrap_or(zero),
                &right.get_opt(idx).unwrap_or(zero),
            ) {
                Ordering::Equal => (),
                ord => return ord,
            }
        }
        Ordering::Equal
    }
}
