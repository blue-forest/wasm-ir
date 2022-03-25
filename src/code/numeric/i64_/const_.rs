use crate::{ConstInstruction, Instruction};
use crate::values::from_u64;

#[derive(Debug)]
pub struct I64Const(u64);

impl I64Const {
  pub fn create(value: u64) -> Box<dyn Instruction> {
    Box::new(Self(value))
  }

  pub fn with_const(value: u64) -> Box<dyn ConstInstruction> {
    Box::new(Self(value))
  }
}

impl ConstInstruction for I64Const {
  fn const_compile(&self, buf: &mut Vec<u8>) {
    buf.push(0x42);
    buf.extend(from_u64(self.0));
  }
}

