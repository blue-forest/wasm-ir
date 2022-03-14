use crate::{Compilable, Instruction};

pub struct RefIsNull{}

impl RefIsNull {
  pub fn create() -> Box<dyn Instruction> {
    Box::new(Self{})
  }
}

impl Compilable for RefIsNull {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0xd1);
  }
}

impl Instruction for RefIsNull {}
