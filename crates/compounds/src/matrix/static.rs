use core::array;
use core::ops::{Add, Index, IndexMut, Mul, Sub};
use core::ptr::NonNull;
use numeric_traits::class::RealSigned;
use numeric_static_iter::{IntoStaticIter, StaticIter, zip_all};
use numeric_traits::identity::{One, Zero};
use crate::vector::Vector;

pub type SquareMatrix<T, const N: usize> = Matrix<T, N, N>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<T, const ROW: usize, const COL: usize>([[T; COL]; ROW]);

impl<T, const ROW: usize, const COL: usize> Matrix<T, ROW, COL> {
    pub const fn new(rows: [[T; COL]; ROW]) -> Matrix<T, ROW, COL> {
        Matrix(rows)
    }

    pub(crate) fn as_ptr(&self) -> NonNull<T> {
        NonNull::from(&self.0).cast()
    }

    pub(crate) fn as_mut_ptr(&mut self) -> NonNull<T> {
        NonNull::from(&mut self.0).cast()
    }

    pub fn from_columns(vecs: [Vector<T, ROW>; COL]) -> Matrix<T, ROW, COL> {
        Matrix(zip_all(vecs.map(<[T; ROW]>::from)).collect())
    }

    pub fn from_rows(vecs: [Vector<T, COL>; ROW]) -> Matrix<T, ROW, COL> {
        Matrix(vecs.into_static_iter().map(|v| v.into()).collect())
    }

    /// Transpose the matrix. This can be interpreted as the following equivalent operations:
    /// - Reflecting the matrix over its main diagonal
    /// - Writing the rows of the matrix as the columns
    /// - Writing the columns of the matrix as the rows
    pub fn transpose(self) -> Matrix<T, COL, ROW> {
        Matrix::new(zip_all(self.0).collect())
    }

    pub fn swap_rows(&mut self, a: usize, b: usize) {
        self.0.swap(a, b)
    }
}

impl<T: RealSigned, const ROW: usize, const COL: usize> Matrix<T, ROW, COL> {
    gauss_elim!(self, ROW, COL, self.0);
    row_reduce!();
}

impl<T: Clone, const N: usize> SquareMatrix<T, N> {
    pub fn diag(&self) -> Vector<T, N> {
        array::from_fn(|idx| self[(idx, idx)].clone()).into()
    }
}

impl<T: RealSigned, const N: usize> SquareMatrix<T, N> {
    pub fn determinant(&self) -> T {
        // Optimize small matrices, which have short determinant formulas that should be faster than
        // doing a full row-reduction.
        match N {
            1 => self[(0, 0)].clone(),
            2 => self[(0, 0)].clone() * self[(1, 1)].clone() - self[(1, 0)].clone() * self[(0, 1)].clone(),
            _ => {
                let (reduced, factor) = self.clone()
                    .gauss_elim();

                reduced.diag().product() / factor
            }
        }
    }
}

impl<T, const ROW: usize, const COL: usize> Default for Matrix<T, ROW, COL>
where
    T: Default,
{
    fn default() -> Self {
        Matrix(array::from_fn(|_| array::from_fn(|_| T::default())))
    }
}

impl<T, const ROW: usize, const COL: usize> Add for Matrix<T, ROW, COL>
where
    T: Add,
{
    type Output = Matrix<T::Output, ROW, COL>;

    fn add(self, rhs: Self) -> Self::Output {
        let map_rows = |l: [T; COL], r: [T; COL]| {
            l.into_static_iter()
                .zip(r)
                .map(|(i, j)| i + j)
                .collect()
        };

        let rows = self.0.into_static_iter()
            .zip(rhs.0)
            .map(|(i, j)| map_rows(i, j))
            .collect();
        Matrix::new(rows)
    }
}

impl<T, const ROW: usize, const COL: usize> Sub for Matrix<T, ROW, COL>
where
    T: Sub,
{
    type Output = Matrix<T::Output, ROW, COL>;

    fn sub(self, rhs: Self) -> Self::Output {
        let map_rows = |l: [T; COL], r: [T; COL]| {
            l.into_static_iter()
                .zip(r)
                .map(|(i, j)| i - j)
                .collect()
        };

        let rows = self.0.into_static_iter()
            .zip(rhs.0)
            .map(|(i, j)| map_rows(i, j))
            .collect();

        Matrix::new(rows)
    }
}

