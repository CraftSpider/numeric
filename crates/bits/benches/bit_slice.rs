#![allow(missing_docs, clippy::missing_panics_doc)]

use core::cmp::Ordering;
use core::hint::black_box;
use criterion::measurement::WallTime;
use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkGroup, BenchmarkId, Criterion,
    PlotConfiguration,
};
use numeric_bench_util::make_criterion;
use numeric_bits::algos::{
    Add, Algo, AlgoTy, AssignAlgo, AssignShlAlgo, Bitwise, CmpAlgo, DivRem, Element, Mul,
    NewtonRaphson, ShlAlgo, Sub,
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

pub fn bench_common<A: AlgoTy, B: Algo<A>>(c: &mut Criterion, name: &str, algo: &str) {
    fn bench_output<A: AlgoTy, B: Algo<A>, O: BitOwned<Bit = usize>>(
        group: &mut BenchmarkGroup<'_, WallTime>,
        name: &str,
    ) {
        group
            .bench_function(BenchmarkId::new(name, "[1], [1]"), |b| {
                b.iter(|| B::wrapping::<O, _, _>(black_box(ONE), black_box(ONE)))
            })
            .bench_function(BenchmarkId::new(name, "[1], [usize::MAX]"), |b| {
                b.iter(|| B::wrapping::<O, _, _>(black_box(ONE), black_box(MAX)))
            })
            .bench_function(BenchmarkId::new(name, "[usize::MAX], [1]"), |b| {
                b.iter(|| B::wrapping::<O, _, _>(black_box(MAX), black_box(ONE)))
            })
            .bench_function(BenchmarkId::new(name, "[usize::MAX], [usize::MAX]"), |b| {
                b.iter(|| B::wrapping::<O, _, _>(black_box(MAX), black_box(MAX)))
            });
    }

    let mut group = c.benchmark_group(format!("Algo<{}>", algo));

    let long_name = format!("<{} as Algo<{}>>::wrapping::<Vec<usize>>", name, algo);
    bench_output::<A, B, Vec<_>>(&mut group, &long_name);
    let array1_name = format!("<{} as Algo<{}>>::wrapping::<[usize; 1]>", name, algo);
    bench_output::<A, B, [usize; 1]>(&mut group, &array1_name);

    fn bench_output_scale<A: AlgoTy, B: Algo<A>>(
        group: &mut BenchmarkGroup<'_, WallTime>,
        name: &str,
    ) {
        group
            .bench_function(BenchmarkId::new(name, "[usize::MAX], [usize::MAX]"), |b| {
                b.iter(|| B::wrapping::<[usize; 1], _, _>(black_box(MAX), black_box(MAX)))
            })
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 2], [usize::MAX; 2]"),
                |b| b.iter(|| B::wrapping::<[usize; 2], _, _>(black_box(MAX_2), black_box(MAX_2))),
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 4], [usize::MAX; 4]"),
                |b| b.iter(|| B::wrapping::<[usize; 4], _, _>(black_box(MAX_4), black_box(MAX_4))),
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 8], [usize::MAX; 8]"),
                |b| b.iter(|| B::wrapping::<[usize; 8], _, _>(black_box(MAX_8), black_box(MAX_8))),
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 16], [usize::MAX; 16]"),
                |b| {
                    b.iter(|| {
                        B::wrapping::<[usize; 16], _, _>(black_box(MAX_16), black_box(MAX_16))
                    })
                },
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 32], [usize::MAX; 32]"),
                |b| {
                    b.iter(|| {
                        B::wrapping::<[usize; 32], _, _>(black_box(MAX_32), black_box(MAX_32))
                    })
                },
            );
    }

    drop(group);
    let mut group = c.benchmark_group(format!("Algo<{}> scaled", algo));
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let array_name = format!("<{} as Algo<{}>>::wrapping::<[usize; N]>", name, algo);
    bench_output_scale::<A, B>(&mut group, &array_name);
}

