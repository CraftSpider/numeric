//! A type for bitwise operations on slices of integers

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::iter::Copied;
use core::{array, mem, slice};
use numeric_traits::class::{BoundedBit, Integral};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::core::{BitAssignOps, NumAssignOps};
use numeric_traits::ops::overflowing::OverflowingOps;
use numeric_traits::ops::widening::WideningMul;

mod iter;

pub use iter::*;

#[inline]
fn idx_bit<T: ?Sized + BitSliceExt>(idx: usize) -> (usize, usize) {
    (idx / T::Bit::BIT_LEN, idx % T::Bit::BIT_LEN)
}

/// Trait for types that can be used as 'bit containers'. This means integers that support the
/// common bit ops, in addition to being copyable and bounded.
pub trait BitLike:
    Integral + NumAssignOps + BitAssignOps + BoundedBit + OverflowingOps + WideningMul + Ord + Copy
{
    /// The length of this type in bits.
    const BIT_LEN: usize;
}

impl<
        T: Integral
            + NumAssignOps
            + BitAssignOps
            + BoundedBit
            + OverflowingOps
            + WideningMul
            + Ord
            + Copy,
    > BitLike for T
{
    const BIT_LEN: usize = mem::size_of::<T>() * 8;
}

/// Trait for things that can be considered slices of bits. This includes slices obviously, as well
/// as vectors and other slice-like containers.
pub trait BitSliceExt: core::fmt::Debug {
    /// The bit container type contained in this slice
    type Bit: BitLike;

    type Iter<'a>: Iterator<Item = Self::Bit> + ExactSizeIterator + DoubleEndedIterator + 'a
    where
        Self: 'a;

    type IterMut<'a>: Iterator<Item = &'a mut Self::Bit>
        + ExactSizeIterator
        + DoubleEndedIterator
        + 'a
    where
        Self: 'a;

    // /// Access this item as a mutable slice of its elements
    // fn slice_mut(&mut self) -> &mut [Self::Bit];

    /// Get the length of this slice in terms of [`Self::Bit`]
    fn len(&self) -> usize;

    /// Whether this slice is empty
    fn is_empty(&self) -> bool;

    /// Get the length of this slice in bits
    #[inline]
    fn bit_len(&self) -> usize {
        self.len() * Self::Bit::BIT_LEN
    }

    /// Get the value of an element at a given index, returning `None` if the index is out of
    /// range
    fn get(&self, idx: usize) -> Option<Self::Bit>;

    /// Get a mutable reference to a value at a given index, returning `None` if the index is out
    /// of range.
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Bit>;

    /// Get the value of a bit at a given position, panicking if the index is out of range
    ///
    /// # Panics
    ///
    /// If `idx` is outside the range of this slice
    fn get_bit(&self, idx: usize) -> bool {
        self.get_bit_opt(idx).expect("get_bit index in-bounds")
    }

    /// Get the value of a bit at a given index, returning `None` if the index is out of range
    fn get_bit_opt(&self, idx: usize) -> Option<bool> {
        let (idx, bit) = idx_bit::<Self>(idx);
        self.get(idx)
            .map(|val| val & (<Self::Bit as One>::one() << bit) != <Self::Bit as Zero>::zero())
    }

    /// Set a single value by index on this slice, panicking if the index is out of range
    ///
    /// # Panics
    ///
    /// If `idx` is outside the range of this slice
    fn set(&mut self, idx: usize, val: Self::Bit) {
        self.set_opt(idx, val)
            .unwrap_or_else(|| panic!("Attempt to write value at index {} out of bounds", idx));
    }

    /// Set a single value by index on this slice, returning `None` if the index is out of range
    #[must_use]
    fn set_opt(&mut self, idx: usize, val: Self::Bit) -> Option<()> {
        self.get_mut(idx).map(|cur| {
            *cur = val;
        })
    }

    /// Set a single bit by index on this slice, panicking if the index is out of range
    ///
    /// # Panics
    ///
    /// If `idx` is outside the range of this slice
    fn set_bit(&mut self, idx: usize, val: bool) {
        self.set_bit_opt(idx, val).unwrap_or_else(|| {
            let (idx, bit) = idx_bit::<Self>(idx);
            panic!("Attempt to write bit at {}:{} out of bounds", idx, bit)
        });
    }

    /// Set a single bit by index on this slice, returning `None` if the index is out of range
    fn set_bit_opt(&mut self, idx: usize, val: bool) -> Option<()> {
        let (idx, bit) = idx_bit::<Self>(idx);
        self.get_mut(idx).map(|item| {
            *item &= !(Self::Bit::one() << bit);
            if val {
                *item |= Self::Bit::one() << bit;
            }
        })
    }

