use crate::{Compilable, FunctionType};
use super::{Module, Section};

impl Module {
  #[inline(always)]
  pub fn add_type(&mut self, profile: FunctionType) -> u32 {
    self.sec_type.push(profile)
  }

  #[inline(always)]
  pub fn get_type(&self, idx: u32) -> Option<&FunctionType> {
    self.sec_type.get(idx)
  }

  #[inline(always)]
  pub fn get_function_type(&self, idx: u32) -> Option<&FunctionType> {
    if let Some(type_idx) = self.sec_func.get(idx) {
      self.get_type(*type_idx)
    } else {
      None
    }
  }
}

#[derive(Debug)]
pub struct TypeSection {
  types: Vec<FunctionType>,
}

impl TypeSection {
  pub fn new() -> Self {
    Self{
      types: Vec::new(),
    }
  }

  pub fn push(&mut self, type_: FunctionType) -> u32 {
    let result = self.len();
    self.types.push(type_);
    result
  }

  pub fn get(&self, idx: u32) -> Option<&FunctionType> {
    self.types.get(idx as usize)
  }
}

impl Default for TypeSection {
  fn default() -> Self {
    Self::new()
  }
}

impl Section for TypeSection {
  fn section_id(&self) -> u8 { 0x01 }

  fn len(&self) -> u32 { self.types.len() as u32 }

  fn content(&self, _module: &Module) -> Vec<u8> {
    let mut result = Vec::new();
    for type_ in self.types.iter() {
      type_.compile(&mut result);
    }
    result
  }
}
