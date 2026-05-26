use crate::matrix::traits::Sealed;
use crate::matrix::{index_check, MatrixLike};
use crate::vector::{Vec2, Vector};
use core::array;
use core::ops::{Add, Index, IndexMut, Mul, Sub};
use numeric_static_iter::{zip_all, IntoStaticIter, StaticIter};
use numeric_traits::class::RealSigned;
use numeric_traits::identity::{One, Zero};

/// A matrix with both rows and columns of the same length.
pub type SquareMatrix<T, const N: usize> = Matrix<T, N, N>;

/// Arbitrary type matrix with a fixed number of rows and columns.
///
/// This type will generally be more optimized than a [`DynMatrix`], and is available even when
/// allocation isn't, but can take up quite a bit of stack space if `ROW` and `COL` are large.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<T, const ROW: usize, const COL: usize>([[T; COL]; ROW]);

impl<T, const ROW: usize, const COL: usize> Matrix<T, ROW, COL> {
    /// Create a new matrix from a 2D array of rows and columns.
    pub const fn new(rows: [[T; COL]; ROW]) -> Matrix<T, ROW, COL> {
        Matrix(rows)
    }

    /// Create a matrix from an array of vector rows
    pub fn from_columns(vecs: [Vector<T, ROW>; COL]) -> Matrix<T, ROW, COL> {
        Matrix(zip_all(vecs.map(<[T; ROW]>::from)).collect())
    }

    /// Create a matrix from an array of vector columns
    pub fn from_rows(vecs: [Vector<T, COL>; ROW]) -> Matrix<T, ROW, COL> {
        Matrix(vecs.into_static_iter().map(Vector::into).collect())
    }

    /// Convert this matrix into a 2D array of rows and columns.
    pub fn into_arrays(self) -> [[T; COL]; ROW] {
        self.0
    }

    /// Transpose the matrix. This can be interpreted as the following equivalent operations:
    /// - Reflecting the matrix over its main diagonal
    /// - Writing the rows of the matrix as the columns
    /// - Writing the columns of the matrix as the rows
    pub fn transpose(self) -> Matrix<T, COL, ROW> {
        Matrix::new(zip_all(self.0).collect())
    }
}

impl<T, const ROW: usize, const COL: usize> Sealed for Matrix<T, ROW, COL> {}

impl<T, const ROW: usize, const COL: usize> MatrixLike for Matrix<T, ROW, COL> {
    type Element = T;

    fn rows(&self) -> usize {
        ROW
    }

    fn cols(&self) -> usize {
        COL
    }

    fn as_ptr(&self) -> *const Self::Element {
        self.0.as_ptr().cast()
    }

    fn as_ptr_mut(&mut self) -> *mut Self::Element {
        self.0.as_mut_ptr().cast()
    }

    fn swap_rows(&mut self, a: usize, b: usize) {
        self.0.swap(a, b);
    }

    fn swap_columns(&mut self, a: usize, b: usize) {
        self.0.iter_mut().for_each(|row| row.swap(a, b))
    }
}

impl<T: Clone, const N: usize> SquareMatrix<T, N> {
    /// Get the diagonal of the matrix as a vector.
    pub fn diag(&self) -> Vector<T, N> {
        array::from_fn(|idx| self[(idx, idx)].clone()).into()
    }
}

