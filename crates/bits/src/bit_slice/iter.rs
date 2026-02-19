use crate::bit_slice::BitLike;
use core::iter::FusedIterator;
use numeric_traits::identity::{One, Zero};

/// See `BitSlice::iter_bits`
pub struct BitIter<I: Iterator> {
    iter: I,
    cur: Option<I::Item>,
    idx: usize,
}

impl<I: Iterator> BitIter<I> {
    pub(super) fn new(mut iter: I) -> BitIter<I> {
        BitIter {
            cur: iter.next(),
            idx: 0,
            iter,
        }
    }
}

impl<I> Iterator for BitIter<I>
where
    I: ExactSizeIterator,
    I::Item: BitLike,
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.cur?;
        let bit_idx = self.idx % I::Item::BIT_LEN;

        self.idx += 1;
        if bit_idx == I::Item::BIT_LEN - 1 {
            self.cur = self.iter.next();
        }

        Some(val & (I::Item::one() << bit_idx) != I::Item::zero())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<I> ExactSizeIterator for BitIter<I>
where
    I: ExactSizeIterator,
    I::Item: BitLike,
{
    fn len(&self) -> usize {
        let remaining = self.iter.len();
        I::Item::BIT_LEN * remaining
            + if self.cur.is_some() {
                I::Item::BIT_LEN - self.idx
            } else {
                0
            }
    }
}

impl<I> FusedIterator for BitIter<I>
where
    I: ExactSizeIterator + FusedIterator,
    I::Item: BitLike,
{
}
