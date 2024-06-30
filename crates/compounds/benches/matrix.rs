use std::fmt::Debug;
use std::hint::black_box;
use std::array;
use std::ops::{Add, AddAssign, Mul};
use criterion::{BenchmarkGroup, BenchmarkId, Criterion, criterion_group, criterion_main};
use criterion::measurement::Measurement;
use numeric_bench_util::make_criterion;
use numeric_compounds::matrix::Matrix;
use numeric_traits::identity::{One, Zero};

fn make_matrices<T: Zero + One + Clone + AddAssign, const N: usize>() -> (Matrix<T, N, N>, Matrix<T, N, N>) {
    let mut cur = T::zero();
    let l = Matrix::new(array::from_fn(|_| array::from_fn(|_| {
        cur += T::one();
        cur.clone()
    })));
    let r = Matrix::new(array::from_fn(|_| array::from_fn(|_| {
        cur += T::one();
        cur.clone()
    })));
    (l, r)
}

macro_rules! multi_bench {
    (@ $group:ident $count:literal $ty:ty, $fn:ident) => {
        let (l, r) = make_matrices::<$ty, $count>();
        $fn(&mut $group, l, r);
    };
    ($c:ident, $ty:ty, $fn:ident, $group:literal) => {
        let mut group = $c.benchmark_group($group);
        multi_bench!(@ group 1 $ty, $fn);
        multi_bench!(@ group 2 $ty, $fn);
        multi_bench!(@ group 3 $ty, $fn);
        multi_bench!(@ group 4 $ty, $fn);
        multi_bench!(@ group 5 $ty, $fn);
        multi_bench!(@ group 6 $ty, $fn);
        multi_bench!(@ group 7 $ty, $fn);
        multi_bench!(@ group 8 $ty, $fn);
        multi_bench!(@ group 16 $ty, $fn);
        multi_bench!(@ group 24 $ty, $fn);
        multi_bench!(@ group 32 $ty, $fn);
        multi_bench!(@ group 40 $ty, $fn);
        multi_bench!(@ group 48 $ty, $fn);
        multi_bench!(@ group 56 $ty, $fn);
        multi_bench!(@ group 64 $ty, $fn);
        drop(group);
    };
}

fn inner_bench_add<M: Measurement, T: Add + Clone + Debug, const N: usize>(g: &mut BenchmarkGroup<M>, l: Matrix<T, N, N>, r: Matrix<T, N, N>) {
    g.bench_with_input(
        BenchmarkId::from_parameter(format!("{N}")),
        &(l, r),
        |b, (l, r)| b.iter(|| {
            black_box(l.clone()) + black_box(r.clone())
    }));
}

fn bench_add(c: &mut Criterion) {
    multi_bench!(c, u8, inner_bench_add, "Matrix<u8, _, _>::add");
    multi_bench!(c, i32, inner_bench_add, "Matrix<i32, _, _>::add");
    multi_bench!(c, f64, inner_bench_add, "Matrix<f64, _, _>::add");
}

fn inner_bench_mul<M: Measurement, T: Add<Output = T> + Mul<Output = T> + Clone + Zero + Debug, const N: usize>(g: &mut BenchmarkGroup<M>, l: Matrix<T, N, N>, r: Matrix<T, N, N>) {
    g.bench_with_input(
        BenchmarkId::from_parameter(format!("{N}")),
        &(l, r),
        |b, (l, r)| b.iter(|| {
        black_box(l.clone()) * black_box(r.clone())
    }));
}

fn bench_mul(c: &mut Criterion) {
    multi_bench!(c, u8, inner_bench_mul, "Matrix<u8, _, _>::mul");
    multi_bench!(c, i32, inner_bench_mul, "Matrix<i32, _, _>::mul");
    multi_bench!(c, f64, inner_bench_mul, "Matrix<f64, _, _>::mul");
}

criterion_group!(
    name = benches;
    config = make_criterion();
    targets = bench_add, bench_mul
);

criterion_main!(benches);
