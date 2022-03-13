use crate::Compilable;
use crate::values::from_u32;
use super::Instruction;

pub struct TableInit {
  element_idx: u32,
  table_idx:   u32,
  destination: Option<Box<dyn Instruction>>,
  offset:      Option<Box<dyn Instruction>>,
  size:        Option<Box<dyn Instruction>>,
}

impl TableInit {
  pub fn new(
    element_idx: u32,
    table_idx:   u32,
    destination: Box<dyn Instruction>,
    offset:      Box<dyn Instruction>,
    size:        Box<dyn Instruction>,
  ) -> Box<Self> {
    Box::new(Self{
      element_idx,
      table_idx,
      destination: Some(destination),
      offset:      Some(offset),
      size:        Some(size),
    })
  }

  pub fn new_stacked(element_idx: u32, table_idx: u32) -> Box<Self> {
    Box::new(Self{
      element_idx,
      table_idx,
      destination: None,
      offset:      None,
      size:        None,
    })
  }
}

impl Compilable for TableInit {
  fn compile(&self, buf: &mut Vec<u8>) {
    if let Some(destination) = &self.destination {
      destination.compile(buf);
    }
    if let Some(offset) = &self.offset {
      offset.compile(buf);
    }
    if let Some(size) = &self.size {
      size.compile(buf);
    }
    buf.push(0xfc);
    buf.extend(&from_u32(12));
    buf.extend(&from_u32(self.element_idx));
    buf.extend(&from_u32(self.table_idx));
  }
}

impl Instruction for TableInit {}
