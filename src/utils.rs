use num_traits::{PrimInt, Unsigned};

pub fn shrink_slice<T: PrimInt>(val: &[T]) -> &[T] {
    let idx = val.iter().rev().enumerate().find_map(|(idx, v)| if *v != T::zero() {
        Some(idx)
    } else {
        None
    });
    match idx {
        Some(idx) if idx != 0 => &val[..idx],
        _ => val,
    }
}

pub fn int_to_arr<T: PrimInt + Unsigned, U: PrimInt + Unsigned>(val: T) -> Vec<U> {
    match U::from(val) {
        Some(u) => vec![u],
        None => {
            let mut out = Vec::with_capacity(16);

            let max: T = T::from(U::max_value())
                .expect("Our value didn't fit in a usize - must be too big") + T::one();

            let mut left = val;
            loop {
                match U::from(left) {
                    Some(u) => {
                        out.push(u);
                        break;
                    }
                    None => {
                        left = left / max;
                        let rem: T = val % max;
                        out.push(U::from(rem).expect("Modulo U::max_value() + 1 - Will always be >= U::max_value()"))
                    }
                }
            }

            out
        }
    }
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
        assert_eq!(shrink_slice(&[0, 1]), &[0, 1]);
        assert_eq!(shrink_slice(&[1, 0]), &[1]);
        assert_eq!(shrink_slice(&[1]), &[1]);
    }

    #[test]
    fn test_int_arr() {
        assert_eq!(int_to_arr::<u16, usize>(0), vec![0]);
        assert_eq!(int_to_arr::<u16, usize>(1), vec![1]);
        assert_eq!(int_to_arr::<u16, usize>(u16::MAX), vec![u16::MAX as usize]);

        assert_eq!(int_to_arr::<u128, usize>(0), vec![0]);
        assert_eq!(int_to_arr::<u128, usize>(1), vec![1]);
        assert_eq!(int_to_arr::<u128, usize>(usize::MAX as u128 + 1), vec![0, 1]);
        assert_eq!(int_to_arr::<u128, usize>(u128::MAX), vec![usize::MAX, usize::MAX]);
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
