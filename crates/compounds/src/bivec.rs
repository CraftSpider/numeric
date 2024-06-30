use alloc::vec::Vec;

// Bivector contains dC2 components - one for each pair of axes in the dimension

// 1 -> 0
// 2 -> 1
// 3 -> 3
// 4 -> 6

use crate::matrix::Matrix;
use crate::vector::Vector;
use numeric_traits::class::RealSigned;

// TODO: Put this in some algorithms location
const fn choose_n(mut n: usize, mut r: usize) -> usize {
    if n < r {
        return 0;
    }
    let mut out = 1;
    let mut denom = 1;
    // sum(0..r, out *= n/r)
    while r > 0 {
        out *= n;
        denom *= r;
        // TODO: Way to keep the remainder or something, always grow by n/r?
        if out % denom == 0 {
            out /= denom;
            denom = 1;
        }
        n -= 1;
        r -= 1;
    }
    out /= denom;
    out
}

#[allow(dead_code)]
pub struct BiVector<T, const DIM: usize>(Vec<T>);

impl<T: RealSigned, const DIM: usize> BiVector<T, DIM> {
    const COMPS: usize = choose_n(DIM, 2);

    /// Compute the wedge product of two vectors, and return it (i.e. a 2-blade, or 2-vector)
    #[allow(clippy::missing_panics_doc)]
    pub fn new(a: Vector<T, DIM>, b: Vector<T, DIM>) -> BiVector<T, DIM> {
        // For each possible square matrix of two input vectors, generate the determinant. This is
        // the component for that space.
        let mut comps = Vec::with_capacity(choose_n(DIM, 2));
        for i in 0..DIM {
            for j in (i + 1)..DIM {
                let comp =
                    Matrix::new([[a[i].clone(), b[i].clone()], [a[j].clone(), b[j].clone()]])
                        .determinant();
                comps.push(comp);
            }
        }
        assert_eq!(
            comps.len(),
            Self::COMPS,
            "INVALID BIVECTOR STATE: THIS IS A BUG"
        );
        BiVector(comps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose() {
        assert_eq!(choose_n(1, 2), 0);
        assert_eq!(choose_n(2, 2), 1);
        assert_eq!(choose_n(3, 2), 3);
        assert_eq!(choose_n(4, 2), 6);
        assert_eq!(choose_n(25, 2), 300);
        assert_eq!(choose_n(23, 17), 100947);
        assert_eq!(choose_n(1000000000, 2), 499999999500000000);
    }

    #[test]
    fn test_bivec() {
        let bv = BiVector::<f64, 2>::new(Vector::default(), Vector::default());
        assert_eq!(bv.0, &[0.0]);
    }
}
