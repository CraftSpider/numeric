//! A type for bitwise operations on slices of integers

use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Neg;
use num_traits::PrimInt;

mod algos;
mod iter;

pub use iter::*;

/// Utility for algorithms on slices of primitive integers
pub struct BitSlice<S, I>(S, PhantomData<[I]>);

impl<S, I> Clone for BitSlice<S, I>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        BitSlice(self.0.clone(), PhantomData)
    }
}

impl<S, I> BitSlice<S, I> {
    /// Create a new `BitSlice` containing a value
    #[inline(always)]
    pub fn new(inner: S) -> BitSlice<S, I> {
        BitSlice(inner, PhantomData)
    }

    /// Get a reference to the value in this `BitSlice`
    pub fn inner(&self) -> &S {
        &self.0
    }

    /// Consume this `BitSlice` to regain ownership of its contained value
    pub fn into_inner(self) -> S {
        self.0
    }
}

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]>,
{
    /// Access this items as a slice of its elements
    #[inline(always)]
    pub fn slice(&self) -> &[I] {
        self.0.as_ref()
    }

    /// Get the length of this `BitSlice` in terms of `S`
    pub fn len(&self) -> usize {
        self.slice().len()
    }

    /// Whether this `BitSlice` is empty
    pub fn is_empty(&self) -> bool {
        self.slice().is_empty()
    }

    /// Get the length of this `BitSlice` in bits
    pub fn bit_len(&self) -> usize {
        self.len() * std::mem::size_of::<I>() * 8
    }
}

impl<S, I> BitSlice<S, I>
where
    S: AsMut<[I]>,
{
    /// Access this item as a mutable slice of its inner elements
    #[inline(always)]
    pub fn slice_mut(&mut self) -> &mut [I] {
        self.0.as_mut()
    }
}

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]>,
    I: PrimInt,
{
    fn idx_bit(&self, pos: usize) -> (usize, usize) {
        (
            pos / (std::mem::size_of::<I>() * 8),
            pos % (std::mem::size_of::<I>() * 8),
        )
    }

    /// Get the value of an element at a given position, panicking if the index is out of range
    pub fn get(&self, pos: usize) -> I {
        self.get_opt(pos).expect("get index in-bounds")
    }

    /// Get the value of an element at a given position, returning `None` if the index is out of
    /// range
    pub fn get_opt(&self, pos: usize) -> Option<I> {
        self.slice().get(pos).copied()
    }

    /// Get the value of a bit at a given position, panicking if the index is out of range
    ///
    /// # Panics
    ///
    /// If `pos` is outside the range of this slice
    pub fn get_bit(&self, pos: usize) -> bool {
        self.get_bit_opt(pos).expect("get_bit index in-bounds")
    }

    /// Get the value of a bit at a given position, returning `None` if the index is out of range
    pub fn get_bit_opt(&self, pos: usize) -> Option<bool> {
        let (idx, bit) = self.idx_bit(pos);
        self.slice().get(idx).copied().map(|val| {
            val & (I::one() << bit) != I::zero()
        })
    }

    /// Get an iterator over the bit values of this slice
    pub fn iter_bits(&self) -> BitIter<'_, I> {
        BitIter::new(self)
    }
}

impl<S, I> BitSlice<S, I>
where
    S: AsRef<[I]> + AsMut<[I]>,
    I: PrimInt,
{
    /// Set a single value by index on this slice, panicking if the index is out of range
    pub fn set(&mut self, pos: usize, val: I) {
        self.set_opt(pos, val).unwrap_or_else(|| {
            panic!("Attempt to write value at index {} out of bounds", pos)
        })
    }

    /// Set a single value by index on this slice, returning `None` if the index is out of range
    #[must_use]
    pub fn set_opt(&mut self, pos: usize, val: I) -> Option<()> {
        self.slice_mut().get_mut(pos).map(|cur| {
            *cur = val;
        })
    }

    /// Set a single value by index on this slice, doing nothing if the index is out of range
    #[inline]
    pub fn set_ignore(&mut self, pos: usize, val: I) {
        let _ = self.set_opt(pos, val);
    }

    /// Set a single bit by index on this slice, panicking if the index is out of range
    ///
    /// # Panics
    ///
    /// If `pos` is outside the range of this slice
    pub fn set_bit(&mut self, pos: usize, val: bool) {
        self.set_bit_opt(pos, val).unwrap_or_else(|| {
            let (idx, bit) = self.idx_bit(pos);
            panic!("Attempt to write bit at {}:{} out of bounds", idx, bit)
        });
    }

    /// Set a single bit by index on this slice, returning `None` if the index is out of range
    #[must_use]
    pub fn set_bit_opt(&mut self, pos: usize, val: bool) -> Option<()> {
        let (idx, bit) = self.idx_bit(pos);
        if let Some(item) = self.slice_mut().get_mut(idx) {
            *item = *item & !(I::one() << bit);
            if val {
                *item = *item | (I::one() << bit);
            }
            Some(())
        } else {
            None
        }
    }

    /// Set a single bit by index on this slice, doing nothing if the index is out of range
    #[inline]
    pub fn set_bit_ignore(&mut self, pos: usize, val: bool) {
        let _ = self.set_bit_opt(pos, val);
    }
}

