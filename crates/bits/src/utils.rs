#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::ops::Deref;
use numeric_traits::cast::{FromAll, FromChecked, IntoTruncating};
use numeric_traits::class::{Bounded, Integral, Unsigned};
use numeric_traits::ops::checked::CheckedShl;
use numeric_traits::ops::core::NumAssignOps;

pub trait IntSlice<T>: Deref<Target = [T]> {
    fn shrink(self) -> Self;
}

impl<T: Integral + Copy> IntSlice<T> for &[T] {
    fn shrink(self) -> Self {
        let idx = self.iter().rposition(|val| *val != T::zero()).unwrap_or(0);
        &self[..=idx]
    }
}

#[cfg(feature = "alloc")]
impl<T: Integral + Copy> IntSlice<T> for Vec<T> {
    fn shrink(mut self) -> Self {
        let idx = self.iter().rposition(|val| *val != T::zero()).unwrap_or(0);
        self.drain(idx + 1..);
        self
    }
}

pub fn int_to_arr<T, U, const N: usize>(val: T) -> [U; N]
where
    T: Integral + FromAll<U> + Bounded + Copy + Unsigned,
    U: Integral + FromAll<T> + Bounded + Copy + Unsigned,
{
    debug_assert!(N != 0);

    let mut out = [U::zero(); N];

    match U::from_checked(val) {
        Some(u) => out[0] = u,
        None => {
            // This is fine to truncate - if our value didn't fit in a `T`, we should
            // have succeeded our earlier check
            let max: T = T::truncate_from(U::max_value()) + T::one();

            let mut left = val;
            for item in &mut out {
                match U::from_checked(left) {
                    Some(u) => {
                        *item = u;
                        break;
                    }
                    None => {
                        let rem: T = left % max;
                        left = left / max;
                        // Modulo U::max_value() + 1 - Will always be <= U::max_value();
                        *item = rem.truncate();
                    }
                }
            }
        }
    }

    out
}

pub fn arr_to_int<
    T: Integral + Copy,
    U: Integral + CheckedShl<usize, Output = U> + NumAssignOps + FromChecked<T> + Copy,
>(
    arr: &[T],
) -> Option<U> {
    let bit_len = size_of::<T>() * 8;
    let mut out = U::zero();
    for (idx, &i) in arr.iter().enumerate() {
        let t = U::from_checked(i)?;
        out += t.checked_shl(idx * bit_len)?;
    }
    Some(out)
}

pub const fn const_reverse<const N: usize>(mut bytes: [u8; N]) -> [u8; N] {
    let mut idx = 0;
    while idx < N / 2 {
        let tmp = bytes[idx];
        bytes[idx] = bytes[bytes.len() - idx - 1];
        bytes[bytes.len() - idx - 1] = tmp;
        idx += 1;
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "alloc")]
    use alloc::vec;

    #[test]
    fn test_shrink_slice() {
        assert_eq!(IntSlice::shrink(&[0] as &[_]), &[0]);
        assert_eq!(IntSlice::shrink(&[0, 0] as &[_]), &[0]);
        assert_eq!(IntSlice::shrink(&[0, 0, 0] as &[_]), &[0]);
        assert_eq!(IntSlice::shrink(&[0, 1] as &[_]), &[0, 1]);
        assert_eq!(IntSlice::shrink(&[0, 1, 1] as &[_]), &[0, 1, 1]);

        assert_eq!(IntSlice::shrink(&[1] as &[_]), &[1]);
        assert_eq!(IntSlice::shrink(&[1, 1] as &[_]), &[1, 1]);
        assert_eq!(IntSlice::shrink(&[1, 1, 1] as &[_]), &[1, 1, 1]);
        assert_eq!(IntSlice::shrink(&[1, 0] as &[_]), &[1]);
        assert_eq!(IntSlice::shrink(&[1, 0, 0] as &[_]), &[1]);

        assert_eq!(IntSlice::shrink(&[1, 0, 1] as &[_]), &[1, 0, 1]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_shrink_vec() {
        assert_eq!(IntSlice::shrink(vec![0]), &[0]);
        assert_eq!(IntSlice::shrink(vec![0, 0]), &[0]);
        assert_eq!(IntSlice::shrink(vec![0, 0, 0]), &[0]);
        assert_eq!(IntSlice::shrink(vec![0, 1]), &[0, 1]);
        assert_eq!(IntSlice::shrink(vec![0, 1, 1]), &[0, 1, 1]);

        assert_eq!(IntSlice::shrink(vec![1]), &[1]);
        assert_eq!(IntSlice::shrink(vec![1, 1]), &[1, 1]);
        assert_eq!(IntSlice::shrink(vec![1, 1, 1]), &[1, 1, 1]);
        assert_eq!(IntSlice::shrink(vec![1, 0]), &[1]);
        assert_eq!(IntSlice::shrink(vec![1, 0, 0]), &[1]);

        assert_eq!(IntSlice::shrink(vec![1, 0, 1]), &[1, 0, 1]);
    }

    #[test]
    fn test_int_arr() {
        assert_eq!(&int_to_arr::<u16, usize, 1>(0), &[0]);
        assert_eq!(&int_to_arr::<u16, usize, 1>(1), &[1]);
        assert_eq!(&int_to_arr::<u16, usize, 1>(u16::MAX), &[u16::MAX as usize]);

        assert_eq!(&int_to_arr::<u128, usize, 2>(0), &[0; 2]);
        assert_eq!(&int_to_arr::<u128, usize, 2>(1), &[1, 0]);
        assert_eq!(
            &int_to_arr::<u128, usize, 2>(usize::MAX as u128 + 1),
            &[0, 1]
        );
        assert_eq!(
            &int_to_arr::<u128, usize, 2>(u128::MAX),
            &[usize::MAX, usize::MAX]
        );

        assert_eq!(
            &int_to_arr::<u128, u8, 16>(0x0102_0304_0506_0708_090A_0B0C_0D0E_0F00),
            &[0, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
        );
    }

    #[test]
    fn test_arr_int() {
        assert_eq!(arr_to_int::<usize, u8>(&[0]), Some(0));
        assert_eq!(arr_to_int::<usize, u8>(&[1]), Some(1));
        assert_eq!(arr_to_int::<usize, u8>(&[usize::MAX]), None);

        assert_eq!(arr_to_int::<usize, u128>(&[0]), Some(0));
        assert_eq!(arr_to_int::<usize, u128>(&[1]), Some(1));
        assert_eq!(
            arr_to_int::<usize, u128>(&[usize::MAX]),
            Some(usize::MAX as u128)
        );

        assert_eq!(arr_to_int::<u8, usize>(&[0, 1, 2, 3]), Some(0x0302_0100));
        assert_eq!(arr_to_int::<u8, usize>(&[u8::MAX]), Some(255));
    }
}
