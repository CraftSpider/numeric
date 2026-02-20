//! N-byte bounded posit (projective reals) value

use core::cmp::Ordering;
use core::ops::{Add, Neg, Sub};

/// N-byte posit value.
///
/// A posit is similar to a float, in that it forms a computer representation of real numbers,
/// but with several distinctive features:
///
/// - They have no redundant representations
/// - They have only one non-real value: `NaR`, sometimes also called `inf`
///
/// ## Format
///
/// This implementation is conformant with the 2022 Posit Standard, where the `n` of a posit
/// is `N * 8` of this type. As such, `P<1> == posit8`, `P<4> == posit32`, etc.
// [S|R..|R0|EE|F..]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct P<const N: usize>([u8; N]);

impl<const N: usize> P<N> {
    #[allow(unused)] // TODO: Use this
    const FRAC_LEN: usize = N * 8 - 5;

    /// Create a new posit with value `0`.
    pub const fn new() -> P<N> {
        P([0; N])
    }

    /// Whether this value is negative (`< 0`)
    pub const fn is_negative(self) -> bool {
        self.0[0] & 0x1 != 0
    }

    /// Whether this value is `NaR` (not a real). For those familiar with floats, all form of both
    /// `NaN` and `inf` values convert to `NaR` in the posit scheme.
    pub fn is_nar(self) -> bool {
        self.0[0] == 0x1 && self.0[1..].iter().all(|&b| b == 0)
    }
}

impl<const N: usize> Default for P<N> {
    fn default() -> Self {
        P::new()
    }
}

impl<const N: usize> PartialOrd for P<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl<const N: usize> Ord for P<N> {
    fn cmp(&self, _: &Self) -> Ordering {
        todo!()
    }
}

impl<const N: usize> Neg for P<N> {
    type Output = P<N>;

    fn neg(mut self) -> Self::Output {
        self.0[0] ^= 1;
        self
    }
}

impl<const N: usize> Add for P<N> {
    type Output = P<N>;

    fn add(self, _: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Sub for P<N> {
    type Output = P<N>;

    fn sub(self, _: Self) -> Self::Output {
        todo!()
    }
}

/// 8-bit posit. Range of `[2^-24, 2^24]`, with max integer value of `2^4` (16)
#[allow(non_camel_case_types)]
pub type p8 = P<1>;

/// 16-bit posit. Range of `[2^-56, 2^56]`, with max integer value of `2^10` (1024)
#[allow(non_camel_case_types)]
pub type p16 = P<2>;

/// 32-bit posit. Range of `[2^-120, 2^120]`, with max integer value of `2^23` (8,388,608)
#[allow(non_camel_case_types)]
pub type p32 = P<4>;

/// 64-bit posit. Range of `[2^-248, 2^248]`, with max integer value of `2^49` (562,949,953,421,312)
#[allow(non_camel_case_types)]
pub type p64 = P<8>;
