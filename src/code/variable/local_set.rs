use crate::{Compilable, Instruction};
use crate::values::from_u32;

pub struct LocalSet {
  local_idx: u32,
}

impl LocalSet {
  pub fn create(local_idx: u32) -> Box<dyn Instruction> {
    Box::new(Self{ local_idx })
  }
}

impl Compilable for LocalSet {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0x21);
    buf.extend(&from_u32(self.local_idx));
  }
}

impl Instruction for LocalSet {}