impl<T, const ROW: usize, const COL: usize, const COL2: usize> Mul<Matrix<T, COL, COL2>> for Matrix<T, ROW, COL>
where
    T: Add<Output = T> + Mul<Output = T> + Clone,
{
    type Output = Matrix<T, ROW, COL2>;

    fn mul(self, rhs: Matrix<T, COL, COL2>) -> Self::Output {
        let rows = array::from_fn(|i| array::from_fn(|j| {
            let mut out = self[(i, 0)].clone() * rhs[(0, j)].clone();
            for k in 1..COL {
                out = out + self[(i, k)].clone() * rhs[(k, j)].clone();
            }
            out
        }));
        Matrix::new(rows)
    }
}

impl<T: Zero, const ROW: usize, const COL: usize> Zero for Matrix<T, ROW, COL> {
    fn zero() -> Self {
        Matrix(array::from_fn(|_| array::from_fn(|_| T::zero())))
    }

    fn is_zero(&self) -> bool {
        (&self.0).into_static_iter()
            .all(|r| r.into_static_iter().all(|v| v.is_zero()))
    }
}

/// Note that this produces the multiplicative identity matrix, which is the matrix with one on
/// the diagonals, not a matrix filled with ones.
impl<T: Zero + One, const ROW: usize, const COL: usize> One for Matrix<T, ROW, COL> {
    fn one() -> Self {
        let rows = array::from_fn(|i| array::from_fn(|j| {
            if i == j {
                T::one()
            } else {
                T::zero()
            }
        }));
        Matrix::new(rows)
    }

    fn is_one(&self) -> bool {
        (&self.0).into_static_iter()
            .enumerate()
            .all(|(i, row)| {
                row.into_static_iter()
                    .enumerate()
                    .all(|(j, val)| if i == j {
                        val.is_one()
                    } else {
                        val.is_zero()
                    })
            })
    }
}

impl<T, const ROW: usize, const COL: usize> Index<(usize, usize)> for Matrix<T, ROW, COL> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<T, const ROW: usize, const COL: usize> IndexMut<(usize, usize)> for Matrix<T, ROW, COL> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

#[cfg(test)]
mod tests  {
    use super::*;

    #[test]
    fn test_mul() {
        let a = Matrix::new([
            [1, 2, 3],
            [4, 5, 6],
        ]);
        let b = Matrix::new([
            [7, 8],
            [9, 10],
            [11, 12],
        ]);

        let expected = Matrix::new([
            [58, 64],
            [139, 154],
        ]);
        assert_eq!(a * b, expected);
    }

    #[test]
    fn test_row_reduce() {
        let a = Matrix::new([
            [1., 0., 4., 2.],
            [1., 2., 6., 2.],
            [2., 0., 8., 8.],
            [2., 1., 9., 4.],
        ]);

        let expected = Matrix::new([
            [1., 0., 4., 2.],
            [0., 1., 1., 0.],
            [0., 0., 0., 4.],
            [0., 0., 0., 0.]
        ]);
        assert_eq!(a.row_reduce(), expected);

        let b = Matrix::new([
            [2., -3., 1.],
            [2., 0., -1.],
            [1., 4., 5.],
        ]);

        let expected = Matrix::new([
            [1., 4., 5.],
            [0., -11., -9.],
            [0., 0., -4.454545454545454],
        ]);

        assert_eq!(b.row_reduce(), expected);
    }

    #[test]
    fn test_determinant() {
        let a = Matrix::new([
            [1., 2.],
            [3., 4.],
        ]);

        assert_eq!(a.determinant(), -2.);

        let b = Matrix::<f64, 3, 3>::new([
            [2., -3., 1.],
            [2., 0., -1.],
            [1., 4., 5.],
        ]);
        assert_eq!(b.determinant().round(), 49.);
    }
}