pub fn bench_common_assign<A: AlgoTy, B: AssignAlgo<A>>(c: &mut Criterion, name: &str, algo: &str) {
    fn bench_assign<A: AlgoTy, B: AssignAlgo<A>, O: BitOwned<Bit = usize>>(
        group: &mut BenchmarkGroup<'_, WallTime>,
        name: &str,
    ) {
        group
            .bench_function(BenchmarkId::new(name, "[1], [1]"), |b| {
                b.iter(|| {
                    let mut left = [1];
                    B::wrapping::<O, _, _>(black_box(&mut left), black_box(ONE));
                })
            })
            .bench_function(BenchmarkId::new(name, "[1], [usize::MAX]"), |b| {
                b.iter(|| {
                    let mut left = [1];
                    B::wrapping::<O, _, _>(black_box(&mut left), black_box(MAX))
                })
            })
            .bench_function(BenchmarkId::new(name, "[usize::MAX], [1]"), |b| {
                b.iter(|| {
                    let mut left = [usize::MAX];
                    B::wrapping::<O, _, _>(black_box(&mut left), black_box(ONE))
                })
            })
            .bench_function(BenchmarkId::new(name, "[usize::MAX], [usize::MAX]"), |b| {
                b.iter(|| {
                    let mut left = [usize::MAX];
                    B::wrapping::<O, _, _>(black_box(&mut left), black_box(MAX));
                })
            });
    }

    fn bench_assign_scale<A: AlgoTy, B: AssignAlgo<A>>(
        group: &mut BenchmarkGroup<'_, WallTime>,
        name: &str,
    ) {
        group
            .bench_function(BenchmarkId::new(name, "[usize::MAX], [usize::MAX]"), |b| {
                b.iter(|| {
                    let mut left = [usize::MAX];
                    B::wrapping::<[usize; 1], _, _>(black_box(&mut left), black_box(MAX));
                })
            })
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 2], [usize::MAX; 2]"),
                |b| {
                    b.iter(|| {
                        let mut left = [usize::MAX; 2];
                        B::wrapping::<[usize; 2], _, _>(black_box(&mut left), black_box(MAX_2));
                    })
                },
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 4], [usize::MAX; 4]"),
                |b| {
                    b.iter(|| {
                        let mut left = [usize::MAX; 4];
                        B::wrapping::<[usize; 4], _, _>(black_box(&mut left), black_box(MAX_4));
                    })
                },
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 8], [usize::MAX; 8]"),
                |b| {
                    b.iter(|| {
                        let mut left = [usize::MAX; 8];
                        B::wrapping::<[usize; 8], _, _>(black_box(&mut left), black_box(MAX_8));
                    })
                },
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 16], [usize::MAX; 16]"),
                |b| {
                    b.iter(|| {
                        let mut left = [usize::MAX; 16];
                        B::wrapping::<[usize; 16], _, _>(black_box(&mut left), black_box(MAX_16));
                    })
                },
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 32], [usize::MAX; 32]"),
                |b| {
                    b.iter(|| {
                        let mut left = [usize::MAX; 32];
                        B::wrapping::<[usize; 32], _, _>(black_box(&mut left), black_box(MAX_32));
                    })
                },
            );
    }

    let mut group = c.benchmark_group(format!("AssignAlgo<{}>", algo));

    let long_name = format!("<{} as AssignAlgo<{}>>::wrapping::<Vec<usize>>", name, algo);
    bench_assign::<A, B, Vec<_>>(&mut group, &long_name);
    let array1_name = format!("<{} as AssignAlgo<{}>>::wrapping::<[usize; 1]>", name, algo);
    bench_assign::<A, B, [usize; 1]>(&mut group, &array1_name);

    drop(group);
    let mut group = c.benchmark_group(format!("AssignAlgo<{}> scaled", algo));
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let array_name = format!("<{} as AssignAlgo<{}>>::wrapping::<[usize; N]>", name, algo);
    bench_assign_scale::<A, B>(&mut group, &array_name);
}

pub fn bench_cmp(c: &mut Criterion) {
    let one = &[1usize];
    let two = &[2usize];

    let long = &[0usize, 0, 0, 0, 0, 0, 0, 1];
    let long2 = &[0usize, 0, 0, 0, 0, 0, 0, 2];

    c.benchmark_group("CmpAlgo")
        .bench_function(
            BenchmarkId::new("<Element as CmpAlgo>::cmp", "[1], [1]"),
            |b| {
                b.iter(|| {
                    assert_eq!(
                        black_box(<Element as CmpAlgo>::cmp(black_box(one), black_box(one))),
                        Ordering::Equal,
                    )
                })
            },
        )
        .bench_function(
            BenchmarkId::new("<Element as CmpAlgo>::cmp", "[1], [2]"),
            |b| {
                b.iter(|| {
                    assert_eq!(
                        black_box(<Element as CmpAlgo>::cmp(black_box(one), black_box(two))),
                        Ordering::Less,
                    )
                })
            },
        )
        .bench_function(
            BenchmarkId::new("<Element as CmpAlgo>::cmp", "[2], [1]"),
            |b| {
                b.iter(|| {
                    assert_eq!(
                        black_box(<Element as CmpAlgo>::cmp(black_box(two), black_box(one))),
                        Ordering::Greater,
                    )
                })
            },
        )
        .bench_function(
            BenchmarkId::new("<Element as CmpAlgo>::cmp", "[0; 7, 1], [0; 7, 1]"),
            |b| {
                b.iter(|| {
                    assert_eq!(
                        black_box(<Element as CmpAlgo>::cmp(black_box(long), black_box(long))),
                        Ordering::Equal,
                    )
                })
            },
        )
        .bench_function(
            BenchmarkId::new("<Element as CmpAlgo>::cmp", "[0; 7, 1], [0; 7, 2]"),
            |b| {
                b.iter(|| {
                    assert_eq!(
                        black_box(<Element as CmpAlgo>::cmp(black_box(long), black_box(long2))),
                        Ordering::Less,
                    )
                })
            },
        )
        .bench_function(
            BenchmarkId::new("<Element as CmpAlgo>::cmp", "[0; 7, 2], [0; 7, 1]"),
            |b| {
                b.iter(|| {
                    assert_eq!(
                        black_box(<Element as CmpAlgo>::cmp(black_box(long2), black_box(long))),
                        Ordering::Greater,
                    )
                })
            },
        );
}

