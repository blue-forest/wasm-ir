// This code is free software distributed under GPLv3 by Blue Forest.

use crate::Compilable;
use crate::values::from_u32;

pub struct Function(u32);

impl Function {
  pub fn new(value: u32) -> Self {
    Self(value)
  }
}

impl Compilable for Function {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.extend(&from_u32(self.0));
  }
}
