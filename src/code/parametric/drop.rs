use crate::{Compilable, Instruction};

pub struct DropStack{}

impl DropStack {
  pub fn new() -> Box<dyn Instruction> {
    Box::new(Self{})
  }
}

impl Compilable for DropStack {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0x1a);
  }
}

impl Instruction for DropStack {}
