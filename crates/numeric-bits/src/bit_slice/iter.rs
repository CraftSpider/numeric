use std::iter::FusedIterator;
use crate::bit_slice::BitLike;

/// See `BitSlice::iter_bits`
pub struct BitIter<'a, I> {
    iter: core::slice::Iter<'a, I>,
    cur: Option<I>,
    idx: usize,
}

impl<'a, I> BitIter<'a, I> {
    pub(super) fn new(slice: &'a [I]) -> BitIter<'a, I>
    where
        I: Copy,
    {
        let mut iter = slice.iter();
        BitIter {
            cur: iter.next().copied(),
            idx: 0,
            iter,
        }
    }
}

impl<'a, I> Iterator for BitIter<'a, I>
where
    I: BitLike,
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.cur?;
        let bit_idx = self.idx % I::BIT_LEN;

        self.idx += 1;
        if bit_idx == I::BIT_LEN - 1 {
            self.cur = self.iter.next().copied();
        }

        Some(val & (I::one() << bit_idx) != I::zero())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, I> ExactSizeIterator for BitIter<'a, I>
where
    I: BitLike,
{
    fn len(&self) -> usize {
        let remaining = self.iter.len();
        I::BIT_LEN * remaining + if self.cur.is_some() { I::BIT_LEN - self.idx } else { 0 }
    }
}

impl<I> FusedIterator for BitIter<'_, I>
where
    I: BitLike,
{}