impl<T> BitSlice<Vec<T>, T> {
    /// Get a `BitSlice<&[T]>` of this value
    #[must_use]
    pub fn as_slice(&self) -> BitSlice<&[T], T> {
        BitSlice::new(&self.0)
    }

    /// Get a `BitSlice<&mut [T]>` of this value
    #[must_use]
    pub fn as_mut_slice(&mut self) -> BitSlice<&mut [T], T> {
        BitSlice::new(&mut self.0)
    }
}

impl<T> BitSlice<Vec<T>, T>
where
    T: PrimInt,
{
    /// Set a single value by index on this slice, extending it if the index is out of range
    pub fn set_pushing(&mut self, pos: usize, val: T) {
        let item = loop {
            match self.0.get_mut(pos) {
                Some(item) => break item,
                None => self.0.push(T::zero()),
            }
        };
        *item = val;
    }

    /// Set a single bit by index on this slice, extending it if the index is out of range
    pub fn set_bit_pushing(&mut self, pos: usize, val: bool) {
        let (idx, bit) = self.idx_bit(pos);

        let item = loop {
            match self.0.get_mut(idx) {
                Some(item) => break item,
                None => self.0.push(T::zero()),
            }
        };

        *item = *item & !(T::one() << bit);
        if val {
            *item = *item | (T::one() << bit);
        }
    }
}

impl<T> BitSlice<&mut [T], T> {
    /// Get a `BitSlice<&[T]>` of this value
    #[must_use]
    pub fn as_slice(&self) -> BitSlice<&[T], T> {
        BitSlice::new(self.0)
    }
}

impl<S, I> fmt::Debug for BitSlice<S, I>
where
    S: AsRef<[I]>,
    I: PrimInt,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for idx in (0..self.bit_len()).rev() {
            write!(f, "{}", self.get_bit(idx) as u8)?;
        }
        Ok(())
    }
}

impl<S, I> Neg for BitSlice<S, I>
where
    S: AsRef<[I]> + AsMut<[I]>,
    I: PrimInt,
{
    type Output = BitSlice<S, I>;

    fn neg(mut self) -> Self::Output {
        for idx in 0..self.bit_len() {
            self.set_bit(idx, !self.get_bit(idx));
        }
        self
    }
}

impl<S, T, I1, I2> PartialEq<BitSlice<T, I2>> for BitSlice<S, I1>
where
    S: PartialEq<T>,
{
    fn eq(&self, other: &BitSlice<T, I2>) -> bool {
        self.0 == other.0
    }
}

