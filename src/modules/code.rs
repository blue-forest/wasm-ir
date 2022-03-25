use crate::Body;
use super::{Module, Section};

#[derive(Debug)]
pub struct CodeSection {
  bodies: Vec<Body>,
}

impl CodeSection {
  pub fn new() -> Self {
    Self{ bodies: Vec::new() }
  }

  pub fn push(&mut self, body: Body) {
    self.bodies.push(body);
  }
}

impl Default for CodeSection {
  fn default() -> Self { Self::new() }
}

impl Section for CodeSection {
  fn section_id(&self) -> u8 { 0x0a }

  fn len(&self) -> u32 { self.bodies.len() as u32 }

  fn content(&self, module: &Module) -> Vec<u8> {
    let mut result = Vec::new();
    for (i, body) in self.bodies.iter().enumerate() {
      body.compile(&mut result, module.get_function_type(i as u32).unwrap());
    }
    result
  }
}
