use super::Instruction;

pub trait RefInstruction: Instruction {}

mod func;
pub use func::RefFunc;

mod is_null;
pub use is_null::RefIsNull;

mod null;
pub use null::RefNull;

