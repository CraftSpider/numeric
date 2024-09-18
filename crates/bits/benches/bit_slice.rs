use core::cmp::Ordering;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use numeric_bench_util::make_criterion;
use numeric_bits::algos::{
    BitwiseAdd, BitwiseDiv, BitwiseShl, BitwiseSub, ElementAdd, ElementCmp, ElementDiv, ElementShl,
    ElementSub,
};

pub struct MathMeths {
    tr: &'static str,
    name: &'static str,
    big_bit: fn(&[usize], &[usize]) -> Vec<usize>,
    big_elem: fn(&[usize], &[usize]) -> Vec<usize>,
    checked_elem: for<'a> fn(&'a mut [usize], &[usize]) -> Option<&'a mut [usize]>,
    wrapping_elem: for<'a> fn(&'a mut [usize], &[usize]) -> &'a mut [usize],
}

pub fn bench_common(c: &mut Criterion, meth: MathMeths) {
    let one = &[1usize];
    let max = &[usize::MAX];
    let max_8 = &[usize::MAX; 8];

    let big_bit_name = format!("Bitwise{}::{}", meth.tr, meth.name);
    let big_elem_name = format!("Element{}::{}", meth.tr, meth.name);
    let elem_checked_name = format!("Element{}::{}_checked", meth.tr, meth.name);
    let elem_wrapping_name = format!("Element{}::{}_wrapping", meth.tr, meth.name);

    c.benchmark_group(format!("BitSliceExt::{}*", meth.name))
        .bench_function(BenchmarkId::new(&big_bit_name, "[1], [1]"), |b| {
            b.iter(|| (meth.big_bit)(black_box(one), black_box(one)))
        })
        .bench_function(BenchmarkId::new(&big_bit_name, "[1], [usize::MAX]"), |b| {
            b.iter(|| (meth.big_bit)(black_box(one), black_box(max)))
        })
        .bench_function(BenchmarkId::new(&big_bit_name, "[usize::MAX], [1]"), |b| {
            b.iter(|| (meth.big_bit)(black_box(max), black_box(one)))
        })
        .bench_function(
            BenchmarkId::new(&big_bit_name, "[usize::MAX], [usize::MAX]"),
            |b| b.iter(|| (meth.big_bit)(black_box(max), black_box(max))),
        )
        .bench_function(BenchmarkId::new(&big_elem_name, "[1], [1]"), |b| {
            b.iter(|| (meth.big_elem)(black_box(one), black_box(one)))
        })
        .bench_function(BenchmarkId::new(&big_elem_name, "[1], [usize::MAX]"), |b| {
            b.iter(|| (meth.big_elem)(black_box(one), black_box(max)))
        })
        .bench_function(BenchmarkId::new(&big_elem_name, "[usize::MAX], [1]"), |b| {
            b.iter(|| (meth.big_elem)(black_box(max), black_box(one)))
        })
        .bench_function(
            BenchmarkId::new(&big_elem_name, "[usize::MAX], [usize::MAX]"),
            |b| b.iter(|| (meth.big_elem)(black_box(max), black_box(max))),
        )
        .bench_function(
            BenchmarkId::new(&big_elem_name, "[usize::MAX; 8], [usize::MAX; 8]"),
            |b| b.iter(|| (meth.big_elem)(black_box(max_8), black_box(max_8))),
        )
        .bench_function(BenchmarkId::new(&elem_checked_name, "[1], [1]"), |b| {
            let mut left = [1];
            b.iter(|| {
                (meth.checked_elem)(black_box(&mut left), black_box(one));
            })
        })
        .bench_function(
            BenchmarkId::new(&elem_checked_name, "[usize::MAX], [usize::MAX]"),
            |b| {
                let mut left = [usize::MAX];
                b.iter(|| {
                    (meth.checked_elem)(black_box(&mut left), black_box(max));
                })
            },
        )
        .bench_function(BenchmarkId::new(&elem_wrapping_name, "[1], [1]"), |b| {
            let mut left = [1];
            b.iter(|| {
                (meth.wrapping_elem)(black_box(&mut left), black_box(one));
            })
        })
        .bench_function(
            BenchmarkId::new(&elem_wrapping_name, "[usize::MAX], [usize::MAX]"),
            |b| {
                let mut left = [usize::MAX];
                b.iter(|| {
                    (meth.wrapping_elem)(black_box(&mut left), black_box(max));
                })
            },
        );
}