pub fn bench_add(c: &mut Criterion) {
    bench_common::<Add, Element>(c, "Element", "Add");
    bench_common_assign::<Add, Element>(c, "Element", "Add");
    bench_common::<Add, Bitwise>(c, "Bitwise", "Add");
    bench_common_assign::<Add, Bitwise>(c, "Bitwise", "Add");
}

pub fn bench_sub(c: &mut Criterion) {
    bench_common::<Sub, Element>(c, "Element", "Sub");
    bench_common_assign::<Sub, Element>(c, "Element", "Sub");
    bench_common::<Sub, Bitwise>(c, "Bitwise", "Sub");
    bench_common_assign::<Sub, Bitwise>(c, "Bitwise", "Sub");
}

pub fn bench_mul(c: &mut Criterion) {
    bench_common::<Mul, Element>(c, "Element", "Mul");
    bench_common_assign::<Mul, Element>(c, "Element", "Mul");
    bench_common::<Mul, Bitwise>(c, "Bitwise", "Mul");
    bench_common_assign::<Mul, Bitwise>(c, "Bitwise", "Mul");
}

pub fn bench_div(c: &mut Criterion) {
    bench_common::<DivRem, Bitwise>(c, "Bitwise", "DivRem");
    // bench_common_assign::<DivRem, Bitwise>(c, "Bitwise", "DivRem");
    bench_common::<DivRem, NewtonRaphson>(c, "NewtonRaphson", "DivRem");
    // bench_common_assign::<DivRem, NewtonRaphson>(c, "NewtonRaphson", "DivRem");
}

pub fn bench_shl(c: &mut Criterion) {
    let one = &[1usize];
    let max = &[usize::MAX];

    c.benchmark_group("ShlAlgo")
        .bench_with_input(
            BenchmarkId::new("<Bitwise as ShlAlgo>::long", "[1], 1"),
            one,
            |b, one| b.iter(|| <Bitwise as ShlAlgo>::long(black_box(one), black_box(1))),
        )
        .bench_with_input(
            BenchmarkId::new("<Bitwise as ShlAlgo>::long", "[usize::MAX], 1"),
            max,
            |b, max| b.iter(|| <Bitwise as ShlAlgo>::long(black_box(max), black_box(1))),
        )
        .bench_with_input(
            BenchmarkId::new("<Element as ShlAlgo>::long", "[1], 1"),
            one,
            |b, one| b.iter(|| <Element as ShlAlgo>::long(black_box(one), black_box(1))),
        )
        .bench_with_input(
            BenchmarkId::new("<Element as ShlAlgo>::long", "[usize::MAX], 1"),
            max,
            |b, max| b.iter(|| <Element as ShlAlgo>::long(black_box(max), black_box(1))),
        )
        .bench_function(
            BenchmarkId::new("<Element as ShlAlgo>::checked", "[1], 1"),
            |b| {
                b.iter(|| {
                    let mut one = [1usize];
                    <Element as AssignShlAlgo>::checked(black_box(&mut one), black_box(1));
                })
            },
        )
        .bench_function(
            BenchmarkId::new("<Element as ShlAlgo>::checked", "[usize::MAX], 1"),
            |b| {
                b.iter(|| {
                    let mut max = [usize::MAX];
                    <Element as AssignShlAlgo>::checked(black_box(&mut max), black_box(1));
                })
            },
        )
        .bench_function(
            BenchmarkId::new("<Element as ShlAlgo>::wrapping", "[1], 1"),
            |b| {
                b.iter(|| {
                    let mut one = [1usize];
                    <Element as AssignShlAlgo>::wrapping(black_box(&mut one), black_box(1));
                })
            },
        )
        .bench_function(
            BenchmarkId::new("<Element as ShlAlgo>::wrapping", "[usize::MAX], 1"),
            |b| {
                b.iter(|| {
                    let mut max = [usize::MAX];
                    <Element as AssignShlAlgo>::wrapping(black_box(&mut max), black_box(1));
                })
            },
        );
}

criterion_group!(
    name = benches;
    config = make_criterion()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3));
    targets = bench_shl, bench_add, bench_sub, bench_mul, bench_div, bench_cmp
);
criterion_main!(benches);
