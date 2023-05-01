
pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

pub trait RealConsts {
    fn pi() -> Self;
    fn e() -> Self;
}
