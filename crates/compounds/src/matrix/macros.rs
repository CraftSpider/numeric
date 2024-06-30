macro_rules! gauss_elim {
    ($self:ident, $cols:expr, $rows:expr, $data:expr) => {
        /// Uses Gaussian Elimination to put the matrix in triangular row-echelon form,
        /// and returns the new matrix and the product of the scalars by which the determinant was
        /// multiplied to produce the result.
        // TODO: This should be a signed integer op too, probably?
        fn gauss_elim(mut $self) -> (Self, T) {
            let mut factor = T::one();
            let mut row = 0;
            for col in 0..$cols {
                // We always act on submatrix [row..][col..]

                // If all columns are 0, skip
                let all_zero = (row..$rows).all(|r| $self[(r, col)].is_zero());
                if all_zero {
                    continue;
                }

                // Otherwise, find smallest non-zero number, move to top
                let (small_idx, _) = (row..$rows).map(|r| &$self[(r, col)])
                    .enumerate()
                    .filter(|(_, val)| !val.is_zero())
                    .min_by(|(_, val1), (_, val2)| val1.partial_cmp(val2).unwrap())
                    .unwrap();
                let small_idx = small_idx + row;

                if small_idx != row {
                    factor = factor.neg();
                    $self.swap_rows(small_idx, row);
                }

                // Remove all other values in this column by subtracting top row
                //   if it doesn't go evenly into some values, multiply by factor to make it so
                let top = $self[(row, col)].clone();
                for r in row+1..$rows {
                    let factor = $self[(r, col)].clone() / top.clone();
                    for c in col..$cols {
                        $self[(r, c)] = $self[(r, c)].clone() - $self[(row, c)].clone() * factor.clone();
                    }
                }

                // Increment row we're operating on
                row += 1;
            }

            ($self, factor)
        }
    }
}

macro_rules! row_reduce {
    () => {
        pub fn row_reduce(self) -> Self {
            self.gauss_elim().0
        }
    };
}
