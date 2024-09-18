use core::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use numeric_bench_util::make_criterion;
use numeric_bits::utils::{int_to_arr, IntSlice};

fn bench_int_arr(c: &mut Criterion) {
    c.benchmark_group("int_to_arr")
        .bench_function(BenchmarkId::new("<u128, usize>", "0"), |b| {
            b.iter(|| int_to_arr::<u128, usize, 4>(black_box(0)))
        })
        .bench_function(BenchmarkId::new("<u128, usize>", "u128::MAX"), |b| {
            b.iter(|| int_to_arr::<u128, usize, 4>(black_box(0)))
        })
        .bench_function(
            BenchmarkId::new("<u128, u8>", "0x0102030405060708090A0B0C0D0E0F00"),
            |b| {
                b.iter(|| int_to_arr::<u128, u8, 16>(black_box(0x0102030405060708090A0B0C0D0E0F00)))
            },
        );
}

fn bench_shrink_slice(c: &mut Criterion) {
    c.benchmark_group("IntSlice::shrink")
        .bench_function(BenchmarkId::new("<usize>", "[0; 16]"), |b| {
            b.iter(|| IntSlice::shrink(black_box(&[0usize; 16] as &[usize])))
        })
        .bench_function(BenchmarkId::new("<usize>", "[1, 0;15]"), |b| {
            b.iter(|| {
                IntSlice::shrink(black_box(
                    &[1usize, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] as &[usize],
                ))
            })
        })
        .bench_function(BenchmarkId::new("<usize>", "[0;15, 1]"), |b| {
            b.iter(|| {
                IntSlice::shrink(black_box(
                    &[0usize, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1] as &[usize],
                ))
            })
        })
        .bench_function(BenchmarkId::new("<usize>", "vec![0; 16]"), |b| {
            b.iter(|| IntSlice::shrink(black_box(vec![0usize; 16])))
        })
        .bench_function(BenchmarkId::new("<usize>", "vec![1, 0;15]"), |b| {
            b.iter(|| {
                IntSlice::shrink(black_box(vec![
                    1usize, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ]))
            })
        })
        .bench_function(BenchmarkId::new("<usize>", "vec![0;15, 1]"), |b| {
            b.iter(|| {
                IntSlice::shrink(black_box(vec![
                    0usize, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                ]))
            })
        });
}

criterion_group!(
    name = benches;
    config = make_criterion();
    targets = bench_int_arr, bench_shrink_slice
);
criterion_main!(benches);
