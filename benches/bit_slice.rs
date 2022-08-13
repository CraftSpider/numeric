
use numeric::bit_slice::BitSlice;
use criterion::{criterion_main, criterion_group, Criterion, black_box, BenchmarkId};
use pprof::criterion::{PProfProfiler, Output};

pub fn bench_add(c: &mut Criterion) {
    let one = BitSlice::new([1usize]);
    let max = BitSlice::new([usize::MAX]);

    c.benchmark_group("BitSlice::add*")
        .bench_with_input(
            BenchmarkId::new("add_bitwise", "[1], [1]"),
            &one,
            |b, one| {
                b.iter(|| BitSlice::add_bitwise(black_box(one.clone()), black_box(one.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("add_bitwise", "[usize::MAX], [usize::MAX]"),
            &max,
            |b, max| {
                b.iter(|| BitSlice::add_bitwise(black_box(max.clone()), black_box(max.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("add_element", "[1], [1]"),
            &one,
            |b, one| {
                b.iter(|| BitSlice::add_element(black_box(one.clone()), black_box(one.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("add_element", "[usize::MAX], [usize::MAX]"),
            &max,
            |b, max| {
                b.iter(|| BitSlice::add_element(black_box(max.clone()), black_box(max.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("add_element_checked", "[1], [1]"),
            &one,
            |b, one| {
                b.iter(|| BitSlice::add_element_checked(black_box(one.clone()), black_box(one.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("add_element_checked", "[usize::MAX], [usize::MAX]"),
            &max,
            |b, max| {
                b.iter(|| BitSlice::add_element_checked(black_box(max.clone()), black_box(max.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("add_element_wrapping", "[1], [1]"),
            &one,
            |b, one| {
                b.iter(|| BitSlice::add_element_wrapping(black_box(one.clone()), black_box(one.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("add_element_wrapping", "[usize::MAX], [usize::MAX]"),
            &max,
            |b, max| {
                b.iter(|| BitSlice::add_element_wrapping(black_box(max.clone()), black_box(max.clone())))
            });
}

pub fn bench_sub(c: &mut Criterion) {
    let one = BitSlice::new([1usize]);
    let max = BitSlice::new([usize::MAX]);

    c.benchmark_group("BitSlice::sub*")
        .bench_with_input(
            BenchmarkId::new("sub_bitwise", "[1], [1]"),
            &one,
            |b, one| {
                b.iter(|| BitSlice::sub_bitwise(black_box(one.clone()), black_box(one.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("sub_bitwise", "[usize::MAX], [usize::MAX]"),
            &max,
            |b, max| {
                b.iter(|| BitSlice::sub_bitwise(black_box(max.clone()), black_box(max.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("sub_element", "[1], [1]"),
            &one,
            |b, one| {
                b.iter(|| BitSlice::sub_element(black_box(one.clone()), black_box(one.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("sub_element", "[usize::MAX], [usize::MAX]"),
            &max,
            |b, max| {
                b.iter(|| BitSlice::sub_element(black_box(max.clone()), black_box(max.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("sub_element_checked", "[1], [1]"),
            &one,
            |b, one| {
                b.iter(|| BitSlice::sub_element_checked(black_box(one.clone()), black_box(one.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("sub_element_checked", "[usize::MAX], [usize::MAX]"),
            &max,
            |b, max| {
                b.iter(|| BitSlice::sub_element_checked(black_box(max.clone()), black_box(max.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("sub_element_wrapping", "[1], [1]"),
            &one,
            |b, one| {
                b.iter(|| BitSlice::sub_element_wrapping(black_box(one.clone()), black_box(one.clone())))
            })
        .bench_with_input(
            BenchmarkId::new("sub_element_wrapping", "[usize::MAX], [usize::MAX]"),
            &max,
            |b, max| {
                b.iter(|| BitSlice::sub_element_wrapping(black_box(max.clone()), black_box(max.clone())))
            });
}

pub fn bench_shl(c: &mut Criterion) {
    let one = BitSlice::new([1usize]);
    let max = BitSlice::new([usize::MAX]);

    c.benchmark_group("BitSlice::shl*")
        .bench_with_input(
            BenchmarkId::new("shl_bitwise", "[1], 1"),
            &one,
            |b, one| {
                b.iter(|| BitSlice::shl_bitwise(black_box(one.clone()), black_box(1)))
            })
        .bench_with_input(
            BenchmarkId::new("shl_bitwise", "[usize::MAX], 1"),
            &max,
            |b, max| {
                b.iter(|| BitSlice::shl_bitwise(black_box(max.clone()), black_box(1)))
            })
        .bench_with_input(
            BenchmarkId::new("shl_wrap_and_mask", "[1], 1"),
            &one,
            |b, one| {
                b.iter(|| BitSlice::shl_wrap_and_mask(black_box(one.clone()), black_box(1)))
            })
        .bench_with_input(
            BenchmarkId::new("shl_wrap_and_mask", "[usize::MAX], 1"),
            &max,
            |b, max| {
                b.iter(|| BitSlice::shl_wrap_and_mask(black_box(max.clone()), black_box(1)))
            })
        .bench_with_input(
            BenchmarkId::new("shl_wrap_and_mask_checked", "[1], 1"),
            &one,
            |b, one| {
                b.iter(|| BitSlice::shl_wrap_and_mask_checked(black_box(one.clone()), black_box(1)))
            })
        .bench_with_input(
            BenchmarkId::new("shl_wrap_and_mask_checked", "[usize::MAX], 1"),
            &max,
            |b, max| {
                b.iter(|| BitSlice::shl_wrap_and_mask_checked(black_box(max.clone()), black_box(1)))
            })
        .bench_with_input(
            BenchmarkId::new("shl_wrap_and_mask_wrapping", "[1], 1"),
            &one,
            |b, one| {
                b.iter(|| BitSlice::shl_wrap_and_mask_wrapping(black_box(one.clone()), black_box(1)))
            })
        .bench_with_input(
            BenchmarkId::new("shl_wrap_and_mask_wrapping", "[usize::MAX], 1"),
            &max,
            |b, max| {
                b.iter(|| BitSlice::shl_wrap_and_mask_wrapping(black_box(max.clone()), black_box(1)))
            });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_shl, bench_add, bench_sub
);
criterion_main!(benches);