impl<T: RealSigned, const N: usize> SquareMatrix<T, N> {
    /// Calculate the matrix determinant
    pub fn determinant(&self) -> T {
        // Optimize small matrices, which have short determinant formulas that should be faster than
        // doing a full row-reduction.
        match N {
            1 => self[(0, 0)].clone(),
            2 => {
                self[(0, 0)].clone() * self[(1, 1)].clone()
                    - self[(1, 0)].clone() * self[(0, 1)].clone()
            }
            _ => {
                let mut reduced = self.clone();
                let factor = reduced.gauss_elim();

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
        let map_rows =
            |l: [T; COL], r: [T; COL]| l.into_static_iter().zip(r).map(|(i, j)| i + j).collect();

        let rows = self
            .0
            .into_static_iter()
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
        let map_rows =
            |l: [T; COL], r: [T; COL]| l.into_static_iter().zip(r).map(|(i, j)| i - j).collect();

        let rows = self
            .0
            .into_static_iter()
            .zip(rhs.0)
            .map(|(i, j)| map_rows(i, j))
            .collect();

        Matrix::new(rows)
    }
}

impl<T, const ROW: usize, const COL: usize, const COL2: usize> Mul<Matrix<T, COL, COL2>>
    for Matrix<T, ROW, COL>
where
    T: Add<Output = T> + Mul<Output = T> + Clone,
{
    type Output = Matrix<T, ROW, COL2>;

    fn mul(self, rhs: Matrix<T, COL, COL2>) -> Self::Output {
        let rows = array::from_fn(|i| {
            array::from_fn(|j| {
                let mut out = self[(i, 0)].clone() * rhs[(0, j)].clone();
                for k in 1..COL {
                    out = out + self[(i, k)].clone() * rhs[(k, j)].clone();
                }
                out
            })
        });
        Matrix::new(rows)
    }
}

impl<T: Zero, const ROW: usize, const COL: usize> Zero for Matrix<T, ROW, COL> {
    fn zero() -> Self {
        Matrix(array::from_fn(|_| array::from_fn(|_| T::zero())))
    }

    fn is_zero(&self) -> bool {
        (&self.0)
            .into_static_iter()
            .all(|r| r.into_static_iter().all(T::is_zero))
    }
}

/// Note that this produces the multiplicative identity matrix, which is the matrix with one on
/// the diagonals, not a matrix filled with ones.
impl<T: Zero + One, const ROW: usize, const COL: usize> One for Matrix<T, ROW, COL> {
    fn one() -> Self {
        let rows =
            array::from_fn(|i| array::from_fn(|j| if i == j { T::one() } else { T::zero() }));
        Matrix::new(rows)
    }

    fn is_one(&self) -> bool {
        (&self.0).into_static_iter().enumerate().all(|(i, row)| {
            row.into_static_iter().enumerate().all(|(j, val)| {
                if i == j {
                    val.is_one()
                } else {
                    val.is_zero()
                }
            })
        })
    }
}

impl<T, const ROW: usize, const COL: usize> Index<(usize, usize)> for Matrix<T, ROW, COL> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        index_check((ROW, COL), index);
        &self.0[index.0][index.1]
    }
}

impl<T, const ROW: usize, const COL: usize> IndexMut<(usize, usize)> for Matrix<T, ROW, COL> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        index_check((ROW, COL), index);
        &mut self.0[index.0][index.1]
    }
}

impl<T, const ROW: usize, const COL: usize> Index<Vec2<usize>> for Matrix<T, ROW, COL> {
    type Output = T;

    fn index(&self, index: Vec2<usize>) -> &Self::Output {
        index_check((ROW, COL), (*index.x(), *index.y()));
        &self.0[*index.y()][*index.x()]
    }
}

impl<T, const ROW: usize, const COL: usize> IndexMut<Vec2<usize>> for Matrix<T, ROW, COL> {
    fn index_mut(&mut self, index: Vec2<usize>) -> &mut Self::Output {
        index_check((ROW, COL), (*index.x(), *index.y()));
        &mut self.0[*index.y()][*index.x()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let a = Matrix::new([[1, 2, 3], [4, 5, 6]]);

        let mut b = a.clone();
        b.swap_rows(0, 1);
        assert_eq!(b, Matrix::new([[4, 5, 6], [1, 2, 3]]));

        let mut b = a.clone();
        b.swap_columns(0, 2);
        assert_eq!(b, Matrix::new([[3, 2, 1], [6, 5, 4]]));
    }

    #[test]
    fn test_mul() {
        let a = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let b = Matrix::new([[7, 8], [9, 10], [11, 12]]);

        let expected = Matrix::new([[58, 64], [139, 154]]);
        assert_eq!(a * b, expected);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_row_reduce() {
        let mut a = Matrix::new([
            [1., 0., 4., 2.],
            [1., 2., 6., 2.],
            [2., 0., 8., 8.],
            [2., 1., 9., 4.],
        ]);

        let expected = Matrix::new([
            [1., 0., 4., 2.],
            [0., 1., 1., 0.],
            [0., 0., 0., 4.],
            [0., 0., 0., 0.],
        ]);
        a.gauss_elim();
        assert_eq!(a, expected);

        let mut b = Matrix::new([[2., -3., 1.], [2., 0., -1.], [1., 4., 5.]]);

        let expected = Matrix::new([
            [1., 4., 5.],
            [0., -11., -9.],
            [0., 0., -4.454_545_454_545_454],
        ]);
        b.gauss_elim();
        assert_eq!(b, expected);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_determinant() {
        let a = Matrix::new([[1., 2.], [3., 4.]]);

        assert_eq!(a.determinant(), -2.);

        let b = Matrix::<f64, 3, 3>::new([[2., -3., 1.], [2., 0., -1.], [1., 4., 5.]]);
        assert_eq!(b.determinant().round(), 49.);
    }

    #[test]
    fn test_slice() {
        let a = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        let tl = a.slice(0..2, 0..2);
        assert_eq!(tl.rows(), 2);
        assert_eq!(tl.cols(), 2);
        assert_eq!(tl, Matrix::new([[1, 2], [4, 5]]));

        let br = a.slice(1..3, 1..3);
        assert_eq!(br.rows(), 2);
        assert_eq!(br.cols(), 2);
        assert_eq!(br, Matrix::new([[5, 6], [8, 9]]));
    }
}
