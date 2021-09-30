use std::ops::Add;
use num::Zero;

use dflow;

pub use dflow::CFG;

pub trait Offset
where
    Self: Clone + Eq + Add + Zero
{
}

pub trait StackOp {
    type Offset: Offset;
    fn pointer_change(&self) -> Self::Offset;
}

mod stackoff;
pub use stackoff::convert;
pub use stackoff::Error;