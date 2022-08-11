
use numeric::utils::{shrink_slice, int_to_arr};
use criterion::{criterion_main, criterion_group, Criterion, black_box};
use pprof::criterion::{PProfProfiler, Output};

fn bench_int_arr(c: &mut Criterion) {
    c.bench_function("int_to_arr::<u128, usize>(0)", |b| {
        b.iter(|| int_to_arr::<u128, usize>(black_box(0)))
    });

    c.bench_function("int_to_arr::<u128, usize>(u128::MAX)", |b| {
        b.iter(|| int_to_arr::<u128, usize>(black_box(u128::MAX)))
    });
}

fn bench_shrink_slice(c: &mut Criterion) {
    c.bench_function("shrink_slice::<usize>([0; 16])", |b| {
        let slice = &[0; 16];
        b.iter(|| shrink_slice::<usize>(black_box(slice)))
    });

    c.bench_function("shrink_slice::<usize>([1, 0; 15])", |b| {
        let slice = &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        b.iter(|| shrink_slice::<usize>(black_box(slice)))
    });

    c.bench_function("shrink_slice::<usize>([0; 15, 1])", |b| {
        let slice = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        b.iter(|| shrink_slice::<usize>(black_box(slice)))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_int_arr, bench_shrink_slice
);
criterion_main!(benches);
