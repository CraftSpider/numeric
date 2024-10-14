//! Various scalar 'real' value type implementation

#![no_std]

pub mod f;
pub mod fixed;
pub mod p;
pub mod rat;

pub use f::F;
pub use fixed::Fixed;
pub use rat::Rat;
