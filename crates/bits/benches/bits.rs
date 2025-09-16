use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use numeric_bench_util::make_criterion;
use numeric_bits::bit_slice::BitSliceExt;

fn make_slice(len: usize) -> Vec<u16> {
    let slice = &[0b1010101010101010u16, 0b0101010101010101u16];
    slice.repeat(len / 2)
}

pub fn bench_bit_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("BitIter::sum");
    for len in [2, 4, 8, 16, 32] {
        group.bench_function(BenchmarkId::from_parameter(len), |b| {
            let slice = make_slice(len);
            b.iter(|| black_box(&slice).iter_bits().map(u32::from).sum::<u32>())
        });
    }
    drop(group);

    let mut group = c.benchmark_group("BitIter::all");
    for len in [2, 4, 8, 16, 32] {
        group.bench_function(BenchmarkId::from_parameter(len), |b| {
            let slice = make_slice(len);
            b.iter(|| {
                black_box(&slice).iter_bits().all(|b| {
                    black_box(b);
                    true
                })
            })
        });
    }
}

criterion_group!(
    name = benches;
    config = make_criterion();
    targets = bench_bit_iter,
);
criterion_main!(benches);
