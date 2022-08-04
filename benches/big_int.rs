
use numeric::big_int::BigInt;
use criterion::{criterion_main, criterion_group, Criterion, black_box};
use pprof::criterion::{PProfProfiler, Output};

pub fn bench_from(c: &mut Criterion) {
    c.bench_function("From<u8>::from(0)", |b| b.iter(|| BigInt::from(black_box(0u8))));
    c.bench_function("From<u8>::from(u8::MAX)", |b| b.iter(|| BigInt::from(black_box(u8::MAX))));

    c.bench_function("From<u16>::from(0)", |b| b.iter(|| BigInt::from(black_box(0u16))));
    c.bench_function("From<u16>::from(u16::MAX)", |b| b.iter(|| BigInt::from(black_box(u16::MAX))));

    c.bench_function("From<u32>::from(0)", |b| b.iter(|| BigInt::from(black_box(0u32))));
    c.bench_function("From<u32>::from(u32::MAX)", |b| b.iter(|| BigInt::from(black_box(u32::MAX))));

    c.bench_function("From<u64>::from(0)", |b| b.iter(|| BigInt::from(black_box(0u64))));
    c.bench_function("From<u64>::from(u64::MAX)", |b| b.iter(|| BigInt::from(black_box(u64::MAX))));

    c.bench_function("From<u128>::from(0)", |b| b.iter(|| BigInt::from(black_box(0u128))));
    c.bench_function("From<u128>::from(u128::MAX)", |b| b.iter(|| BigInt::from(black_box(u128::MAX))));
}

pub fn bench_clone(c: &mut Criterion) {
    c.bench_function("Clone::clone(inline)", |b| {
        let i = BigInt::from(0);
        b.iter(|| black_box(&i).clone())
    });

    c.bench_function("Clone::clone(intern)", |b| {
        let i = BigInt::from(usize::MAX);
        b.iter(|| black_box(&i).clone())
    });

    // Un-comment to check how we're doing against just an inline vector.
    // At start: We need like 128 items to be slower than interning
    c.bench_function("Clone::clone(vector)", |b| {
        let v = vec![0usize; 32];
        b.iter(|| black_box(&v).clone())
    });
}

pub fn bench_add(c: &mut Criterion) {
    c.bench_function("Add::add(1, 1)", |b| {
        let i = BigInt::from(1);
        b.iter(|| black_box(&i) + black_box(&i))
    });
    c.bench_function("Add::add(usize::MAX, usize::MAX)", |b| {
        let i = BigInt::from(usize::MAX);
        b.iter(|| black_box(&i) + black_box(&i))
    });
}

pub fn bench_sub(c: &mut Criterion) {
    c.bench_function("Sub::sub(1, 1)", |b| {
        let i = BigInt::from(1);
        b.iter(|| black_box(&i) - black_box(&i))
    });
    c.bench_function("Sub::sub(usize::MAX, usize::MAX)", |b| {
        let i = BigInt::from(usize::MAX);
        b.iter(|| black_box(&i) - black_box(&i))
    });
}

pub fn bench_mul(c: &mut Criterion) {
    c.bench_function("Mul::mul(1, 1)", |b| {
        let i = BigInt::from(1);
        b.iter(|| black_box(&i) * black_box(&i))
    });
    c.bench_function("Mul::mul(usize::MAX, usize::MAX)", |b| {
        let i = BigInt::from(usize::MAX);
        b.iter(|| black_box(&i) * black_box(&i))
    });
}

pub fn bench_div(c: &mut Criterion) {
    c.bench_function("Div::div(1, 1)", |b| {
        let i = BigInt::from(1);
        b.iter(|| black_box(&i) / black_box(&i))
    });
    c.bench_function("Div::div(usize::MAX, usize::MAX)", |b| {
        let i = BigInt::from(usize::MAX);
        b.iter(|| black_box(&i) / black_box(&i))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_from, bench_clone, bench_add, bench_sub
);
criterion_main!(benches);
