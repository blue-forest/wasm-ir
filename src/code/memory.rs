// This code is free software distributed under GPLv3 by Blue Forest.

use crate::Compilable;
use crate::values::from_u32;

use super::Instruction;

pub struct I32Store {
  align:   u32,
  offset:  u32,
  address: Box<dyn Instruction>,
  value:   Box<dyn Instruction>,
}

impl I32Store {
  pub fn new(
    align:   u32,
    offset:  u32,
    address: Box<dyn Instruction>,
    value:   Box<dyn Instruction>,
  ) -> Box<Self> {
    Box::new(Self{ align, offset, address, value })
  }
}

impl Compilable for I32Store {
  fn compile(&self, buf: &mut Vec<u8>) {
    self.address.compile(buf);
    self.value.compile(buf);
    buf.push(0x36);
    buf.extend(from_u32(self.align));
    buf.extend(from_u32(self.offset));
  }
}

impl Instruction for I32Store {}

