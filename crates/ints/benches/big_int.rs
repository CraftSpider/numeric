use core::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use numeric_bench_util::make_criterion;
use numeric_ints::IBig;
use numeric_traits::identity::One;
use numeric_traits::ops::Pow;

pub fn bench_from(c: &mut Criterion) {
    c.benchmark_group("<IBig as From<u*>>")
        .bench_function(BenchmarkId::new("From<u8>", "0"), |b| {
            b.iter(|| IBig::from(black_box(0u8)))
        })
        .bench_function(BenchmarkId::new("From<u8>", "MAX"), |b| {
            b.iter(|| IBig::from(black_box(u8::MAX)))
        })
        .bench_function(BenchmarkId::new("From<u16>", "0"), |b| {
            b.iter(|| IBig::from(black_box(0u16)))
        })
        .bench_function(BenchmarkId::new("From<u16>", "MAX"), |b| {
            b.iter(|| IBig::from(black_box(u16::MAX)))
        })
        .bench_function(BenchmarkId::new("From<u32>", "0"), |b| {
            b.iter(|| IBig::from(black_box(0u32)))
        })
        .bench_function(BenchmarkId::new("From<u32>", "MAX"), |b| {
            b.iter(|| IBig::from(black_box(u32::MAX)))
        })
        .bench_function(BenchmarkId::new("From<u64>", "0"), |b| {
            b.iter(|| IBig::from(black_box(0u64)))
        })
        .bench_function(BenchmarkId::new("From<u64>", "MAX"), |b| {
            b.iter(|| IBig::from(black_box(u64::MAX)))
        })
        .bench_function(BenchmarkId::new("From<u128>", "0"), |b| {
            b.iter(|| IBig::from(black_box(0u128)))
        })
        .bench_function(BenchmarkId::new("From<u128>", "MAX"), |b| {
            b.iter(|| IBig::from(black_box(u128::MAX)))
        })
        .bench_function(BenchmarkId::new("From<usize>", "0"), |b| {
            b.iter(|| IBig::from(black_box(0usize)))
        })
        .bench_function(BenchmarkId::new("From<usize>", "MAX"), |b| {
            b.iter(|| IBig::from(black_box(usize::MAX)))
        });
}

pub fn bench_clone(c: &mut Criterion) {
    let zero = IBig::from(0);
    let max = IBig::from(usize::MAX);

    c.benchmark_group("IBig::clone")
        .bench_with_input(BenchmarkId::from_parameter("0"), &zero, |b, zero| {
            b.iter(|| black_box(zero).clone())
        })
        .bench_with_input(BenchmarkId::from_parameter("usize::MAX"), &max, |b, max| {
            b.iter(|| black_box(max).clone())
        });
}

pub fn bench_add(c: &mut Criterion) {
    let one = IBig::from(1);
    let max = IBig::from(usize::MAX);

    c.benchmark_group("IBig::add")
        .bench_with_input(BenchmarkId::from_parameter("1, 1"), &one, |b, one| {
            b.iter(|| black_box(one) + black_box(one))
        })
        .bench_with_input(
            BenchmarkId::from_parameter("usize::MAX, usize::MAX"),
            &max,
            |b, max| b.iter(|| black_box(max) + black_box(max)),
        );
}

pub fn bench_sub(c: &mut Criterion) {
    let one = IBig::from(1);
    let max = IBig::from(usize::MAX);

    c.benchmark_group("IBig::sub")
        .bench_with_input(BenchmarkId::from_parameter("1, 1"), &one, |b, one| {
            b.iter(|| black_box(one) - black_box(one))
        })
        .bench_with_input(
            BenchmarkId::from_parameter("usize::MAX, usize::MAX"),
            &max,
            |b, max| b.iter(|| black_box(max) - black_box(max)),
        );
}

pub fn bench_mul(c: &mut Criterion) {
    let one = IBig::from(1);
    let max = IBig::from(usize::MAX);

    c.benchmark_group("IBig::mul")
        .bench_with_input(BenchmarkId::from_parameter("1, 1"), &one, |b, one| {
            b.iter(|| black_box(one) * black_box(one))
        })
        .bench_with_input(
            BenchmarkId::from_parameter("usize::MAX, usize::MAX"),
            &max,
            |b, max| b.iter(|| black_box(max) * black_box(max)),
        );
}

pub fn bench_div(c: &mut Criterion) {
    let one = IBig::from(1);
    let two = IBig::from(2);
    let max = IBig::from(usize::MAX);
    let really_big = max.clone().pow(IBig::from(2));

    c.benchmark_group("IBig::div")
        .bench_with_input(BenchmarkId::from_parameter("1, 1"), &one, |b, one| {
            b.iter(|| black_box(one) / black_box(one))
        })
        .bench_with_input(
            BenchmarkId::from_parameter("usize::MAX, usize::MAX"),
            &max,
            |b, max| b.iter(|| black_box(max) / black_box(max)),
        )
        .bench_with_input(
            BenchmarkId::from_parameter("usize::MAX^3, 2"),
            &(really_big, two),
            |b, (big, two)| b.iter(|| black_box(big) / black_box(two)),
        );
}

pub fn bench_shl(c: &mut Criterion) {
    let one = IBig::from(1);
    let max = IBig::from(usize::MAX);

    c.benchmark_group("IBig::shl")
        .bench_with_input(BenchmarkId::from_parameter("1, 1"), &one, |b, one| {
            b.iter(|| black_box(one) << black_box(one))
        })
        .bench_with_input(
            BenchmarkId::from_parameter("usize::MAX, 1"),
            &max,
            |b, max| b.iter(|| black_box(max) << black_box(&one)),
        );
}

pub fn bench_stress(c: &mut Criterion) {
    let max = IBig::from(usize::MAX);
    let mut vec = Vec::new();
    let mut cur = max;
    for _ in 0..1000 {
        vec.push(cur.clone());
        cur += IBig::one();
    }

    let one = IBig::one();
    let r = vec.last().unwrap();

    c.benchmark_group("IBig stress").bench_with_input(
        BenchmarkId::new("add", "<large>, 1"),
        &(one, r),
        |b, (one, r)| b.iter(|| black_box(*r) + black_box(one)),
    );
}

criterion_group!(
    name = benches;
    config = make_criterion();
    targets = bench_from, bench_clone, bench_add, bench_sub, bench_mul, bench_div, bench_shl, bench_stress
);
criterion_main!(benches);
