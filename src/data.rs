use crate::{Compilable, Instruction};
use crate::values::from_u32;

pub struct Data {
  byte: Vec<u8>,
  mode: DataMode,
}

impl Data {
  pub fn new(byte: Vec<u8>, mode: DataMode) -> Self {
    Self{ byte, mode }
  }
}

impl Compilable for Data {
  fn compile(&self, buf: &mut Vec<u8>) {
    self.mode.compile(buf);
    buf.extend(&from_u32(self.byte.len() as u32));
    buf.extend(&self.byte);
  }
}

pub enum DataMode {
  Passive,
  Active(Box<dyn Instruction>)
}

impl Compilable for DataMode {
  fn compile(&self, buf: &mut Vec<u8>) {
    match self {
      Self::Passive             => buf.push(0x01),
      Self::Active(instruction) => {
        buf.push(0x00);
        instruction.compile(buf);
        buf.push(0x0b); // end
      }
    }
  }
}
