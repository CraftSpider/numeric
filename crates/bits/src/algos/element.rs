// Operation Types: carrying, checked, overflowing (wrapping + carrying), saturating, wrapping
// Shift Types: checked, overflowing (wrapping + checked), wrapping

pub struct Element;

mod add;
mod bitand;
mod bitor;
mod bitxor;
mod cmp;
mod div;
mod mul;
mod neg;
mod shl;
mod shr;
mod sub;

pub use bitand::ElementBitand;
pub use bitor::ElementBitor;
pub use bitxor::ElementBitxor;
pub use cmp::ElementCmp;
pub use div::ElementDiv;
pub use mul::ElementMul;
pub use neg::ElementNot;
pub use shl::ElementShl;
pub use shr::ElementShr;
