use crate::Compilable;
use crate::values::from_u32;

pub struct Import {
  module: String,
  name:   String,
}

impl Import {
  pub fn new(
    module: String,
    name:   String,
  ) -> Self {
    Self{ module, name }
  }
}

impl Compilable for Import {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.extend(&from_u32(self.module.len() as u32));
    buf.extend(self.module.as_bytes());
    buf.extend(&from_u32(self.name.len() as u32));
    buf.extend(self.name.as_bytes());
  }
}

pub enum ImportDescription {
  Func(u32),
  // Table(),
  // Mem(),
  // Global(),
}
