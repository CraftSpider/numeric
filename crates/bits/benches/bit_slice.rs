use core::cmp::Ordering;
use core::hint::black_box;
use criterion::measurement::WallTime;
use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkGroup, BenchmarkId, Criterion,
    PlotConfiguration,
};
use numeric_bench_util::make_criterion;
use numeric_bits::algos::{
    AddAlgo, AssignAddAlgo, AssignDivRemAlgo, AssignMulAlgo, AssignShlAlgo, AssignSubAlgo, Bitwise,
    CmpAlgo, DivRemAlgo, Element, MulAlgo, ShlAlgo, SubAlgo,
};
use std::time::Duration;

type LongFn = fn(&[usize], &[usize]) -> Vec<usize>;
type CheckedFn = for<'a> fn(&'a mut [usize], &[usize]) -> Option<()>;
type WrappingFn = for<'a> fn(&'a mut [usize], &[usize]);

pub struct MathMeths {
    tr: &'static str,

    long_elem: Option<LongFn>,
    long_bit: Option<LongFn>,

    checked_elem: Option<CheckedFn>,
    checked_bit: Option<CheckedFn>,

    wrapping_elem: Option<WrappingFn>,
    wrapping_bit: Option<WrappingFn>,
}

const ONE: &[usize] = &[1usize];
const MAX: &[usize] = &[usize::MAX];
const MAX_2: &[usize] = &[usize::MAX; 2];
const MAX_4: &[usize] = &[usize::MAX; 4];
const MAX_8: &[usize] = &[usize::MAX; 8];
const MAX_16: &[usize] = &[usize::MAX; 16];
const MAX_32: &[usize] = &[usize::MAX; 32];

pub fn bench_common(c: &mut Criterion, meth: MathMeths) {
    let long_elem_name = format!("<Element as {}Algo>::long", meth.tr);
    let long_bit_name = format!("<Bitwise as {}Algo>::long", meth.tr);
    let elem_wrapping_name = format!("<Element as {}Algo>::wrapping", meth.tr);
    let bit_wrapping_name = format!("<Bitwise as {}Algo>::wrapping", meth.tr);
    let elem_checked_name = format!("<Element as {}Algo>::checked", meth.tr);
    let bit_checked_name = format!("<Bitwise as {}Algo>::checked", meth.tr);

    fn bench_long(group: &mut BenchmarkGroup<WallTime>, name: &str, f: LongFn) {
        group
            .bench_function(BenchmarkId::new(name, "[1], [1]"), |b| {
                b.iter(|| f(black_box(ONE), black_box(ONE)))
            })
            .bench_function(BenchmarkId::new(name, "[1], [usize::MAX]"), |b| {
                b.iter(|| f(black_box(ONE), black_box(MAX)))
            })
            .bench_function(BenchmarkId::new(name, "[usize::MAX], [1]"), |b| {
                b.iter(|| f(black_box(MAX), black_box(ONE)))
            })
            .bench_function(BenchmarkId::new(name, "[usize::MAX], [usize::MAX]"), |b| {
                b.iter(|| f(black_box(MAX), black_box(MAX)))
            });
    }

    fn bench_assign<T>(
        group: &mut BenchmarkGroup<WallTime>,
        name: &str,
        f: for<'a> fn(&mut [usize], &[usize]) -> T,
    ) {
        group
            .bench_function(BenchmarkId::new(name, "[1], [1]"), |b| {
                b.iter(|| {
                    let mut left = [1];
                    f(black_box(&mut left), black_box(ONE));
                })
            })
            .bench_function(BenchmarkId::new(name, "[1], [usize::MAX]"), |b| {
                b.iter(|| {
                    let mut left = [1];
                    f(black_box(&mut left), black_box(MAX))
                })
            })
            .bench_function(BenchmarkId::new(name, "[usize::MAX], [1]"), |b| {
                b.iter(|| {
                    let mut left = [usize::MAX];
                    f(black_box(&mut left), black_box(ONE))
                })
            })
            .bench_function(BenchmarkId::new(name, "[usize::MAX], [usize::MAX]"), |b| {
                b.iter(|| {
                    let mut left = [usize::MAX];
                    f(black_box(&mut left), black_box(MAX));
                })
            });
    }

    let mut group = c.benchmark_group(format!("{}Algo", meth.tr));

    if let Some(f) = meth.long_elem {
        bench_long(&mut group, &long_elem_name, f);
    }

    if let Some(f) = meth.long_bit {
        bench_long(&mut group, &long_bit_name, f);
    }

    if let Some(f) = meth.wrapping_elem {
        bench_assign(&mut group, &elem_wrapping_name, f);
    }

    if let Some(f) = meth.wrapping_bit {
        bench_assign(&mut group, &bit_wrapping_name, f);
    }

    if let Some(f) = meth.checked_elem {
        bench_assign(&mut group, &elem_checked_name, f);
    }

    if let Some(f) = meth.checked_bit {
        bench_assign(&mut group, &bit_checked_name, f);
    }

    fn bench_long_scale(group: &mut BenchmarkGroup<WallTime>, name: &str, f: LongFn) {
        group
            .bench_function(BenchmarkId::new(name, "[usize::MAX], [usize::MAX]"), |b| {
                b.iter(|| f(black_box(MAX), black_box(MAX)))
            })
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 2], [usize::MAX; 2]"),
                |b| b.iter(|| f(black_box(MAX_2), black_box(MAX_2))),
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 4], [usize::MAX; 4]"),
                |b| b.iter(|| f(black_box(MAX_4), black_box(MAX_4))),
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 8], [usize::MAX; 8]"),
                |b| b.iter(|| f(black_box(MAX_8), black_box(MAX_8))),
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 16], [usize::MAX; 16]"),
                |b| b.iter(|| f(black_box(MAX_16), black_box(MAX_16))),
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 32], [usize::MAX; 32]"),
                |b| b.iter(|| f(black_box(MAX_32), black_box(MAX_32))),
            );
    }

    fn bench_assign_scale<T>(
        group: &mut BenchmarkGroup<WallTime>,
        name: &str,
        f: for<'a> fn(&mut [usize], &[usize]) -> T,
    ) {
        group
            .bench_function(BenchmarkId::new(name, "[usize::MAX], [usize::MAX]"), |b| {
                b.iter(|| {
                    let mut left = [usize::MAX];
                    f(black_box(&mut left), black_box(MAX));
                })
            })
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 2], [usize::MAX; 2]"),
                |b| {
                    b.iter(|| {
                        let mut left = [usize::MAX; 2];
                        f(black_box(&mut left), black_box(MAX_2));
                    })
                },
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 4], [usize::MAX; 4]"),
                |b| {
                    b.iter(|| {
                        let mut left = [usize::MAX; 4];
                        f(black_box(&mut left), black_box(MAX_4));
                    })
                },
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 8], [usize::MAX; 8]"),
                |b| {
                    b.iter(|| {
                        let mut left = [usize::MAX; 8];
                        f(black_box(&mut left), black_box(MAX_8));
                    })
                },
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 16], [usize::MAX; 16]"),
                |b| {
                    b.iter(|| {
                        let mut left = [usize::MAX; 16];
                        f(black_box(&mut left), black_box(MAX_16));
                    })
                },
            )
            .bench_function(
                BenchmarkId::new(name, "[usize::MAX; 32], [usize::MAX; 32]"),
                |b| {
                    b.iter(|| {
                        let mut left = [usize::MAX; 32];
                        f(black_box(&mut left), black_box(MAX_32));
                    })
                },
            );
    }

    drop(group);
    let mut group = c.benchmark_group(format!("{}Algo scaled", meth.tr));
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    if let Some(f) = meth.long_elem {
        bench_long_scale(&mut group, &long_elem_name, f);
    }

    if let Some(f) = meth.long_bit {
        bench_long_scale(&mut group, &long_bit_name, f);
    }

    if let Some(f) = meth.wrapping_elem {
        bench_assign_scale(&mut group, &elem_wrapping_name, f);
    }

    if let Some(f) = meth.wrapping_bit {
        bench_assign_scale(&mut group, &bit_wrapping_name, f);
    }

    if let Some(f) = meth.checked_elem {
        bench_assign_scale(&mut group, &elem_checked_name, f);
    }

    if let Some(f) = meth.checked_bit {
        bench_assign_scale(&mut group, &bit_checked_name, f);
    }
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
    bench_common(
        c,
        MathMeths {
            tr: "Add",
            long_elem: Some(<Element as AddAlgo>::long),
            long_bit: Some(<Bitwise as AddAlgo>::long),
            checked_elem: Some(<Element as AssignAddAlgo>::checked),
            checked_bit: Some(<Bitwise as AssignAddAlgo>::checked),
            wrapping_elem: Some(<Element as AssignAddAlgo>::wrapping),
            wrapping_bit: Some(<Bitwise as AssignAddAlgo>::wrapping),
        },
    );
}

