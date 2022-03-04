use crate::Compilable;

use super::Instruction;

pub struct DropStack{}

impl Compilable for DropStack {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0x1a);
  }
}

impl Instruction for DropStack {}
