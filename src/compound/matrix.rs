
pub type SquareMatrix<T, const N: usize> = Matrix<T, N, N>;

pub struct Matrix<T, const N: usize, const M: usize>([[T; N]; M]);
