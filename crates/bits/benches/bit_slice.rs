use core::cmp::Ordering;
use numeric_bits::algos::{BitwiseAdd, BitwiseDiv, BitwiseShl, BitwiseSub, ElementAdd, ElementCmp, ElementDiv, ElementShl, ElementSub};
use criterion::{criterion_main, criterion_group, Criterion, black_box, BenchmarkId};
use numeric_bench_util::make_criterion;

pub struct MathMeths {
    tr: &'static str,
    name: &'static str,
    big_bit: fn(&[usize], &[usize]) -> Vec<usize>,
    big_elem: fn(&[usize], &[usize]) -> Vec<usize>,
}

pub fn bench_common(c: &mut Criterion, meth: MathMeths) {
    let one = &[1usize];
    let max = &[usize::MAX];

    let big_bit_name = format!("BitwiseOps::{}", meth.name);
    let big_elem_name = format!("{}::{}", meth.tr, meth.name);

    c.benchmark_group(format!("BitSliceExt::{}*", meth.name))
        .bench_function(
            BenchmarkId::new(&big_bit_name, "[1], [1]"),
            |b| b.iter(|| (meth.big_bit)(black_box(one), black_box(one))),
        )
        .bench_function(
            BenchmarkId::new(&big_bit_name, "[1], [usize::MAX]"),
            |b| b.iter(|| (meth.big_bit)(black_box(one), black_box(max))),
        )
        .bench_function(
            BenchmarkId::new(&big_bit_name, "[usize::MAX], [1]"),
            |b| b.iter(|| (meth.big_bit)(black_box(max), black_box(one))),
        )
        .bench_function(
            BenchmarkId::new(&big_bit_name, "[usize::MAX], [usize::MAX]"),
            |b| b.iter(|| (meth.big_bit)(black_box(max), black_box(max))),
        )

        .bench_function(
            BenchmarkId::new(&big_elem_name, "[1], [1]"),
            |b| b.iter(|| (meth.big_elem)(black_box(one), black_box(one))),
        )
        .bench_function(
            BenchmarkId::new(&big_elem_name, "[1], [usize::MAX]"),
            |b| b.iter(|| (meth.big_elem)(black_box(one), black_box(max))),
        )
        .bench_function(
            BenchmarkId::new(&big_elem_name, "[usize::MAX], [1]"),
            |b| b.iter(|| (meth.big_elem)(black_box(max), black_box(one))),
        )
        .bench_function(
            BenchmarkId::new(&big_elem_name, "[usize::MAX], [usize::MAX]"),
            |b| b.iter(|| (meth.big_elem)(black_box(max), black_box(max))),
        );
}

pub fn bench_cmp(c: &mut Criterion) {
    let one = &[1usize];
    let two = &[2usize];

    let long = &[0usize, 0, 0, 0, 0, 0, 0, 1];
    let long2 = &[0usize, 0, 0, 0, 0, 0, 0, 2];


    c.benchmark_group("BitSliceExt::cmp*")
        .bench_function(
            BenchmarkId::new("ElementCmp::cmp", "[1], [1]"),
            |b| b.iter(|| assert_eq!(
                black_box(ElementCmp::cmp(black_box(one), black_box(one))),
                Ordering::Equal,
            ))
        )
        .bench_function(
            BenchmarkId::new("ElementCmp::cmp", "[1], [2]"),
            |b| b.iter(|| assert_eq!(
                black_box(ElementCmp::cmp(black_box(one), black_box(two))),
                Ordering::Less,
            ))
        )
        .bench_function(
            BenchmarkId::new("ElementCmp::cmp", "[2], [1]"),
            |b| b.iter(|| assert_eq!(
                black_box(ElementCmp::cmp(black_box(two), black_box(one))),
                Ordering::Greater,
            ))
        )
        .bench_function(
            BenchmarkId::new("ElementCmp::cmp", "[0; 7, 1], [0; 7, 1]"),
            |b| b.iter(|| assert_eq!(
                black_box(ElementCmp::cmp(black_box(long), black_box(long))),
                Ordering::Equal,
            ))
        )
        .bench_function(
            BenchmarkId::new("ElementCmp::cmp", "[0; 7, 1], [0; 7, 2]"),
            |b| b.iter(|| assert_eq!(
                black_box(ElementCmp::cmp(black_box(long), black_box(long2))),
                Ordering::Less,
            ))
        )
        .bench_function(
            BenchmarkId::new("ElementCmp::cmp", "[0; 7, 2], [0; 7, 1]"),
            |b| b.iter(|| assert_eq!(
                black_box(ElementCmp::cmp(black_box(long2), black_box(long))),
                Ordering::Greater,
            ))
        );
}

