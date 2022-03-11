use crate::{Compilable, Instruction};
use crate::values::from_u32;

pub struct Element {
  type_:     u8,
  init_expr: Vec<Box<dyn Instruction>>,
  init_func: Vec<u32>,
  mode:      ElementMode,
}

impl Element {
  pub fn with_expr(
    type_: u8,
    init:  Vec<Box<dyn Instruction>>,
    mode:  ElementMode,
  ) -> Self {
    Self{
      type_, init_expr: init, init_func: Vec::new(), mode,
    }
  }

  pub fn with_func(
    init:  Vec<u32>,
    mode:  ElementMode,
  ) -> Self {
    Self{
      type_: 0x00, init_expr: Vec::new(), init_func: init, mode,
    }
  }
}

impl Compilable for Element {
  fn compile(&self, buf: &mut Vec<u8>) {
    let expr_bit = if self.init_expr.is_empty() { 0x00 } else { 0x04 };
    if self.type_ != 0 {
      todo!()
    }
    match &self.mode {
      ElementMode::Passive => {
        buf.push(expr_bit + 0x01);
        todo!()
      }
      ElementMode::Active{ table_idx, offset } => {
        if *table_idx != 0 {
          todo!()
        }
        buf.push(expr_bit + 0x00);
        offset.compile(buf);
        buf.push(0x0b);
        buf.extend(&from_u32(self.init_func.len() as u32));
        for function_idx in self.init_func.iter() {
          buf.extend(&from_u32(*function_idx));
        }
        // buf.push(0x0b);
      }
      ElementMode::Declarative => {
        buf.push(expr_bit + 0x03);
        todo!()
      }
    };
  }
}

pub enum ElementMode {
  Passive,
  Active{
    table_idx: u32,
    offset:    Box<dyn Instruction>,
  },
  Declarative,
}
