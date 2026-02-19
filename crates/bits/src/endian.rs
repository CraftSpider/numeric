use crate::bit_slice::{BitLike, BitSliceExt};
use core::iter::{Copied, Rev};
use core::{fmt, slice};

macro_rules! impl_from {
    ($ty:ident) => {
        impl<T> From<&[T]> for &$ty<T> {
            fn from(value: &[T]) -> Self {
                // SAFETY: All endian slices are repr(transparent) wrappers over other slice types
                //         or wrappers
                unsafe { core::mem::transmute::<&[T], &$ty<T>>(value) }
            }
        }

        impl<T> From<&mut [T]> for &mut $ty<T> {
            fn from(value: &mut [T]) -> Self {
                // SAFETY: All endian slices are repr(transparent) wrappers over other slice types
                //         or wrappers
                unsafe { core::mem::transmute::<&mut [T], &mut $ty<T>>(value) }
            }
        }

        impl<T, const N: usize> From<&[T; N]> for &$ty<T> {
            fn from(value: &[T; N]) -> Self {
                <&$ty<T> as From<&[T]>>::from(value)
            }
        }

        impl<T, const N: usize> From<&mut [T; N]> for &mut $ty<T> {
            fn from(value: &mut [T; N]) -> Self {
                <&mut $ty<T> as From<&mut [T]>>::from(value)
            }
        }
    };
}

#[repr(transparent)]
pub struct LeSlice<T>([T]);

impl_from!(LeSlice);

impl<T: fmt::Debug> fmt::Debug for LeSlice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <[T] as fmt::Debug>::fmt(&self.0, f)
    }
}

impl<T: BitLike> BitSliceExt for LeSlice<T> {
    type Bit = T;
    type Iter<'a>
        = Copied<slice::Iter<'a, T>>
    where
        Self: 'a;

    type IterMut<'a>
        = slice::IterMut<'a, T>
    where
        Self: 'a;

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn get(&self, idx: usize) -> Option<Self::Bit> {
        self.0.get(idx).copied()
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Bit> {
        self.0.get_mut(idx)
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter().copied()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.0.iter_mut()
    }
}

#[repr(transparent)]
pub struct BeSlice<T>([T]);

impl_from!(BeSlice);

impl<T: fmt::Debug> fmt::Debug for BeSlice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <[T] as fmt::Debug>::fmt(&self.0, f)
    }
}

impl<T: BitLike> BitSliceExt for BeSlice<T> {
    type Bit = T;
    type Iter<'a>
        = Rev<Copied<slice::Iter<'a, T>>>
    where
        Self: 'a;

    type IterMut<'a>
        = Rev<slice::IterMut<'a, T>>
    where
        Self: 'a;

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn get(&self, idx: usize) -> Option<Self::Bit> {
        let len = self.len();
        self.0.get(len - 1 - idx).copied()
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Bit> {
        let len = self.len();
        self.0.get_mut(len - 1 - idx)
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter().copied().rev()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.0.iter_mut().rev()
    }
}

#[repr(transparent)]
pub struct NeSlice<T>(
    #[cfg(target_endian = "little")] LeSlice<T>,
    #[cfg(target_endian = "big")] BeSlice<T>,
);

impl_from!(NeSlice);

impl<T: fmt::Debug> fmt::Debug for NeSlice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <[T] as fmt::Debug>::fmt(&self.0 .0, f)
    }
}

impl<T: BitLike> BitSliceExt for NeSlice<T> {
    type Bit = T;

    #[cfg(target_endian = "little")]
    type Iter<'a>
        = <LeSlice<T> as BitSliceExt>::Iter<'a>
    where
        Self: 'a;

    #[cfg(target_endian = "big")]
    type Iter<'a>
        = <BeSlice<T> as BitSliceExt>::Iter<'a>
    where
        Self: 'a;

    #[cfg(target_endian = "little")]
    type IterMut<'a>
        = <LeSlice<T> as BitSliceExt>::IterMut<'a>
    where
        Self: 'a;

    #[cfg(target_endian = "big")]
    type IterMut<'a>
        = <BeSlice<T> as BitSliceExt>::IterMut<'a>
    where
        Self: 'a;

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn get(&self, idx: usize) -> Option<Self::Bit> {
        self.0.get(idx)
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Bit> {
        self.0.get_mut(idx)
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.0.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_le_index() {
        let s = <&LeSlice<_>>::from(&[0u32, 1, 2, 3]);

        assert_eq!(s.get(0), Some(0));
        assert_eq!(s.get(3), Some(3));
    }

    #[test]
    fn test_be_index() {
        let s = <&BeSlice<_>>::from(&[0u32, 1, 2, 3]);

        assert_eq!(s.get(0), Some(3));
        assert_eq!(s.get(3), Some(0));
    }
}
