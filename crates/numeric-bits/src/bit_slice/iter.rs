use std::iter::FusedIterator;
use crate::bit_slice::BitLike;

/// See `BitSlice::iter_bits`
pub struct BitIter<'a, I> {
    slice: &'a [I],
    idx: usize,
}

impl<'a, I> BitIter<'a, I> {
    #[inline]
    pub(super) fn new(slice: &'a [I]) -> BitIter<'a, I>
    where
        I: BitLike,
    {
        BitIter {
            slice,
            idx: 0,
        }
    }
}

impl<'a, I> Iterator for BitIter<'a, I>
where
    I: BitLike,
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx / I::BIT_LEN;
        let bit_idx = self.idx % I::BIT_LEN;
        
        let val = *self.slice.get(idx)?;
        self.idx += 1;
        Some(val >> bit_idx & I::one() != I::zero())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, I> ExactSizeIterator for BitIter<'a, I>
where
    I: BitLike,
{
    #[inline]
    fn len(&self) -> usize {
        (self.slice.len() * I::BIT_LEN).saturating_sub(self.idx)
    }
}

impl<I> FusedIterator for BitIter<'_, I>
where
    I: BitLike,
{}
