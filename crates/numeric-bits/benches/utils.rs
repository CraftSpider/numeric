
use core::hint::black_box;
use numeric_core::utils::{int_to_arr, IntSlice};
use criterion::{criterion_main, criterion_group, Criterion, BenchmarkId};
use pprof::criterion::{PProfProfiler, Output};

fn bench_int_arr(c: &mut Criterion) {
    c.benchmark_group("int_to_arr")
        .bench_function(
            BenchmarkId::new("<u128, usize>", "0"),
            |b| b.iter(|| int_to_arr::<u128, usize, 4>(black_box(0)))
        )
        .bench_function(
            BenchmarkId::new("<u128, usize>", "u128::MAX"),
            |b| b.iter(|| int_to_arr::<u128, usize, 4>(black_box(0)))
        )
        .bench_function(
            BenchmarkId::new("<u128, u8>", "0x0102030405060708090A0B0C0D0E0F00"),
            |b| b.iter(|| int_to_arr::<u128, u8, 16>(black_box(0x0102030405060708090A0B0C0D0E0F00)))
        );
}

fn bench_shrink_slice(c: &mut Criterion) {
    c.benchmark_group("IntSlice::shrink")
        .bench_function(
            BenchmarkId::new("<usize>", "[0; 16]"),
            |b| b.iter(|| IntSlice::shrink(black_box(
                &[0; 16] as &[usize]
            ))),
        )
        .bench_function(
            BenchmarkId::new("<usize>", "[1, 0;15]"),
            |b| b.iter(|| IntSlice::shrink(black_box(
                &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] as &[usize]
            ))),
        )
        .bench_function(
            BenchmarkId::new("<usize>", "[0;15, 1]"),
            |b| b.iter(|| IntSlice::shrink(black_box(
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1] as &[usize]
            ))),
        );
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_int_arr, bench_shrink_slice
);
criterion_main!(benches);
