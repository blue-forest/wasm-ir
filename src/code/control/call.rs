use crate::{Compilable, Instruction};
use crate::values::from_u32;

pub struct Call {
  function_idx: u32,
  parameters:   Vec<Box<dyn Instruction>>
}

impl Call {
  pub fn new(
    function_idx: u32,
    parameters:   Vec<Box<dyn Instruction>>
  ) -> Box<dyn Instruction> {
    Box::new(Self{ function_idx, parameters })
  }

  pub fn new_stacked(function_idx: u32) -> Box<dyn Instruction> {
    Box::new(Self{ function_idx, parameters: Vec::new() })
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

