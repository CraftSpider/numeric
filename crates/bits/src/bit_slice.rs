//! Traits for bitwise operations on slices of integers

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::fmt::Write;
use core::iter::Copied;
use core::{array, fmt, slice};
use numeric_traits::class::{BoundedBit, Integral};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::core::{BitAssignOps, NumAssignOps};
use numeric_traits::ops::overflowing::OverflowingOps;
use numeric_traits::ops::widening::WideningMul;

mod iter;

pub use iter::*;
use numeric_traits::cast::{FromAll, FromSaturating, IntoSaturating, IntoTruncating};

#[inline]
fn idx_bit<T: ?Sized + BitSlice>(idx: usize) -> (usize, usize) {
    (idx / T::Bit::BIT_LEN, idx % T::Bit::BIT_LEN)
}

/// Types that can be used as 'bit containers'. This means integers that support the common bit ops,
/// in addition to being copyable and bounded.
///
/// Note: This may be sealed or the auto-impl may be removed in future breaking versions.
pub trait BitLike:
    Integral
    + NumAssignOps
    + BitAssignOps
    + BoundedBit
    + OverflowingOps
    + WideningMul
    + FromAll<u8>
    + Ord
    + Copy
    + IntoSaturating<u8>
    + IntoTruncating<u8>
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
            + FromAll<u8>
            + IntoSaturating<u8>
            + IntoTruncating<u8>
            + Ord
            + Copy,
    > BitLike for T
{
    const BIT_LEN: usize = size_of::<T>() * 8;
}

/// Format to display a [`BitSlice`] in
#[derive(Default, PartialEq)]
pub enum DisplayFmt {
    /// Hex (base 16) format
    Hex,
    /// Binary (base 2) format
    #[default]
    Binary,
}

/// Display options for a [`BitSlice`]
#[derive(Default)]
pub struct DisplayOpts {
    /// Format or base to print in
    pub format: DisplayFmt,
}

/// Struct for writing a [`BitSlice`] to a buffer in a human-readable format
pub struct BitSliceDisplay<'a, B: ?Sized>(&'a B, DisplayOpts);

impl<B: ?Sized + BitSlice> fmt::Display for BitSliceDisplay<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        match self.1.format {
            DisplayFmt::Binary => {
                for b in (0..self.0.bit_len()).rev() {
                    let b = self.0.get_bit(b).unwrap();
                    let c = if b { '1' } else { '0' };
                    f.write_char(c)?;
                }
            }
            DisplayFmt::Hex => {
                for b in (0..self.0.len()).rev() {
                    let b = self.0.get(b).unwrap();
                    for i in (0..(B::Bit::BIT_LEN / 4)).rev() {
                        let digit: u8 = ((b >> (i * 4)) & B::Bit::saturate_from(0xF)).truncate();
                        let c = if digit < 10 { digit + 48 } else { digit + 55 };
                        f.write_char(c as char)?;
                    }
                }
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

/// Things that can be considered slices of bits. This includes slices obviously, as well as vectors
/// and other slice-like containers.
pub trait BitSlice: fmt::Debug {
    /// The bit container type contained in this slice
    type Bit: BitLike;

    /// Iterator over the items in this slice
    type Iter<'a>: Iterator<Item = Self::Bit> + ExactSizeIterator + DoubleEndedIterator + 'a
    where
        Self: 'a;

    /// Iterator over mutable references to the items in this slice
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

    /// Get the value of a bit at a given index, returning `None` if the index is out of range
    fn get_bit(&self, idx: usize) -> Option<bool> {
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

    /// Get an iterator over the values of this slice
    fn iter(&self) -> Self::Iter<'_>;

    /// Get an iterator over mutable references to the values of this slice
    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    /// Get an iterator over the bit values of this slice
    fn iter_bits(&self) -> BitIter<Self::Iter<'_>> {
        BitIter::new(self.iter())
    }

    /// Get a human-readable display value for this slice
    fn display(&self, opts: Option<DisplayOpts>) -> BitSliceDisplay<'_, Self> {
        BitSliceDisplay(self, opts.unwrap_or_default())
    }
}

impl<I: BitLike> BitSlice for [I] {
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

    #[inline]
    fn get(&self, idx: usize) -> Option<Self::Bit> {
        self.get(idx).copied()
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Bit> {
        self.get_mut(idx)
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.iter().copied()
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }
}

impl<I: BitLike, const N: usize> BitSlice for [I; N] {
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

    #[inline]
    fn get(&self, idx: usize) -> Option<Self::Bit> {
        <[I]>::get(self, idx).copied()
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Bit> {
        <[I]>::get_mut(self, idx)
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        (*self).into_iter()
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        #[allow(clippy::into_iter_on_ref)]
        self.into_iter()
    }
}

#[cfg(feature = "alloc")]
impl<I: BitLike> BitSlice for Vec<I> {
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

    #[inline]
    fn get(&self, idx: usize) -> Option<Self::Bit> {
        <[I]>::get(self, idx).copied()
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Bit> {
        <[I]>::get_mut(self, idx)
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        <&[I]>::into_iter(self).copied()
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        <&mut [I]>::into_iter(self)
    }
}

/// Things that can be considered growable vectors of bits. This includes [`Vec`] and similar
/// list-like objects.
pub trait BitVecExt: BitSlice {
    /// Extend this type with `val` up to `len`
    fn extend(&mut self, len: usize, val: Self::Bit);

    /// Truncate this type to len, which must be <= current len
    fn truncate(&mut self, len: usize);

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

    fn truncate(&mut self, len: usize) {
        self.truncate(len);
    }
}

/// Owned bit-slice variants, such as arrays and vectors.
pub trait BitOwned: BitSlice {
    /// Get a new, zeroed instance of this item. The length value is only used for dynamic values,
    /// users must always accept returned items being of lower length than requested.
    fn zeroed(len: usize) -> Self;

    /// For dynamic-length values, shrink the result down to have no trailing zeroes, or if all
    /// values are zero, only one trailing zero. This has no effect on static-length types.
    fn shrink(&mut self);
}

impl<T: BitLike, const N: usize> BitOwned for [T; N] {
    fn zeroed(_: usize) -> Self {
        [T::zero(); N]
    }

    #[inline]
    fn shrink(&mut self) {}
}

#[cfg(feature = "alloc")]
impl<T: BitLike> BitOwned for Vec<T> {
    fn zeroed(len: usize) -> Self {
        alloc::vec![T::zero(); len]
    }

    fn shrink(&mut self) {
        let idx = self.iter().rposition(|val| val != T::zero()).unwrap_or(0);
        self.drain(idx + 1..);
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
        let slice = &[0b1010_1010_1010_1010u16, 0b1010_1010_1010_1010];
        for idx in 0..32 {
            let b = slice.get_bit(idx).unwrap();
            assert_eq!(b, (idx % 2) != 0);
        }
    }

    #[test]
    fn test_set_bit() {
        let mut data = [0b1010_1010_1010_1010u16, 0b1010_1010_1010_1010];
        let slice = &mut data;
        slice.set_bit(0, true);
        slice.set_bit(31, false);
        assert_eq!(slice, &[0b1010_1010_1010_1011, 0b0010_1010_1010_1010])
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
