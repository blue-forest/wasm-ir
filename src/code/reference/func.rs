use crate::{Compilable, Instruction};
use crate::values::from_u32;

use super::RefInstruction;

pub struct RefFunc {
  function_idx: u32,
}

impl RefFunc {
  pub fn create(function_idx: u32) -> Box<dyn RefInstruction> {
    Box::new(Self{ function_idx })
  }
}

impl Compilable for RefFunc {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0xd2);
    buf.extend(&from_u32(self.function_idx));
  }
}

impl Instruction for RefFunc {}

impl RefInstruction for RefFunc {}

