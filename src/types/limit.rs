use crate::Compilable;
use crate::values::from_u32;

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
