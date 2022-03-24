use crate::{Compilable, FunctionType};
use super::{Module, Section};

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
