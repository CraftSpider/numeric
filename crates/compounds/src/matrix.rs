//! Mathematical matrices of various types.
//!
//! Support is included for both fixed and dynamic sized matrices, as well as views into such.

#[macro_use]
mod macros;
#[cfg(feature = "alloc")]
mod dynamic;
mod refs;
mod r#static;

#[cfg(feature = "alloc")]
pub use dynamic::DynMatrix;
pub use r#static::{Matrix, SquareMatrix};
pub use refs::{MatrixMut, MatrixRef, MatrixSlice};

// TODO:
//   - Matrix refs? MatRef/MatMut would be just (*T, usize, usize), allow passing any size without
//     needing size-changing ops.
//   - Matrix slices? MatSlice/MatSliceMut would be (*T, usize, usize, Range<usize>) - we can slice
//     out unneeded rows, then it's row length, column length, and accessible columns.
//   - Will get to have fun re-implementing some ops on each of these.
