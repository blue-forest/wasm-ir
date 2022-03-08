use crate::Compilable;
use crate::types::Limit;

pub struct Table {
  ref_type: u8,
  limit:    Limit,
}

impl Table {
  pub fn new(ref_type: u8, limit: Limit) -> Self {
    Self{ ref_type, limit }
  }
}

impl Compilable for Table {
  fn compile(&self, buf: &mut Vec<u8>) { todo!() }
}
