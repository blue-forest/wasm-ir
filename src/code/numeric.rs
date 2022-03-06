// This code is free software distributed under GPLv3 by Blue Forest.

use crate::Compilable;
use crate::values::from_u32;

use super::Instruction;

pub struct I32Const(u32);

impl I32Const {
  pub fn new(value: u32) -> Box<Self> {
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
