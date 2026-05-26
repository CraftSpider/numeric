//! Mathematical matrices of various types.
//!
//! Support is included for both fixed and dynamic sized matrices, as well as views into such.

#[cfg(feature = "alloc")]
mod dynamic;
mod slice;
mod r#static;
mod traits;

use core::ops::Bound;
#[cfg(feature = "alloc")]
pub use dynamic::DynMatrix;
pub use r#static::{Matrix, SquareMatrix};
pub use slice::{MatMut, MatSlice, *};
pub use traits::MatrixLike;

fn index_check(expected: (usize, usize), actual: (usize, usize)) {
    if actual.0 >= expected.0 || actual.1 >= expected.1 {
        panic!(
            "Index out of range for matrix of size {}x{}: ({}, {})",
            expected.0, expected.1, actual.0, actual.1
        )
    }
}

fn range_check(expected: (usize, usize), rows: (usize, usize), cols: (usize, usize)) {
    if rows.0 >= expected.0 {
        panic!(
            "Row start index out of range for matrix of size {}x{}: {}",
            expected.0, expected.1, rows.0
        )
    }
    if rows.1 >= expected.0 {
        panic!(
            "Row end index out of range for matrix of size {}x{}: {}",
            expected.0, expected.1, rows.1
        )
    }
    if cols.0 >= expected.1 {
        panic!(
            "Column start index out of range for matrix of size {}x{}: {}",
            expected.0, expected.1, cols.0
        )
    }
    if cols.1 >= expected.1 {
        panic!(
            "Column end index out of range for matrix of size {}x{}: {}",
            expected.0, expected.1, cols.1
        )
    }
}

fn bound_start(bound: Bound<&usize>) -> usize {
    match bound {
        Bound::Included(start) => *start,
        Bound::Excluded(start) => start + 1,
        Bound::Unbounded => 0,
    }
}

fn bound_end(bound: Bound<&usize>) -> Option<usize> {
    match bound {
        Bound::Included(end) => Some(end + 1),
        Bound::Excluded(end) => Some(*end),
        Bound::Unbounded => None,
    }
}

// TODO:
//   - Will get to have fun re-implementing some ops on each of these.
//   - Clean up implementation to reduce repetition.
