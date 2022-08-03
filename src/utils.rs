use num_traits::{PrimInt, Unsigned};

pub fn shrink_vec<T: PrimInt>(mut v: Vec<T>) -> Vec<T> {
    while v.len() > 1 && *v.last().unwrap() == T::zero() {
        v.pop();
    }
    v
}

pub fn int_to_arr<T: PrimInt + Unsigned, U: PrimInt + Unsigned>(val: T) -> Vec<U> {
    match U::from(val) {
        Some(u) => vec![u],
        None => {
            let max: T = T::from(U::max_value())
                .expect("Our value didn't fit in a usize - must be too big") + T::one();

            let l: T = val / max;
            let rem: T = val % max;

            let mut out = int_to_arr(l);
            out.push(U::from(rem).expect("Modulo U::max_value() + 1 - Will always be >= U::max_value()"));
            out
        }
    }
}

pub fn arr_to_int<T: PrimInt>(arr: &[usize]) -> T {
    let mut out = T::zero();
    for (idx, &i) in arr.iter().enumerate() {
        let t = T::from(i).unwrap();
        out = out + (t << (idx * (usize::BITS as usize)));
    }
    out
}
