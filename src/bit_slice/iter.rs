use num_traits::{One, PrimInt, Zero};
use crate::bit_slice::private::{IndexOpt, Len};
use super::BitSlice;

pub struct Iter<'a, S> {
    slice: &'a BitSlice<S>,
    idx_front: usize,
    idx_back: usize,
}

impl<'a, S> Iter<'a, S> {
    pub(super) fn new(slice: &'a BitSlice<S>) -> Iter<'a, S> {
        Iter {
            slice,
            idx_front: 0,
            idx_back: 0,
        }
    }
}

impl<'a, S> Iterator for Iter<'a, S>
where
    S: IndexOpt<usize> + Len,
    S::Output: Sized + Copy,
{
    type Item = (usize, S::Output);

    fn next(&mut self) -> Option<Self::Item> {
        let orig_idx = self.idx_front;
        if self.idx_front + self.idx_back >= self.slice.len() {
            return None;
        }
        let out = *self.slice.0.index(orig_idx)?;
        self.idx_front += 1;
        Some((orig_idx, out))
    }
}

impl<'a, S> ExactSizeIterator for Iter<'a, S>
where
    S: IndexOpt<usize> + Len,
    S::Output: Sized + Copy,
{
    fn len(&self) -> usize {
        self.slice.len()
    }
}

impl<'a, S> DoubleEndedIterator for Iter<'a, S>
where
    S: IndexOpt<usize> + Len,
    S::Output: Sized + Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.slice.len();
        if self.idx_back == len || self.idx_front + self.idx_back >= len {
            return None;
        }
        let orig_idx = len - self.idx_back - 1;
        let out = *self.slice.0.index(orig_idx)?;
        self.idx_back += 1;
        Some((orig_idx, out))
    }
}

pub struct BitIter<'a, S> {
    slice: &'a BitSlice<S>,
    idx: usize,
}

impl<'a, S> BitIter<'a, S> {
    pub(super) fn new(slice: &'a BitSlice<S>) -> BitIter<'a, S> {
        BitIter {
            slice,
            idx: 0,
        }
    }
}

impl<'a, S> Iterator for BitIter<'a, S>
where
    S: IndexOpt<usize>,
    S::Output: PrimInt,
{
    type Item = (usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let orig_idx = self.idx;
        let (idx, bit) = self.slice.idx_bit(orig_idx);
        let val = *self.slice.0.index(idx)?;
        self.idx += 1;
        Some((
            orig_idx,
            val & (<S::Output>::one() << bit) != <S::Output>::zero(),
        ))
    }
}

