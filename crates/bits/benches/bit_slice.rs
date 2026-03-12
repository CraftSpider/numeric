#![allow(missing_docs, clippy::missing_panics_doc)]

use core::cmp::Ordering;
use core::hint::black_box;
use criterion::measurement::{Measurement, WallTime};
use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkGroup, BenchmarkId, Criterion,
    PlotConfiguration,
};
use numeric_bench_util::make_criterion;
use numeric_bits::algos::{
    Add, Algo, AlgoTy, AssignAlgo, Bitwise, CmpAlgo, Div, DivRem, Element, Mul, NewtonRaphson, Rem,
    Shl, Shr, Sub,
};
use numeric_bits::bit_slice::BitOwned;
use std::time::Duration;

const ONE: &[usize] = &[1usize];
const MAX: &[usize] = &[usize::MAX];
const MAX_2: &[usize] = &[usize::MAX; 2];
const MAX_4: &[usize] = &[usize::MAX; 4];
const MAX_8: &[usize] = &[usize::MAX; 8];
const MAX_16: &[usize] = &[usize::MAX; 16];
const MAX_32: &[usize] = &[usize::MAX; 32];

fn bench_output<A, B, O, M>(group: &mut BenchmarkGroup<'_, M>, name: &str)
where
    A: for<'a> AlgoTy<In<'a, [usize]> = &'a [usize]>,
    B: Algo<A>,
    O: BitOwned<Bit = usize>,
    M: Measurement,
{
    group
        .bench_function(BenchmarkId::new(name, "[1], [1]"), |b| {
            b.iter(|| B::wrapping::<O, _, [_]>(black_box(ONE), black_box(ONE)))
        })
        .bench_function(BenchmarkId::new(name, "[1], [usize::MAX]"), |b| {
            b.iter(|| B::wrapping::<O, _, [_]>(black_box(ONE), black_box(MAX)))
        })
        .bench_function(BenchmarkId::new(name, "[usize::MAX], [1]"), |b| {
            b.iter(|| B::wrapping::<O, _, [_]>(black_box(MAX), black_box(ONE)))
        })
        .bench_function(BenchmarkId::new(name, "[usize::MAX], [usize::MAX]"), |b| {
            b.iter(|| B::wrapping::<O, _, [_]>(black_box(MAX), black_box(MAX)))
        });
}

fn bench_output_scale<A, B, M>(group: &mut BenchmarkGroup<'_, M>, name: &str)
where
    A: for<'a> AlgoTy<In<'a, [usize]> = &'a [usize]>,
    B: Algo<A>,
    M: Measurement,
{
    group
        .bench_function(BenchmarkId::new(name, "[usize::MAX], [usize::MAX]"), |b| {
            b.iter(|| B::wrapping::<[usize; 1], _, [_]>(black_box(MAX), black_box(MAX)))
        })
        .bench_function(
            BenchmarkId::new(name, "[usize::MAX; 2], [usize::MAX; 2]"),
            |b| b.iter(|| B::wrapping::<[usize; 2], _, [_]>(black_box(MAX_2), black_box(MAX_2))),
        )
        .bench_function(
            BenchmarkId::new(name, "[usize::MAX; 4], [usize::MAX; 4]"),
            |b| b.iter(|| B::wrapping::<[usize; 4], _, [_]>(black_box(MAX_4), black_box(MAX_4))),
        )
        .bench_function(
            BenchmarkId::new(name, "[usize::MAX; 8], [usize::MAX; 8]"),
            |b| b.iter(|| B::wrapping::<[usize; 8], _, [_]>(black_box(MAX_8), black_box(MAX_8))),
        )
        .bench_function(
            BenchmarkId::new(name, "[usize::MAX; 16], [usize::MAX; 16]"),
            |b| b.iter(|| B::wrapping::<[usize; 16], _, [_]>(black_box(MAX_16), black_box(MAX_16))),
        )
        .bench_function(
            BenchmarkId::new(name, "[usize::MAX; 32], [usize::MAX; 32]"),
            |b| b.iter(|| B::wrapping::<[usize; 32], _, [_]>(black_box(MAX_32), black_box(MAX_32))),
        );
}

pub fn bench_common<A, B, M>(c: &mut Criterion<M>, name: &str, algo: &str)
where
    A: for<'a> AlgoTy<In<'a, [usize]> = &'a [usize]>,
    B: Algo<A>,
    M: Measurement,
{
    let mut group = c.benchmark_group(format!("Algo<{algo}>"));

    let long_name = format!("{name}::wrapping::<Vec<usize>>");
    bench_output::<A, B, Vec<_>, M>(&mut group, &long_name);
    let array1_name = format!("{name}::wrapping::<[usize; 1]>");
    bench_output::<A, B, [usize; 1], M>(&mut group, &array1_name);

    drop(group);
    let mut group = c.benchmark_group(format!("Algo<{}> scaled", algo));
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let array_name = format!("{name}::wrapping::<[usize; N]>");
    bench_output_scale::<A, B, M>(&mut group, &array_name);
}

fn bench_assign<A, B, O, M>(group: &mut BenchmarkGroup<'_, M>, name: &str)
where
    A: for<'a> AlgoTy<In<'a, [usize]> = &'a [usize]>,
    B: AssignAlgo<A>,
    O: BitOwned<Bit = usize>,
    M: Measurement,
{
    group
        .bench_function(BenchmarkId::new(name, "[1], [1]"), |b| {
            b.iter(|| {
                let mut left = [1];
                B::wrapping::<O, _, [_]>(black_box(&mut left), black_box(ONE));
            })
        })
        .bench_function(BenchmarkId::new(name, "[1], [usize::MAX]"), |b| {
            b.iter(|| {
                let mut left = [1];
                B::wrapping::<O, _, [_]>(black_box(&mut left), black_box(MAX))
            })
        })
        .bench_function(BenchmarkId::new(name, "[usize::MAX], [1]"), |b| {
            b.iter(|| {
                let mut left = [usize::MAX];
                B::wrapping::<O, _, [_]>(black_box(&mut left), black_box(ONE))
            })
        })
        .bench_function(BenchmarkId::new(name, "[usize::MAX], [usize::MAX]"), |b| {
            b.iter(|| {
                let mut left = [usize::MAX];
                B::wrapping::<O, _, [_]>(black_box(&mut left), black_box(MAX));
            })
        });
}

fn bench_assign_scale<A, B, M>(group: &mut BenchmarkGroup<'_, M>, name: &str)
where
    A: for<'a> AlgoTy<In<'a, [usize]> = &'a [usize]>,
    B: AssignAlgo<A>,
    M: Measurement,
{
    group
        .bench_function(BenchmarkId::new(name, "[usize::MAX], [usize::MAX]"), |b| {
            b.iter(|| {
                let mut left = [usize::MAX];
                B::wrapping::<[usize; 1], _, [_]>(black_box(&mut left), black_box(MAX));
            })
        })
        .bench_function(
            BenchmarkId::new(name, "[usize::MAX; 2], [usize::MAX; 2]"),
            |b| {
                b.iter(|| {
                    let mut left = [usize::MAX; 2];
                    B::wrapping::<[usize; 2], _, [_]>(black_box(&mut left), black_box(MAX_2));
                })
            },
        )
        .bench_function(
            BenchmarkId::new(name, "[usize::MAX; 4], [usize::MAX; 4]"),
            |b| {
                b.iter(|| {
                    let mut left = [usize::MAX; 4];
                    B::wrapping::<[usize; 4], _, [_]>(black_box(&mut left), black_box(MAX_4));
                })
            },
        )
        .bench_function(
            BenchmarkId::new(name, "[usize::MAX; 8], [usize::MAX; 8]"),
            |b| {
                b.iter(|| {
                    let mut left = [usize::MAX; 8];
                    B::wrapping::<[usize; 8], _, [_]>(black_box(&mut left), black_box(MAX_8));
                })
            },
        )
        .bench_function(
            BenchmarkId::new(name, "[usize::MAX; 16], [usize::MAX; 16]"),
            |b| {
                b.iter(|| {
                    let mut left = [usize::MAX; 16];
                    B::wrapping::<[usize; 16], _, [_]>(black_box(&mut left), black_box(MAX_16));
                })
            },
        )
        .bench_function(
            BenchmarkId::new(name, "[usize::MAX; 32], [usize::MAX; 32]"),
            |b| {
                b.iter(|| {
                    let mut left = [usize::MAX; 32];
                    B::wrapping::<[usize; 32], _, [_]>(black_box(&mut left), black_box(MAX_32));
                })
            },
        );
}

pub fn bench_common_assign<A, B, M>(c: &mut Criterion<M>, name: &str, algo: &str)
where
    A: for<'a> AlgoTy<In<'a, [usize]> = &'a [usize]>,
    B: AssignAlgo<A>,
    M: Measurement,
{
    let mut group = c.benchmark_group(format!("AssignAlgo<{}>", algo));

    let long_name = format!("{name}::wrapping::<Vec<usize>>");
    bench_assign::<A, B, Vec<_>, M>(&mut group, &long_name);
    let array1_name = format!("{name}::wrapping::<[usize; 1]>");
    bench_assign::<A, B, [usize; 1], M>(&mut group, &array1_name);

    drop(group);
    let mut group = c.benchmark_group(format!("AssignAlgo<{}> scaled", algo));
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let array_name = format!("{name}::wrapping::<[usize; N]>");
    bench_assign_scale::<A, B, M>(&mut group, &array_name);
}

pub fn bench_cmp(c: &mut Criterion) {
    let one = &[1usize];
    let two = &[2usize];

    let long = &[0usize, 0, 0, 0, 0, 0, 0, 1];
    let long2 = &[0usize, 0, 0, 0, 0, 0, 0, 2];

    let cmp_name = "Element::cmp";

    c.benchmark_group("CmpAlgo")
        .bench_function(BenchmarkId::new(cmp_name, "[1], [1]"), |b| {
            b.iter(|| {
                assert_eq!(
                    black_box(<Element as CmpAlgo>::cmp(black_box(one), black_box(one))),
                    Ordering::Equal,
                )
            })
        })
        .bench_function(BenchmarkId::new(cmp_name, "[1], [2]"), |b| {
            b.iter(|| {
                assert_eq!(
                    black_box(<Element as CmpAlgo>::cmp(black_box(one), black_box(two))),
                    Ordering::Less,
                )
            })
        })
        .bench_function(BenchmarkId::new(cmp_name, "[2], [1]"), |b| {
            b.iter(|| {
                assert_eq!(
                    black_box(<Element as CmpAlgo>::cmp(black_box(two), black_box(one))),
                    Ordering::Greater,
                )
            })
        })
        .bench_function(BenchmarkId::new(cmp_name, "[0; 7, 1], [0; 7, 1]"), |b| {
            b.iter(|| {
                assert_eq!(
                    black_box(<Element as CmpAlgo>::cmp(black_box(long), black_box(long))),
                    Ordering::Equal,
                )
            })
        })
        .bench_function(BenchmarkId::new(cmp_name, "[0; 7, 1], [0; 7, 2]"), |b| {
            b.iter(|| {
                assert_eq!(
                    black_box(<Element as CmpAlgo>::cmp(black_box(long), black_box(long2))),
                    Ordering::Less,
                )
            })
        })
        .bench_function(BenchmarkId::new(cmp_name, "[0; 7, 2], [0; 7, 1]"), |b| {
            b.iter(|| {
                assert_eq!(
                    black_box(<Element as CmpAlgo>::cmp(black_box(long2), black_box(long))),
                    Ordering::Greater,
                )
            })
        });
}

pub fn bench_add(c: &mut Criterion) {
    bench_common::<Add, Element, _>(c, "Element", "Add");
    bench_common_assign::<Add, Element, _>(c, "Element", "Add");
    bench_common::<Add, Bitwise, _>(c, "Bitwise", "Add");
    bench_common_assign::<Add, Bitwise, _>(c, "Bitwise", "Add");
}

pub fn bench_sub(c: &mut Criterion) {
    bench_common::<Sub, Element, _>(c, "Element", "Sub");
    bench_common_assign::<Sub, Element, _>(c, "Element", "Sub");
    bench_common::<Sub, Bitwise, _>(c, "Bitwise", "Sub");
    bench_common_assign::<Sub, Bitwise, _>(c, "Bitwise", "Sub");
}

pub fn bench_mul(c: &mut Criterion) {
    bench_common::<Mul, Element, _>(c, "Element", "Mul");
    bench_common_assign::<Mul, Element, _>(c, "Element", "Mul");
    bench_common::<Mul, Bitwise, _>(c, "Bitwise", "Mul");
    bench_common_assign::<Mul, Bitwise, _>(c, "Bitwise", "Mul");
}

pub fn bench_div(c: &mut Criterion) {
    bench_common::<DivRem, Bitwise, _>(c, "Bitwise", "DivRem");
    // bench_common_assign::<DivRem, Bitwise, _>(c, "Bitwise", "DivRem");
    bench_common::<DivRem, NewtonRaphson, _>(c, "NewtonRaphson", "DivRem");
    bench_common_assign::<Div, NewtonRaphson, _>(c, "NewtonRaphson", "Div");
    bench_common_assign::<Rem, NewtonRaphson, _>(c, "NewtonRaphson", "Rem");
}

pub fn bench_shift<A, B>(c: &mut Criterion, name: &str, algo: &str)
where
    A: for<'a> AlgoTy<In<'a, [usize]> = usize>,
    B: Algo<A>,
{
    let mut group = c.benchmark_group(format!("Algo<{algo}>"));

    let long_name = format!("{name}::wrapping::<Vec<usize>>");
    let wrapping_name = format!("{name}::wrapping::<[usize; 1]>");

    group
        .bench_function(BenchmarkId::new(&long_name, "[1], 1"), |b| {
            b.iter(|| B::wrapping::<Vec<_>, _, [_]>(black_box(ONE), black_box(1)))
        })
        .bench_function(BenchmarkId::new(&long_name, "[usize::MAX], 1"), |b| {
            b.iter(|| B::wrapping::<Vec<_>, _, [_]>(black_box(MAX), black_box(1)))
        })
        .bench_function(BenchmarkId::new(&wrapping_name, "[1], 1"), |b| {
            b.iter(|| B::wrapping::<[usize; 1], _, [_]>(black_box(ONE), black_box(1)))
        })
        .bench_function(BenchmarkId::new(&wrapping_name, "[usize::MAX], 1"), |b| {
            b.iter(|| B::wrapping::<[usize; 1], _, [_]>(black_box(MAX), black_box(1)))
        });
}

pub fn bench_shift_assign<A, B>(c: &mut Criterion, name: &str, algo: &str)
where
    A: for<'a> AlgoTy<In<'a, [usize]> = usize>,
    B: AssignAlgo<A>,
{
    let mut group = c.benchmark_group(format!("AssignAlgo<{algo}>"));

    let wrapping1_name = format!("{name}::wrapping::<[usize; 1]>");
    let checked1_name = format!("{name}::checked::<[usize; 1]>");

    group
        .bench_function(BenchmarkId::new(&checked1_name, "[1], 1"), |b| {
            b.iter(|| {
                let mut one = [1usize];
                B::checked::<[usize; 1], _, [_]>(black_box(&mut one), black_box(1));
            })
        })
        .bench_function(BenchmarkId::new(&checked1_name, "[usize::MAX], 1"), |b| {
            b.iter(|| {
                let mut max = [usize::MAX];
                B::checked::<[usize; 1], _, [_]>(black_box(&mut max), black_box(1));
            })
        })
        .bench_function(BenchmarkId::new(&wrapping1_name, "[1], 1"), |b| {
            b.iter(|| {
                let mut one = [1usize];
                B::wrapping::<[usize; 1], _, [_]>(black_box(&mut one), black_box(1));
            })
        })
        .bench_function(BenchmarkId::new(&wrapping1_name, "[usize::MAX], 1"), |b| {
            b.iter(|| {
                let mut max = [usize::MAX];
                B::wrapping::<[usize; 1], _, [_]>(black_box(&mut max), black_box(1));
            })
        });
}

pub fn bench_shl(c: &mut Criterion) {
    bench_shift::<Shl, Bitwise>(c, "Bitwise", "Shl");
    bench_shift::<Shl, Element>(c, "Element", "Shl");
    bench_shift_assign::<Shl, Element>(c, "Element", "Shl");
}

pub fn bench_shr(c: &mut Criterion) {
    bench_shift::<Shr, Bitwise>(c, "Bitwise", "Shr");
    bench_shift::<Shr, Element>(c, "Element", "Shr");
    bench_shift_assign::<Shr, Element>(c, "Element", "Shr");
}

criterion_group!(
    name = benches;
    config = make_criterion()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3));
    targets = bench_shl, bench_add, bench_sub, bench_mul, bench_div, bench_cmp, bench_shr
);
criterion_main!(benches);
