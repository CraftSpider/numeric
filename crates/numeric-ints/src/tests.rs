use std::fmt;
use numeric_traits::class::{Integral, Signed};
use numeric_traits::cast::FromPrim;
use crate::{BigInt, U, I};

trait TestInt: fmt::Debug + Clone + Integral + FromPrim {}
impl<T: fmt::Debug + Clone + Integral + FromPrim> TestInt for T {}

#[test]
fn test_big_int() {
    test_int_signed::<BigInt>();
}

#[test]
fn test_u() {
    test_int::<U<1>>();
    test_int::<U<2>>();
    test_int::<U<3>>();
    test_int::<U<4>>();
    test_int::<U<5>>();
    test_int::<U<6>>();
    test_int::<U<7>>();
    test_int::<U<8>>();
}

#[test]
fn test_i() {
    test_int_signed::<I<1>>();
    test_int_signed::<I<2>>();
    test_int_signed::<I<3>>();
    test_int_signed::<I<4>>();
    test_int_signed::<I<5>>();
    test_int_signed::<I<6>>();
    test_int_signed::<I<7>>();
    test_int_signed::<I<8>>();
}

fn test_int<I: TestInt>() {
    test_add::<I>();
    // test_sub::<I>();
    // test_mul::<I>();
    // test_div::<I>();
}

fn test_int_signed<I: TestInt + Signed>() {
    test_int::<I>();
    test_add_signed::<I>();
    // test_sub_signed::<I>();
    // test_mul_signed::<I>();
    // test_div_signed::<I>();
}

fn test_add<I: TestInt>() {
    let zero = I::zero();
    let one = I::one();
    let two = I::truncate(2);

    assert_eq!(zero.clone() + zero.clone(), zero);
    assert_eq!(zero.clone() + one.clone(), one);
    assert_eq!(one.clone() + zero.clone(), one);
    assert_eq!(one.clone() + one.clone(), two);
}

fn test_add_signed<I: TestInt + Signed>() {
    let zero = I::zero();
    let one = I::one();
    let neg_one = I::truncate(-1);
    let neg_two = I::truncate(-2);

    assert_eq!(zero.clone() + neg_one.clone(), neg_one);
    assert_eq!(neg_one.clone() + zero.clone(), neg_one);
    assert_eq!(one.clone() + neg_one.clone(), zero);
    assert_eq!(neg_one.clone() + one.clone(), zero);
    assert_eq!(neg_one.clone() + neg_one.clone(), neg_two);
}
