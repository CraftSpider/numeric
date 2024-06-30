 use crate::*;

#[inline(never)]
pub fn zip_add<const N: usize>(a: [i32; N], b: [i32; N]) -> [i32; N] {
    a.into_static_iter().zip(b.into_static_iter())
        .map(|(l, r)| l + r)
        .collect()
}

#[inline(never)]
pub fn zip_mul<const N: usize>(a: [i32; N], b: [i32; N]) -> [i32; N] {
    a.into_static_iter().zip(b.into_static_iter())
        .map(|(l, r)| l * r)
        .collect()
}

#[inline(never)]
pub fn map_neg<const N: usize>(a: [i32; N]) -> [i32; N] {
    a.into_static_iter().map(|a| -a).collect()
}

#[inline(never)]
pub fn zip_many<const N: usize>(a: [i32; N], b: [i32; N], c: [i32; N], d: [i32; N]) -> [i32; N] {
    zip_all([a, b, c, d])
        .map(|[a, b, c, d]| a * b * c * d)
        .collect()
}

pub fn foo() {
    use core::hint::black_box;
    let a = black_box([1, 2, 3, 4, 5, 6, 7, 8]);
    let b = black_box([5, 6, 7, 8, 9, 10, 11, 12]);

    black_box(zip_add(a, b));
    black_box(zip_mul(a, b));
    black_box(map_neg(a));
    black_box(zip_many(a, b, a, b));
}