    /// Set a single value by index on this slice, doing nothing if the index is out of range
    #[inline]
    fn set_ignore(&mut self, pos: usize, val: Self::Bit) {
        let _ = self.set_opt(pos, val);
    }

    /// Set a single bit by index on this slice, doing nothing if the index is out of range
    #[inline]
    fn set_bit_ignore(&mut self, pos: usize, val: bool) {
        let _ = self.set_bit_opt(pos, val);
    }

    fn iter(&self) -> Self::Iter<'_>;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    /// Get an iterator over the bit values of this slice
    fn iter_bits(&self) -> BitIter<Self::Iter<'_>> {
        BitIter::new(self.iter())
    }
}

impl<I: BitLike> BitSliceExt for [I] {
    type Bit = I;

    type Iter<'a>
        = Copied<slice::Iter<'a, Self::Bit>>
    where
        Self: 'a;

    type IterMut<'a>
        = slice::IterMut<'a, Self::Bit>
    where
        Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter().copied()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }

    fn get(&self, idx: usize) -> Option<Self::Bit> {
        self.get(idx).copied()
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Bit> {
        self.get_mut(idx)
    }
}

impl<I: BitLike, const N: usize> BitSliceExt for [I; N] {
    type Bit = I;

    type Iter<'a>
        = array::IntoIter<Self::Bit, N>
    where
        Self: 'a;

    type IterMut<'a>
        = slice::IterMut<'a, Self::Bit>
    where
        Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn is_empty(&self) -> bool {
        N == 0
    }

    fn iter(&self) -> Self::Iter<'_> {
        (*self).into_iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.into_iter()
    }

    fn get(&self, idx: usize) -> Option<Self::Bit> {
        <[I]>::get(self, idx).copied()
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Bit> {
        <[I]>::get_mut(self, idx)
    }
}

#[cfg(feature = "alloc")]
impl<I: BitLike> BitSliceExt for Vec<I> {
    type Bit = I;

    type Iter<'a>
        = Copied<slice::Iter<'a, Self::Bit>>
    where
        Self: 'a;

    type IterMut<'a>
        = slice::IterMut<'a, Self::Bit>
    where
        Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn iter(&self) -> Self::Iter<'_> {
        <&[I]>::into_iter(self).copied()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        <&mut [I]>::into_iter(self)
    }

    fn get(&self, idx: usize) -> Option<Self::Bit> {
        <[I]>::get(self, idx).copied()
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Bit> {
        <[I]>::get_mut(self, idx)
    }
}

/// Trait for things that can be considered growable vectors of bits. This includes vectors and
/// any vector-like object.
pub trait BitVecExt: BitSliceExt {
    /// Extend this type with `val` up to `len`
    fn extend(&mut self, len: usize, val: Self::Bit);

    /// Set a single value by index on this slice, extending it if the index is out of range
    fn set_push(&mut self, idx: usize, val: Self::Bit) {
        self.extend(idx, Self::Bit::zero());
        self.set_ignore(idx, val);
    }

    /// Set a single bit by index on this slice, extending it if the index is out of range
    fn set_bit_push(&mut self, idx: usize, val: bool) {
        let (len, _) = idx_bit::<Self>(idx);
        self.extend(len + 1, Self::Bit::zero());
        self.set_bit_ignore(idx, val);
    }
}

#[cfg(feature = "alloc")]
impl<I: BitLike> BitVecExt for Vec<I> {
    fn extend(&mut self, len: usize, val: Self::Bit) {
        if len > self.len() {
            self.resize(len, val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "alloc")]
    use alloc::vec;

    #[test]
    fn test_idx() {
        for i in 0..8 {
            for j in 0..8 {
                assert_eq!(idx_bit::<[u8]>(i * 8 + j), (i, j));
            }
        }
    }

    #[test]
    fn test_get_bit() {
        let slice = &[0b1010101010101010u16, 0b1010101010101010];
        for idx in 0..32 {
            let b = slice.get_bit(idx);
            assert_eq!(b, (idx % 2) != 0);
        }
    }

    #[test]
    fn test_set_bit() {
        let mut data = [0b1010101010101010u16, 0b1010101010101010];
        let slice = &mut data;
        slice.set_bit(0, true);
        slice.set_bit(31, false);
        assert_eq!(slice, &[0b1010101010101011, 0b0010101010101010])
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_extend() {
        let mut data = vec![0u8; 1];
        BitVecExt::extend(&mut data, 1, 0);
        assert_eq!(&data, &[0]);
        BitVecExt::extend(&mut data, 2, 1);
        assert_eq!(&data, &[0, 1]);
        BitVecExt::extend(&mut data, 1, 0);
        assert_eq!(&data, &[0, 1]);
    }
}
