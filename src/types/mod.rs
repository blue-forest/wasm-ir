use crate::values::from_u32;

pub trait Type {
  fn compile(&self) -> Vec<u8>;
}

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

impl Type for FunctionType {
  fn compile(&self) -> Vec<u8> {
    let mut result = vec![0x60];
    result.extend(from_u32(self.parameters.len() as u32));
    result.extend(&self.parameters);
    result.extend(from_u32(self.result.len() as u32));
    result.extend(&self.result);
    result
  }
}
