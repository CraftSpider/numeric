
use numeric::utils::{shrink_slice, int_to_arr};
use criterion::{criterion_main, criterion_group, Criterion, black_box, BenchmarkId};
use pprof::criterion::{PProfProfiler, Output};

fn bench_int_arr(c: &mut Criterion) {
    c.benchmark_group("int_to_arr")
        .bench_function(
            BenchmarkId::new("<u128, usize>", "0"),
            |b| b.iter(|| int_to_arr::<u128, usize>(black_box(0)))
        )
        .bench_function(
            BenchmarkId::new("<u128, usize>", "u128::MAX"),
            |b| b.iter(|| int_to_arr::<u128, usize>(black_box(0)))
        );
}

fn bench_shrink_slice(c: &mut Criterion) {
    c.benchmark_group("shrink_slice")
        .bench_with_input(
            BenchmarkId::new("<usize>", "[0; 16]"),
            &[0; 16],
            |b, slice| b.iter(|| shrink_slice::<usize>(black_box(slice))),
        )
        .bench_with_input(
            BenchmarkId::new("<usize>", "[1, 0;15]"),
            &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            |b, slice| b.iter(|| shrink_slice::<usize>(black_box(slice))),
        )
        .bench_with_input(
            BenchmarkId::new("<usize>", "[0;15, 1]"),
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            |b, slice| b.iter(|| shrink_slice::<usize>(black_box(slice))),
        );
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_int_arr, bench_shrink_slice
);
criterion_main!(benches);
