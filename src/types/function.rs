use crate::Compilable;
use crate::values::from_u32;

pub struct FunctionType {
  parameters: Vec<u8>,
  result:     Vec<u8>,
}

impl FunctionType {
  pub fn new(parameters: Vec<u8>, result: Vec<u8>) -> Self {
    Self{ parameters, result }
  }
}

impl Compilable for FunctionType {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0x60);
    buf.extend(from_u32(self.parameters.len() as u32));
    buf.extend(&self.parameters);
    buf.extend(from_u32(self.result.len() as u32));
    buf.extend(&self.result);
  }
}

