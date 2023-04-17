
use numeric::int::big_int::BigInt;
use criterion::{criterion_main, criterion_group, Criterion, black_box, BenchmarkId};
// use pprof::criterion::{PProfProfiler, Output};

pub fn bench_from(c: &mut Criterion) {
    c.benchmark_group("<BigInt as From<u*>>")
        .bench_function(
            BenchmarkId::new("From<u8>", "0"),
            |b| b.iter(|| BigInt::from(black_box(0u8)))
        )
        .bench_function(
            BenchmarkId::new("From<u8>", "MAX"),
            |b| b.iter(|| BigInt::from(black_box(u8::MAX)))
        )
        .bench_function(
            BenchmarkId::new("From<u16>", "0"),
            |b| b.iter(|| BigInt::from(black_box(0u16)))
        )
        .bench_function(
            BenchmarkId::new("From<u16>", "MAX"),
            |b| b.iter(|| BigInt::from(black_box(u16::MAX)))
        )
        .bench_function(
            BenchmarkId::new("From<u32>", "0"),
            |b| b.iter(|| BigInt::from(black_box(0u32)))
        )
        .bench_function(
            BenchmarkId::new("From<u32>", "MAX"),
            |b| b.iter(|| BigInt::from(black_box(u32::MAX)))
        )
        .bench_function(
            BenchmarkId::new("From<u64>", "0"),
            |b| b.iter(|| BigInt::from(black_box(0u64)))
        )
        .bench_function(
            BenchmarkId::new("From<u64>", "MAX"),
            |b| b.iter(|| BigInt::from(black_box(u64::MAX)))
        )
        .bench_function(
            BenchmarkId::new("From<u128>", "0"),
            |b| b.iter(|| BigInt::from(black_box(0u128)))
        )
        .bench_function(
            BenchmarkId::new("From<u128>", "MAX"),
            |b| b.iter(|| BigInt::from(black_box(u128::MAX)))
        )
        .bench_function(
            BenchmarkId::new("From<usize>", "0"),
            |b| b.iter(|| BigInt::from(black_box(0usize)))
        )
        .bench_function(
            BenchmarkId::new("From<usize>", "MAX"),
            |b| b.iter(|| BigInt::from(black_box(usize::MAX)))
        );
}

pub fn bench_clone(c: &mut Criterion) {
    let zero = BigInt::from(0);
    let max = BigInt::from(usize::MAX);

    c.benchmark_group("BigInt::clone")
        .bench_with_input(
            BenchmarkId::from_parameter("0"),
            &zero,
            |b, zero| b.iter(|| black_box(zero).clone())
        )
        .bench_with_input(
            BenchmarkId::from_parameter("usize::MAX"),
            &max,
            |b, max| b.iter(|| black_box(max).clone())
        );
}

pub fn bench_add(c: &mut Criterion) {
    let one = BigInt::from(1);
    let max = BigInt::from(usize::MAX);

    c.benchmark_group("BigInt::add")
        .bench_with_input(
            BenchmarkId::from_parameter("1, 1"),
            &one,
            |b, one| b.iter(|| black_box(one) + black_box(one))
        )
        .bench_with_input(
            BenchmarkId::from_parameter("usize::MAX, usize::MAX"),
            &max,
            |b, max| b.iter(|| black_box(max) + black_box(max))
        );
}

pub fn bench_sub(c: &mut Criterion) {
    let one = BigInt::from(1);
    let max = BigInt::from(usize::MAX);

    c.benchmark_group("BigInt::sub")
        .bench_with_input(
            BenchmarkId::from_parameter("1, 1"),
            &one,
            |b, one| b.iter(|| black_box(one) - black_box(one))
        )
        .bench_with_input(
            BenchmarkId::from_parameter("usize::MAX, usize::MAX"),
            &max,
            |b, max| b.iter(|| black_box(max) - black_box(max))
        );
}

pub fn bench_mul(c: &mut Criterion) {
    let one = BigInt::from(1);
    let max = BigInt::from(usize::MAX);

    c.benchmark_group("BigInt::mul")
        .bench_with_input(
            BenchmarkId::from_parameter("1, 1"),
            &one,
            |b, one| b.iter(|| black_box(one) * black_box(one))
        )
        .bench_with_input(
            BenchmarkId::from_parameter("usize::MAX, usize::MAX"),
            &max,
            |b, max| b.iter(|| black_box(max) * black_box(max))
        );
}

pub fn bench_div(c: &mut Criterion) {
    let one = BigInt::from(1);
    let max = BigInt::from(usize::MAX);

    c.benchmark_group("BigInt::div")
        .bench_with_input(
            BenchmarkId::from_parameter("1, 1"),
            &one,
            |b, one| b.iter(|| black_box(one) / black_box(one))
        )
        .bench_with_input(
            BenchmarkId::from_parameter("usize::MAX, usize::MAX"),
            &max,
            |b, max| b.iter(|| black_box(max) / black_box(max))
        );
}

pub fn bench_shl(c: &mut Criterion) {
    let one = BigInt::from(1);
    let max = BigInt::from(usize::MAX);

    c.benchmark_group("BigInt::mul")
        .bench_with_input(
            BenchmarkId::from_parameter("1, 1"),
            &one,
            |b, one| b.iter(|| black_box(one) << black_box(one))
        )
        .bench_with_input(
            BenchmarkId::from_parameter("usize::MAX, 1"),
            &max,
            |b, max| b.iter(|| black_box(max) << black_box(&one))
        );
}

criterion_group!(
    name = benches;
    config = Criterion::default(); // .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_from, bench_clone, bench_add, bench_sub, bench_mul, bench_div, bench_shl
);
criterion_main!(benches);
