use crate::{Body, Compilable};
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

  fn content(&self, _module: &Module) -> Vec<u8> {
    let mut result = Vec::new();
    for body in self.bodies.iter() {
      body.compile(&mut result);
    }
    result
  }
}
