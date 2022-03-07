// This code is free software distributed under GPLv3 by Blue Forest.

use crate::Compilable;

use super::Instruction;

pub struct DropStack{}

impl DropStack {
  pub fn new() -> Box<Self> {
    Box::new(Self{})
  }
}

impl Compilable for DropStack {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0x1a);
  }
}

impl Instruction for DropStack {}
