use crate::bit_slice::BitSliceExt;
use core::cmp::Ordering;
use numeric_traits::identity::Zero;

pub trait ElementCmp: BitSliceExt {
    fn cmp<T>(left: &Self, right: &T) -> Ordering
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let zero = Self::Bit::zero();
        let len = usize::max(left.len(), right.len());
        // let iter = Iterator::zip(left.iter(), right.iter());
        for idx in 0..len {
            match Ord::cmp(
                &left.get_opt(idx).unwrap_or(zero),
                &right.get_opt(idx).unwrap_or(zero),
            ) {
                Ordering::Equal => (),
                ord => return ord,
            }
        }
        // for (l, r) in iter {
        //     match Ord::cmp(l, r) {
        //         Ordering::Equal => (),
        //         ord => return ord,
        //     }
        // }
        Ordering::Equal
    }
}

impl<T> ElementCmp for T where T: ?Sized + BitSliceExt {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_len() {
        assert_eq!(ElementCmp::cmp(&[0u32], &[0, 1]), Ordering::Less);
        assert_eq!(ElementCmp::cmp(&[0u32, 1], &[0]), Ordering::Greater);
        assert_eq!(ElementCmp::cmp(&[0u32, 0], &[0]), Ordering::Equal);
        assert_eq!(ElementCmp::cmp(&[0u32], &[0, 0]), Ordering::Equal);
    }

    #[test]
    fn test_eq() {
        assert_eq!(ElementCmp::cmp(&[0u32], &[0]), Ordering::Equal,);

        assert_eq!(
            ElementCmp::cmp(&[1u32, 2, 3, 4], &[1, 2, 3, 4]),
            Ordering::Equal,
        );
    }

    #[test]
    fn test_same_len() {
        assert_eq!(ElementCmp::cmp(&[1u32], &[2]), Ordering::Less,);

        assert_eq!(ElementCmp::cmp(&[0u32, 2], &[0, 1]), Ordering::Greater,);
    }
}