impl<T, U> PartialOrd<BitSlice<&[U], U>> for BitSlice<Vec<T>, T>
where
    T: PartialEq<U>,
    [T]: PartialOrd<[U]>,
{
    fn partial_cmp(&self, other: &BitSlice<&[U], U>) -> Option<Ordering> {
        <[T] as PartialOrd<[U]>>::partial_cmp(&self.0, other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idx() {
        let slice = BitSlice::<&[u8], _>::new(&[]);

        for i in 0..8 {
            for j in 0..8 {
                assert_eq!(slice.idx_bit(i * 8 + j), (i, j));
            }
        }
    }

    #[test]
    fn test_get_bit() {
        let slice = BitSlice::<&[u16], _>::new(&[0b1010101010101010, 0b1010101010101010]);
        for idx in 0..32 {
            let b = slice.get_bit(idx);
            assert_eq!(b, (idx % 2) != 0);
        }
    }

    #[test]
    fn test_set_bit() {
        let mut data = [0b1010101010101010, 0b1010101010101010];
        let mut slice = BitSlice::<&mut [u16], _>::new(&mut data);
        slice.set_bit(0, true);
        slice.set_bit(31, false);
        assert_eq!(slice.inner(), &[0b1010101010101011, 0b0010101010101010])
    }

    #[test]
    fn test_add_shift_mul_bitwise() {
        let slice1 = BitSlice::<&[u8], _>::new(&[0b00000000]);
        let slice2 = BitSlice::<&[u8], _>::new(&[0b00000001]);

        assert_eq!(BitSlice::add_shift_mul_bitwise(slice1, slice2).inner(), &[0b0, 0b0, 0b0]);

        let slice3 = BitSlice::<&[u8], _>::new(&[0b00000001]);
        let slice4 = BitSlice::<&[u8], _>::new(&[0b00000010]);

        assert_eq!(BitSlice::add_shift_mul_bitwise(slice3, slice4).inner(), &[0b10, 0b0, 0b0]);

        let slice5 = BitSlice::<&[u8], _>::new(&[0b00000010]);
        let slice6 = BitSlice::<&[u8], _>::new(&[0b00000010]);

        assert_eq!(BitSlice::add_shift_mul_bitwise(slice5, slice6).inner(), &[0b100, 0b0, 0b0]);
    }

    #[test]
    fn test_div() {
        let slice1 = BitSlice::<&[u8], _>::new(&[0b10]);
        let slice2 = BitSlice::<&[u8], _>::new(&[0b01]);

        assert_eq!(BitSlice::long_div_bitwise(slice1, slice2).0.inner(), &[0b10]);

        let slice3 = BitSlice::<&[u8], _>::new(&[0b10]);
        let slice4 = BitSlice::<&[u8], _>::new(&[0b10]);

        assert_eq!(BitSlice::long_div_bitwise(slice3, slice4).0.inner(), &[0b01]);

        let slice5 = BitSlice::<&[u8], _>::new(&[0b00000000, 0b1]);
        let slice6 = BitSlice::<&[u8], _>::new(&[0b00000010]);

        assert_eq!(BitSlice::long_div_bitwise(slice5, slice6).0.inner(), &[0b10000000, 0b0]);

        let slice7 = BitSlice::<&[u8], _>::new(&[0b0, 0b0, 0b0, 0b1]);
        let slice8 = BitSlice::<&[u8], _>::new(&[0b10]);

        assert_eq!(BitSlice::long_div_bitwise(slice7, slice8).0.inner(), &[0b0, 0b0, 0b10000000, 0b0]);
    }

    #[test]
    fn test_rem() {
        for i in 0..4 {
            let slice = &[i];
            let slice1 = BitSlice::<&[u8], _>::new(slice);
            let slice2 = BitSlice::<&[u8], _>::new(&[0b10]);

            assert_eq!(BitSlice::long_div_bitwise(slice1, slice2).1.inner(), &[i % 2]);
        }

        for i in 0..6 {
            let slice = &[i];
            let slice3 = BitSlice::<&[u8], _>::new(slice);
            let slice4 = BitSlice::<&[u8], _>::new(&[0b11]);

            assert_eq!(BitSlice::long_div_bitwise(slice3, slice4).1.inner(), &[i % 3]);
        }

        let slice5 = BitSlice::<&[u8], _>::new(&[0b00000001, 0b111]);
        let slice6 = BitSlice::<&[u8], _>::new(&[0b00000010]);

        assert_eq!(BitSlice::long_div_bitwise(slice5, slice6).1.inner(), &[0b01, 0b0]);
    }

    #[test]
    fn test_shl() {
        let slice = BitSlice::<&[u16], _>::new(&[0b1010101010101010, 0b1010101010101010]);
        let res = BitSlice::shl_bitwise(slice.clone(), 1);
        assert_eq!(res.inner(), &[0b0101010101010100, 0b0101010101010101, 0b1]);

        let res = BitSlice::shl_wrap_and_mask(slice, 1);
        assert_eq!(res.inner(), &[0b0101010101010100, 0b0101010101010101, 0b1]);
    }

    #[test]
    fn test_checked_shl() {
        let val = &mut [0b1010101010101010, 0b1010101010101010];
        let slice = BitSlice::<&mut [u16], _>::new(val);
        let res = BitSlice::shl_wrap_and_mask_checked(slice, 1);
        assert_eq!(res.map(|s| s.into_inner()), Some(&mut [0b0101010101010100u16, 0b0101010101010101] as &mut [_]));

        let val = &mut [0b1010101010101010, 0b1010101010101010];
        let slice = BitSlice::<&mut [u16], _>::new(val);
        let res = BitSlice::shl_wrap_and_mask_checked(slice, 33);
        assert_eq!(res, None);
    }

    #[test]
    fn test_wrapping_shl() {
        let val = &mut [0b1010101010101010, 0b1010101010101010];
        let slice = BitSlice::<&mut [u16], _>::new(val);
        let res = BitSlice::shl_wrap_and_mask_wrapping(slice, 1);
        assert_eq!(res.inner(), &[0b0101010101010100, 0b0101010101010101]);

        let val = &mut [0b1010101010101010, 0b1010101010101010];
        let slice = BitSlice::<&mut [u16], _>::new(val);
        let res = BitSlice::shl_wrap_and_mask_wrapping(slice, 33);
        assert_eq!(res.inner(), &[0b0101010101010100, 0b0101010101010101]);
    }
}
