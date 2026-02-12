// Bivector contains dC2 components - one for each pair of axes in the dimension

// 1 -> 0
// 2 -> 1
// 3 -> 3
// 4 -> 6

use core::mem::ManuallyDrop;
use core::ops::Add;
use numeric_static_iter::{IntoStaticIter, StaticIter};
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

pub type BiVec3<T> = BiVector<T, 3>;

trait Dims {
    type const DIMS: usize;
}

impl<const N: usize> Dims for Helper<N> {
    type const DIMS: usize = const { choose_n(N, 2) };
}

struct Helper<const N: usize>;

extern crate std;

/// An N-dimensional bi-vector, or 2-vector. Read more [here].
///
/// [here]: https://en.wikipedia.org/wiki/Bivector
#[allow(dead_code)]
pub struct BiVector<T, const N: usize>([T; <Helper<N> as Dims>::DIMS]);

impl<T: RealSigned, const N: usize> BiVector<T, N> {
    pub fn from_components<const M: usize>(components: [T; M]) -> BiVector<T, N> {
        const { assert!(M == <Helper<N> as Dims>::DIMS) };
        let comps = ManuallyDrop::new(components);
        BiVector(unsafe { core::mem::transmute_copy::<[T; M], [T; _]>(&*comps) })
    }

    /// Compute the exterior (or wedge) product of two vectors, and return it (i.e. a 2-blade, or 2-vector)
    pub fn new(a: Vector<T, N>, b: Vector<T, N>) -> BiVector<T, N> {
        // For each possible square matrix of two input vectors, generate the determinant. This is
        // the component for that space.
        let mut out = core::array::from_fn(|_| T::zero());

        let mut idx = 0;
        for i in 0..N {
            for j in (i + 1)..N {
                let comp =
                    Matrix::new([[a[i].clone(), b[i].clone()], [a[j].clone(), b[j].clone()]])
                        .determinant();
                out[idx] = comp;
                idx += 1;
            }
        }
        BiVector(out)
    }

    /// Get the list of elements in this BiVector. This slice will always be `DIM choose 2` long.
    pub fn elements(&self) -> &[T] {
        &self.0
    }
}

impl<T: Add, const N: usize> Add for BiVector<T, N> {
    type Output = BiVector<T::Output, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let out = self.0
            .into_static_iter()
            .zip(rhs.0)
            .map(|(l, r)| l + r)
            .collect();
        BiVector(out)
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
        assert_eq!(choose_n(23, 17), 100_947);
        assert_eq!(choose_n(1_000_000_000, 2), 499_999_999_500_000_000);
    }

    #[test]
    fn test_new_2() {
        let bv = BiVector::<f64, 2>::new(
            Vector::from_xy(2., 0.),
            Vector::from_xy(0., 2.),
        );
        assert_eq!(bv.elements(), &[4.0]);

        let bv = BiVector::<f64, 2>::new(
            Vector::from_xy(0., 2.),
            Vector::from_xy(2., 0.),
        );
        assert_eq!(bv.elements(), &[-4.0]);

        let bv = BiVector::<f64, 2>::new(
            Vector::from_xy(2., 1.),
            Vector::from_xy(4., 3.),
        );
        assert_eq!(bv.elements(), &[2.0]);
    }

    #[test]
    fn test_new_3() {
        let bv = BiVector::<f64, 3>::new(
            Vector::from_xyz(1., 2., 3.),
            Vector::from_xyz(4., 5., 6.),
        );

        assert_eq!(bv.elements(), &[-3.0, -6.0, -3.0]);

        let bv = BiVector::<f64, 3>::new(
            Vector::from_xyz(3., 2., 1.),
            Vector::from_xyz(4., 5., 6.),
        );

        assert_eq!(bv.elements(), &[7.0, 14.0, 7.0]);
    }

    #[test]
    fn test_add() {
        let a = BiVector::<_, 3>::from_components([1.0, 2.0, 3.0]);
        let b = BiVector::from_components([1.1, 2.2, 3.3]);

        assert_eq!((a + b).elements(), &[2.1, 4.2, 6.3]);
    }
}
