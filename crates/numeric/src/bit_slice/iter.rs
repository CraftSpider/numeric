use std::iter::FusedIterator;
use std::mem;
use crate::bit_slice::IsBit;
use super::BitSlice;

/// See `BitSlice::iter_bits`
pub struct BitIter<'a, I> {
    iter: core::slice::Iter<'a, I>,
    cur: Option<I>,
    idx: usize,
}

impl<'a, I> BitIter<'a, I> {
    pub(super) fn new<S>(slice: &'a BitSlice<S, I>) -> BitIter<'a, I>
    where
        S: AsRef<[I]>,
        I: Copy,
    {
        let mut iter = slice.slice().iter();
        BitIter {
            cur: iter.next().copied(),
            idx: 0,
            iter,
        }
    }
}

impl<'a, I> Iterator for BitIter<'a, I>
where
    I: IsBit,
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.cur?;
        let bit_size = mem::size_of::<I>() * 8;
        let bit_idx = self.idx % bit_size;

        self.idx += 1;
        if bit_idx == bit_size - 1 {
            self.cur = self.iter.next().copied();
        }

        Some(val & (I::one() << bit_idx) != I::zero(), )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, I> ExactSizeIterator for BitIter<'a, I>
where
    I: IsBit,
{
    fn len(&self) -> usize {
        let remaining = self.iter.len();
        let size = mem::size_of::<I>() * 8;
        size * remaining + if self.cur.is_some() { size - self.idx } else { 0 }
    }
}

impl<I> FusedIterator for BitIter<'_, I>
where
    I: IsBit,
{}
