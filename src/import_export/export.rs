use crate::Compilable;
use crate::values::from_u32;

pub struct Export {
  name: String,
}

impl Export {
  pub fn new(name: String) -> Self {
    Self{ name }
  }
}

impl Compilable for Export {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.extend(&from_u32(self.name.len() as u32));
    buf.extend(self.name.as_bytes());
  }
}

