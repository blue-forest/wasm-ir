use crate::{Compilable, Instruction};

use super::RefInstruction;

pub struct RefNull {
  type_: u8,
}

impl RefNull {
  pub fn create(type_: u8) -> Box<dyn RefInstruction> {
    Box::new(Self{ type_ })
  }
}

impl Compilable for RefNull {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0xd0);
    buf.push(self.type_);
  }
}

impl Instruction for RefNull {}

impl RefInstruction for RefNull {}