pub fn bench_sub(c: &mut Criterion) {
    bench_common(
        c,
        MathMeths {
            tr: "Sub",
            long_elem: Some(|l, r| <Element as SubAlgo>::long(l, r).0),
            long_bit: Some(|l, r| <Bitwise as SubAlgo>::long(l, r).0),
            checked_elem: Some(<Element as AssignSubAlgo>::checked),
            checked_bit: None, // Some(<Bitwise as AssignSubAlgo>::checked),
            wrapping_elem: Some(<Element as AssignSubAlgo>::wrapping),
            wrapping_bit: None, // Some(<Bitwise as AssignSubAlgo>::wrapping),
        },
    );
}

pub fn bench_mul(c: &mut Criterion) {
    bench_common(
        c,
        MathMeths {
            tr: "Mul",
            long_elem: Some(<Element as MulAlgo>::long),
            long_bit: Some(<Bitwise as MulAlgo>::long),
            checked_elem: Some(<Element as AssignMulAlgo>::checked),
            checked_bit: None,
            wrapping_elem: Some(<Element as AssignMulAlgo>::wrapping),
            wrapping_bit: None,
        },
    );
}

pub fn bench_div(c: &mut Criterion) {
    bench_common(
        c,
        MathMeths {
            tr: "Div",
            long_elem: None,
            long_bit: Some(|l, r| <Bitwise as DivRemAlgo>::long(l, r).0),
            checked_elem: None,
            checked_bit: Some(|l, r| <Bitwise as AssignDivRemAlgo>::div_checked(l, r, &mut [0; 8])),
            wrapping_elem: None,
            wrapping_bit: Some(|l, r| {
                <Bitwise as AssignDivRemAlgo>::div_wrapping(l, r, &mut [0; 8])
            }),
        },
    );
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
