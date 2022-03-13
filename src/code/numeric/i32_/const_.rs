use crate::{Compilable, Instruction};
use crate::values::from_u32;

pub struct I32Const(u32);

impl I32Const {
  pub fn new(value: u32) -> Box<dyn Instruction> {
    Box::new(Self(value))
  }
}

impl Compilable for I32Const {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0x41);
    buf.extend(from_u32(self.0));
  }
}

impl Instruction for I32Const {}