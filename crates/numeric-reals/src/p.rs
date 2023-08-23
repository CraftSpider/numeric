//! N-byte bounded posit (projective reals) value

use std::cmp::Ordering;
use std::ops::{Add, Neg, Sub};
use numeric_bits::bit_slice::BitSliceExt;

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
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct P<const N: usize>([u8; N]);

impl<const N: usize> P<N> {
    const FRAC_LEN: usize = N*8 - 5;
    
    const NAR: Self = {
        let mut out = P([0; N]);
        out.0[0] | 0x1;
        out
    };
    
    pub const fn new() -> P<N> {
        P([0; N])
    }
    
    pub const fn is_negative(self) -> bool {
        self.0[0] & 0x1 != 0
    }
    
    pub fn is_nar(self) -> bool {
        self.0[0] == 1 && self.0[1..].iter().all(|&b| b == 0)
    }
    
    pub fn next_up(self) -> P<N> {
        // Implement as two's complement +1
        todo!()
    }
    
    pub fn next_down(self) -> P<N> {
        // Implement as two's complement -1
        todo!()
    }
    
    fn regime(self) -> (bool, usize) {
        let mut iter = self.0.iter_bits().skip(1);
        let first = iter.next().unwrap();
        let count = iter.take_while(|&v| v == first)
            .count();
        (first, count)
    }
}

impl<const N: usize> From<i32> for P<N> {
    fn from(value: i32) -> Self {
        todo!()
    }
}

impl<const N: usize> PartialOrd for P<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl<const N: usize> Ord for P<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Implement as two's complement <=>
        todo!()
    }
}

impl<const N: usize> Neg for P<N> {
    type Output = P<N>;

    fn neg(mut self) -> Self::Output {
        // TODO: This may be wrong, 2's complement
        self.0[0] ^= 1;
        self
    }
}

impl<const N: usize> Add for P<N> {
    type Output = P<N>;
    
    fn add(self, rhs: Self) -> Self::Output {
        match (self.is_zero(), rhs.is_zero()) {
            (true, _) => return rhs,
            (_, true) => return self,
            _ => (),
        };
        
        if self.is_nar() || rhs.is_nar() {
            return Self::NAR
        }
        
        // S = s
        // r = if R0 == 0 { -k } else { k-1 }
        // e = E
        // f = F / 2^m
        // Value: P = ((1-3s)+f) * 2^((1-2s)*(4r+e+s))
        
        // 1-3s == 1 or -2
        // 1-2s == 1 or -1
        
        // When positive, we count fraction up from 1 to 2
        // When R0 is 1, it and exponent form power steps by 2 up from 1 (1, 2, 4)
        // When R0 is 0, it and exponent form power steps by 2 down from 1 (1, 1/2, 1/4)
        //    with additional note that exponent is always added - so on this side, we start at regime -1,
        //    exponent 3, then move exponent down till we tick over regime
        
        // When negative, we count fraction down from -2 to -1 (two's complement)
        // When R0 is 0, it and exponent form power steps by 2 up from 1 (1, 2, 4)
        //    with additional note that exponent is always added - so on this side, we start at regime -1,
        //    exponent 3, then move exponent down till we tick over regime
        // When R0 is 1, it and exponent form power steps by 2 down from 1 (1, 1/2, 1/4)
        
        // Moving up from 1:
        // When fraction rolls over, increment exponent and reset fraction
        // When exponent rolls over, increment regime and reset exponent
        
        // Moving down from 1:
        // Inverted regime
        // When fraction rolls over, decrement exponent and reset fraction
        
        // next_up is just two's complement add 1
        // next_down is just two's complement sub 1
        
        // Addition is more complex - start with just all positives:
        //   Sum will be no more precise than the larger exponent
        //   Take 'theoretical' values, sum them, round
    }
}

impl<const N: usize> Sub for P<N> {
    type Output = P<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

#[allow(non_camel_case_types)]
pub type p8 = P<1>;
#[allow(non_camel_case_types)]
pub type p16 = P<2>;
#[allow(non_camel_case_types)]
pub type p32 = P<4>;
#[allow(non_camel_case_types)]
pub type p64 = P<8>;

/// N-byte posit quire value.
/// 
/// A quire is an intermediate data structure used for performing multiple mathematical operations
/// on posits in succession without incurring precision drift. In other words, a quire allows
/// 'batching' operations before pulling out a result with only one rounding applied, to the final
/// result.
/// 
/// ## Format
///
/// This implementation is conformant with the 2022 Posit Standard, where `Quire<N>` is the quire
/// for [`P<N>`]. As such, `Quire<1>` is 128 bits, `Quire<4>` is 512 bits, etc.
// [S|C..|I..|F..]
pub struct Quire<const N: usize>([u128; N]);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        let p1 = p32::from(1);
        let p2 = p32::from(-1);
        
        assert_eq!(p1 + p1, p32::from(2));
        assert_eq!(p1 + p2, p32::from(0));
        assert_eq!(p2 + p1, p32::from(0));
        assert_eq!(p2 + p2, p32::from(-2));
    }
}
