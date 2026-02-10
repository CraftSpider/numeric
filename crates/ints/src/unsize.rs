use core::cmp::Ordering;
use numeric_bits::algos::{CmpAlgo, Element};
use numeric_traits::class::{Integral, Numeric, Unsigned};
use numeric_traits::identity::{One, Zero};

pub struct Int([u8]);

impl PartialEq for Int {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Int {}

impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for Int {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.is_negative().cmp(&other.is_negative()) {
            Ordering::Equal => self.0.cmp(&other.0),
            ord => ord.reverse(),
        }
    }
}

impl Zero for &Int {
    fn zero() -> Self {
        <&Int>::from(&0i8)
    }

    fn is_zero(&self) -> bool {
        self.0.into_iter().all(|b| *b == 0)
    }
}

impl One for &Int {
    fn one() -> Self {
        <&Int>::from(&1i8)
    }

    fn is_one(&self) -> bool {
        self.0[0] == 1 && self.0[1..].iter().all(|&b| b == 0)
    }
}

pub struct UInt([u8]);

impl PartialEq for UInt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for UInt {}

impl PartialOrd for UInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for UInt {
    fn cmp(&self, other: &Self) -> Ordering {
        <Element as CmpAlgo>::cmp(&self.0, &other.0)
    }
}

impl Zero for &UInt {
    fn zero() -> Self {
        <&UInt>::from(&0u8)
    }

    fn is_zero(&self) -> bool {
        self.0.into_iter().all(|b| *b == 0)
    }
}

impl One for &UInt {
    fn one() -> Self {
        <&UInt>::from(&1u8)
    }

    fn is_one(&self) -> bool {
        self.0[0] == 1 && self.0[1..].iter().all(|&b| b == 0)
    }
}

impl Unsigned for UInt {}
