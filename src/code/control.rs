// This code is free software distributed under GPLv3 by Blue Forest.

use crate::Compilable;
use crate::values::from_u32;

use super::Instruction;

pub struct Call {
  function_idx: u32,
  parameters:   Vec<Box<dyn Instruction>>
}

impl Call {
  pub fn new(
    function_idx: u32,
    parameters:   Vec<Box<dyn Instruction>>
  ) -> Box<Self> {
    Box::new(Self{ function_idx, parameters })
  }
}

impl Compilable for Call {
  fn compile(&self, buf: &mut Vec<u8>) {
    for parameter in self.parameters.iter() {
      parameter.compile(buf);
    }
    buf.push(0x10);
    buf.extend(&from_u32(self.function_idx));
  }
}

impl Instruction for Call {}
