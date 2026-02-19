use crate::bit_slice::BitSliceExt;
use core::cmp::Ordering;

mod impls;

pub trait CmpAlgo {
    fn cmp<L, R>(left: &L, right: &R) -> Ordering
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn gt<L, R>(left: &L, right: &R) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        Self::cmp(left, right).is_gt()
    }

    fn ge<L, R>(left: &L, right: &R) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        Self::cmp(left, right).is_ge()
    }

    fn lt<L, R>(left: &L, right: &R) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        Self::cmp(left, right).is_lt()
    }

    fn le<L, R>(left: &L, right: &R) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        Self::cmp(left, right).is_le()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::Element;

    fn test_diff_len<B: CmpAlgo>() {
        assert_eq!(B::cmp(&[0u32], &[0, 1]), Ordering::Less);
        assert_eq!(B::cmp(&[0u32, 1], &[0]), Ordering::Greater);
        assert_eq!(B::cmp(&[0u32, 0], &[0]), Ordering::Equal);
        assert_eq!(B::cmp(&[0u32], &[0, 0]), Ordering::Equal);
    }

    fn test_eq<B: CmpAlgo>() {
        assert_eq!(B::cmp(&[0u32], &[0]), Ordering::Equal,);

        assert_eq!(B::cmp(&[1u32, 2, 3, 4], &[1, 2, 3, 4]), Ordering::Equal,);
    }

    fn test_same_len<B: CmpAlgo>() {
        assert_eq!(B::cmp(&[1u32], &[2]), Ordering::Less,);

        assert_eq!(B::cmp(&[0u32, 2], &[0, 1]), Ordering::Greater,);
    }

    #[test]
    fn test_element() {
        test_diff_len::<Element>();
        test_eq::<Element>();
        test_same_len::<Element>();
    }
}
