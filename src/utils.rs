use num_traits::{PrimInt, Unsigned};

pub fn shrink_slice<T: PrimInt>(val: &[T]) -> &[T] {
    let mut idx = 0;
    for i in (0..val.len()).rev() {
        // This ensures no bounds checks ever get generated
        // SAFETY: We iterate up to length
        if unsafe { *val.get_unchecked(i) } != T::zero() {
            idx = i;
            break;
        }
    }
    &val[..=idx]
}

pub fn shrink_vec<T: PrimInt>(mut val: Vec<T>) -> Vec<T> {
    while val.len() > 1 && val.last() == Some(&T::zero()) {
        val.pop();
    }
    val
}

// Length 16 chosen as it's the longest possible result for u8 -> u128
pub fn int_to_arr<T: PrimInt + Unsigned, U: PrimInt + Unsigned>(val: T) -> [U; 16] {
    let mut out = [U::zero(); 16];

    match U::from(val) {
        Some(u) => out[0] = u,
        None => {
            let max: T = T::from(U::max_value())
                .expect("Our value didn't fit in a T - must be too big") + T::one();

            let mut left = val;
            for item in &mut out {
                match U::from(left) {
                    Some(u) => {
                        *item = u;
                        break;
                    }
                    None => {
                        left = left / max;
                        let rem: T = val % max;
                        *item = U::from(rem).expect("Modulo U::max_value() + 1 - Will always be >= U::max_value()");
                    }
                }
            }
        }
    }

    out
}

pub fn arr_to_int<T: PrimInt, U: PrimInt>(arr: &[T]) -> Option<U> {
    let mut out = U::zero();
    for (idx, &i) in arr.iter().enumerate() {
        let t = U::from(i)?;
        out = out + (t << (idx * (usize::BITS as usize)));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shrink_slice() {
        assert_eq!(shrink_slice(&[0]), &[0]);
        assert_eq!(shrink_slice(&[0, 0]), &[0]);
        assert_eq!(shrink_slice(&[0, 0, 0]), &[0]);
        assert_eq!(shrink_slice(&[0, 1]), &[0, 1]);
        assert_eq!(shrink_slice(&[0, 1, 1]), &[0, 1, 1]);

        assert_eq!(shrink_slice(&[1]), &[1]);
        assert_eq!(shrink_slice(&[1, 1]), &[1, 1]);
        assert_eq!(shrink_slice(&[1, 1, 1]), &[1, 1, 1]);
        assert_eq!(shrink_slice(&[1, 0]), &[1]);
        assert_eq!(shrink_slice(&[1, 0, 0]), &[1]);
    }

    #[test]
    fn test_int_arr() {
        assert_eq!(&int_to_arr::<u16, usize>(0), &[0; 16]);
        assert_eq!(&int_to_arr::<u16, usize>(1), &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(&int_to_arr::<u16, usize>(u16::MAX), &[u16::MAX as usize, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        assert_eq!(&int_to_arr::<u128, usize>(0), &[0; 16]);
        assert_eq!(&int_to_arr::<u128, usize>(1), &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(&int_to_arr::<u128, usize>(usize::MAX as u128 + 1), &[0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(&int_to_arr::<u128, usize>(u128::MAX), &[usize::MAX, usize::MAX, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_arr_int() {
        assert_eq!(arr_to_int::<usize, u8>(&[0]), Some(0));
        assert_eq!(arr_to_int::<usize, u8>(&[1]), Some(1));
        assert_eq!(arr_to_int::<usize, u8>(&[usize::MAX]), None);

        assert_eq!(arr_to_int::<usize, u128>(&[0]), Some(0));
        assert_eq!(arr_to_int::<usize, u128>(&[1]), Some(1));
        assert_eq!(arr_to_int::<usize, u128>(&[usize::MAX]), Some(usize::MAX as u128));
    }
}
