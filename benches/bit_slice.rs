
use numeric::bit_slice::BitSlice;
use criterion::{criterion_main, criterion_group, Criterion, black_box};
use pprof::criterion::{PProfProfiler, Output};

pub fn bench_shl(c: &mut Criterion) {
    c.bench_function("BitSlice::shl_bitwise(&[1], 1)", |b| {
        let i = BitSlice::new(&[1]);
        b.iter(|| BitSlice::shl_bitwise(black_box(i.clone()), black_box(1)))
    });

    c.bench_function("BitSlice::shl_bitwise(&[usize::MAX], 1)", |b| {
        let i = BitSlice::new(&[usize::MAX]);
        b.iter(|| BitSlice::shl_bitwise(black_box(i.clone()), black_box(1)))
    });

    c.bench_function("BitSlice::shl_wrap_and_mask(&[1], 1)", |b| {
        let i = BitSlice::new(&[1]);
        b.iter(|| BitSlice::shl_wrap_and_mask(black_box(i.clone()), black_box(1)))
    });

    c.bench_function("BitSlice::shl_wrap_and_mask(&[usize::MAX], 1)", |b| {
        let i = BitSlice::new(&[usize::MAX]);
        b.iter(|| BitSlice::shl_wrap_and_mask(black_box(i.clone()), black_box(1)))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_shl
);
criterion_main!(benches);