pub fn bench_cmp(c: &mut Criterion) {
    let one = &[1usize];
    let two = &[2usize];

    let long = &[0usize, 0, 0, 0, 0, 0, 0, 1];
    let long2 = &[0usize, 0, 0, 0, 0, 0, 0, 2];

    c.benchmark_group("BitSliceExt::cmp*")
        .bench_function(BenchmarkId::new("ElementCmp::cmp", "[1], [1]"), |b| {
            b.iter(|| {
                assert_eq!(
                    black_box(ElementCmp::cmp(black_box(one), black_box(one))),
                    Ordering::Equal,
                )
            })
        })
        .bench_function(BenchmarkId::new("ElementCmp::cmp", "[1], [2]"), |b| {
            b.iter(|| {
                assert_eq!(
                    black_box(ElementCmp::cmp(black_box(one), black_box(two))),
                    Ordering::Less,
                )
            })
        })
        .bench_function(BenchmarkId::new("ElementCmp::cmp", "[2], [1]"), |b| {
            b.iter(|| {
                assert_eq!(
                    black_box(ElementCmp::cmp(black_box(two), black_box(one))),
                    Ordering::Greater,
                )
            })
        })
        .bench_function(
            BenchmarkId::new("ElementCmp::cmp", "[0; 7, 1], [0; 7, 1]"),
            |b| {
                b.iter(|| {
                    assert_eq!(
                        black_box(ElementCmp::cmp(black_box(long), black_box(long))),
                        Ordering::Equal,
                    )
                })
            },
        )
        .bench_function(
            BenchmarkId::new("ElementCmp::cmp", "[0; 7, 1], [0; 7, 2]"),
            |b| {
                b.iter(|| {
                    assert_eq!(
                        black_box(ElementCmp::cmp(black_box(long), black_box(long2))),
                        Ordering::Less,
                    )
                })
            },
        )
        .bench_function(
            BenchmarkId::new("ElementCmp::cmp", "[0; 7, 2], [0; 7, 1]"),
            |b| {
                b.iter(|| {
                    assert_eq!(
                        black_box(ElementCmp::cmp(black_box(long2), black_box(long))),
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
            name: "add",
            big_bit: BitwiseAdd::add,
            big_elem: ElementAdd::add,
            checked_elem: ElementAdd::add_checked,
            wrapping_elem: ElementAdd::add_wrapping,
        },
    );

    /*
    c.benchmark_group("BitSlice::add*")
        .bench_with_input(
            BenchmarkId::new("add_element_checked", "[1], [1]"),
            one,
            |b, one| {
                let mut left = [1];
                b.iter(|| ElementAdd::add_checked(black_box(&mut left), black_box(one)))
            })
        .bench_with_input(
            BenchmarkId::new("add_element_checked", "[usize::MAX], [usize::MAX]"),
            max,
            |b, max| {
                let mut left = [usize::MAX];
                b.iter(|| ElementAdd::add_checked(black_box(&mut left), black_box(max)))
            })
        .bench_with_input(
            BenchmarkId::new("add_element_wrapping", "[1], [1]"),
            one,
            |b, one| {
                let mut left = [1];
                b.iter(|| ElementAdd::add_wrapping(black_box(&mut left), black_box(one)))
            })
        .bench_with_input(
            BenchmarkId::new("add_element_wrapping", "[usize::MAX], [usize::MAX]"),
            max,
            |b, max| {
                let mut left = [usize::MAX];
                b.iter(|| ElementAdd::add_wrapping(black_box(&mut left), black_box(max)))
            });
     */
}

pub fn bench_sub(c: &mut Criterion) {
    bench_common(
        c,
        MathMeths {
            tr: "Sub",
            name: "sub",
            big_bit: |l, r| BitwiseSub::sub(l, r).0,
            big_elem: |l, r| ElementSub::sub(l, r).0,
            checked_elem: ElementSub::sub_checked,
            wrapping_elem: ElementSub::sub_wrapping,
        },
    );

    /*
    c.benchmark_group("BitSlice::sub*")
        .bench_with_input(
            BenchmarkId::new("sub_element_checked", "[1], [1]"),
            one,
            |b, one| {
                b.iter(|| ElementSub::sub_checked(black_box(&mut [1]), black_box(one)))
            })
        .bench_with_input(
            BenchmarkId::new("sub_element_checked", "[usize::MAX], [usize::MAX]"),
            max,
            |b, max| {
                b.iter(|| ElementSub::sub_checked(black_box(&mut [usize::MAX]), black_box(max)))
            })
        .bench_with_input(
            BenchmarkId::new("sub_element_wrapping", "[1], [1]"),
            one,
            |b, one| {
                b.iter(|| ElementSub::sub_wrapping(black_box(&mut [1]), black_box(one)))
            })
        .bench_with_input(
            BenchmarkId::new("sub_element_wrapping", "[usize::MAX], [usize::MAX]"),
            max,
            |b, max| {
                b.iter(|| ElementSub::sub_wrapping(black_box(&mut [usize::MAX]), black_box(max)))
            });
     */
}

pub fn bench_div(c: &mut Criterion) {
    bench_common(
        c,
        MathMeths {
            tr: "Div",
            name: "div",
            big_bit: |l, r| BitwiseDiv::div_long(l, r).0,
            big_elem: |l, r| ElementDiv::div_long(l, r).0,
            checked_elem: ElementDiv::div_long_checked,
            wrapping_elem: ElementDiv::div_long_wrapping,
        },
    );

    /*
    c.benchmark_group("BitSlice::div*")
        .bench_with_input(
            BenchmarkId::new("div_element_checked", "[1], [1]"),
            one,
            |b, one| {
                b.iter(|| ElementDiv::div_long_checked(black_box(&mut [1usize]), black_box(one)))
            })
        .bench_with_input(
            BenchmarkId::new("div_element_checked", "[usize::MAX], [usize::MAX]"),
            max,
            |b, max| {
                b.iter(|| ElementDiv::div_long_checked(black_box(&mut [usize::MAX]), black_box(max)))
            })
        .bench_with_input(
            BenchmarkId::new("div_element_wrapping", "[1], [1]"),
            one,
            |b, one| {
                b.iter(|| ElementDiv::div_long_wrapping(black_box(&mut [1usize]), black_box(one)))
            })
        .bench_with_input(
            BenchmarkId::new("div_element_wrapping", "[usize::MAX], [usize::MAX]"),
            max,
            |b, max| {
                b.iter(|| ElementDiv::div_long_wrapping(black_box(&mut [usize::MAX]), black_box(max)))
            });
     */
}

pub fn bench_shl(c: &mut Criterion) {
    let one = &[1usize];
    let max = &[usize::MAX];

    c.benchmark_group("BitSliceExt::shl*")
        .bench_with_input(
            BenchmarkId::new("BitwiseShl::shl", "[1], 1"),
            one,
            |b, one| b.iter(|| BitwiseShl::shl(black_box(one), black_box(1))),
        )
        .bench_with_input(
            BenchmarkId::new("BitwiseShl::shl", "[usize::MAX], 1"),
            max,
            |b, max| b.iter(|| BitwiseShl::shl(black_box(max), black_box(1))),
        )
        .bench_with_input(
            BenchmarkId::new("ElementShl::shl", "[1], 1"),
            one,
            |b, one| b.iter(|| ElementShl::shl(black_box(one), black_box(1))),
        )
        .bench_with_input(
            BenchmarkId::new("ElementShl::shl", "[usize::MAX], 1"),
            max,
            |b, max| b.iter(|| ElementShl::shl(black_box(max), black_box(1))),
        )
        .bench_function(BenchmarkId::new("ElementShl::shl_checked", "[1], 1"), |b| {
            b.iter(|| {
                let mut one = [1usize];
                ElementShl::shl_checked(black_box(&mut one), black_box(1));
            })
        })
        .bench_function(
            BenchmarkId::new("ElementShl::shl_checked", "[usize::MAX], 1"),
            |b| {
                b.iter(|| {
                    let mut max = [usize::MAX];
                    ElementShl::shl_checked(black_box(&mut max), black_box(1));
                })
            },
        )
        .bench_function(
            BenchmarkId::new("ElementShl::shl_wrapping", "[1], 1"),
            |b| {
                b.iter(|| {
                    let mut one = [1usize];
                    ElementShl::shl_wrapping(black_box(&mut one), black_box(1));
                })
            },
        )
        .bench_function(
            BenchmarkId::new("ElementShl::shl_wrapping", "[usize::MAX], 1"),
            |b| {
                b.iter(|| {
                    let mut max = [usize::MAX];
                    ElementShl::shl_wrapping(black_box(&mut max), black_box(1));
                })
            },
        );
}

criterion_group!(
    name = benches;
    config = make_criterion();
    targets = bench_shl, bench_add, bench_sub, bench_div, bench_cmp
);
criterion_main!(benches);
