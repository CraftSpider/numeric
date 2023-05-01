use std::cmp::Ordering;
use crate::bit_slice::BitSliceExt;

pub trait ElementCmp: BitSliceExt {
    fn cmp<T>(left: &Self, right: &T) -> Ordering
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        if left.len() != right.len() {
            return usize::cmp(&left.len(), &right.len());
        }

        let iter = Iterator::zip(
            left.slice().iter(),
            right.slice().iter(),
        );

        for (l, r) in iter {
            match Ord::cmp(l, r) {
                Ordering::Equal => (),
                ord => return ord,
            }
        }
        Ordering::Equal
    }
}

impl<T> ElementCmp for T
where
    T: ?Sized + BitSliceExt,
{}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_len() {
        assert_eq!(
            ElementCmp::cmp(&[0u32], &[0, 1]),
            Ordering::Less,
        );
        assert_eq!(
            ElementCmp::cmp(&[0u32, 1], &[0]),
            Ordering::Greater,
        );
    }

    #[test]
    fn test_eq() {
        assert_eq!(
            ElementCmp::cmp(&[0u32], &[0]),
            Ordering::Equal,
        );

        assert_eq!(
            ElementCmp::cmp(&[1u32, 2, 3, 4], &[1, 2, 3, 4]),
            Ordering::Equal,
        );
    }

    #[test]
    fn test_same_len() {
        assert_eq!(
            ElementCmp::cmp(&[1u32], &[2]),
            Ordering::Less,
        );

        assert_eq!(
            ElementCmp::cmp(&[0u32, 2], &[0, 1]),
            Ordering::Greater,
        );
    }
}
