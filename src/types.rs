use crate::Compilable;
use crate::values::from_u32;

// val type
pub const I32        : u8 = 0x7f;
pub const I64        : u8 = 0x7e;
pub const F32        : u8 = 0x7d;
pub const F64        : u8 = 0x7c;
pub const V128       : u8 = 0x7b;
pub const FUNC_REF   : u8 = 0x70;
pub const EXTERN_REF : u8 = 0x6f;

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

pub struct Limit {
  min: u32,
  max: Option<u32>,
}

impl Limit {
  pub fn new(min: u32, max: Option<u32>) -> Self {
    Self{ min, max }
  }
}

impl Compilable for Limit {
  fn compile(&self, buf: &mut Vec<u8>) {
    if let Some(max) = self.max {
      buf.push(0x01);
      buf.extend(&from_u32(self.min));
      buf.extend(&from_u32(max));
    } else {
      buf.push(0x00);
      buf.extend(&from_u32(self.min));
    }
  }
}