pub fn bench_add(c: &mut Criterion) {
    bench_common(c, MathMeths {
        tr: "ElementAdd",
        name: "add",
        big_bit: BitwiseAdd::add,
        big_elem: ElementAdd::add,
    });

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
    bench_common(c, MathMeths {
        tr: "ElementSub",
        name: "sub",
        big_bit: |l, r| BitwiseSub::sub(l, r).0,
        big_elem: |l, r| ElementSub::sub(l, r).0,
    });

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
    bench_common(c, MathMeths {
        tr: "ElementDiv",
        name: "div",
        big_bit: |l, r| BitwiseDiv::div_long(l, r).0,
        big_elem: |l, r| ElementDiv::div_long(l, r).0,
    });

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

    c.benchmark_group("BitSlice::shl*")
        .bench_with_input(
            BenchmarkId::new("shl_bitwise", "[1], 1"),
            one,
            |b, one| {
                b.iter(|| BitwiseShl::shl(black_box(one), black_box(1)))
            })
        .bench_with_input(
            BenchmarkId::new("shl_bitwise", "[usize::MAX], 1"),
            max,
            |b, max| {
                b.iter(|| BitwiseShl::shl(black_box(max), black_box(1)))
            })
        .bench_with_input(
            BenchmarkId::new("shl_wrap_and_mask", "[1], 1"),
            one,
            |b, one| {
                b.iter(|| ElementShl::shl(black_box(one), black_box(1)))
            })
        .bench_with_input(
            BenchmarkId::new("shl_wrap_and_mask", "[usize::MAX], 1"),
            max,
            |b, max| {
                b.iter(|| ElementShl::shl(black_box(max), black_box(1)))
            })
        .bench_function(
            BenchmarkId::new("shl_wrap_and_mask_checked", "[1], 1"),
            |b| {
                b.iter(|| {
                    let mut one = [1usize];
                    ElementShl::shl_wrap_and_mask_checked(black_box(&mut one), black_box(1));
                })
            })
        .bench_function(
            BenchmarkId::new("shl_wrap_and_mask_checked", "[usize::MAX], 1"),
            |b| {
                b.iter(|| {
                    let mut max = [usize::MAX];
                    ElementShl::shl_wrap_and_mask_checked(black_box(&mut max), black_box(1));
                })
            })
        .bench_function(
            BenchmarkId::new("shl_wrap_and_mask_wrapping", "[1], 1"),
            |b| {
                b.iter(|| {
                    let mut one = [1usize];
                    ElementShl::shl_wrap_and_mask_wrapping(black_box(&mut one), black_box(1));
                })
            })
        .bench_function(
            BenchmarkId::new("shl_wrap_and_mask_wrapping", "[usize::MAX], 1"),
            |b| {
                b.iter(|| {
                    let mut max = [usize::MAX];
                    ElementShl::shl_wrap_and_mask_wrapping(black_box(&mut max), black_box(1));
                })
            });
}

criterion_group!(
    name = benches;
    config = make_criterion();
    targets = bench_shl, bench_add, bench_sub, bench_div, bench_cmp
);
criterion_main!(benches);
