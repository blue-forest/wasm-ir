use crate::{Compilable, FUNC_REF, Instruction};

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
    match &self.mode {
      ElementMode::Passive => {
        buf.push(expr_bit + 0x01);
        todo!()
      }
      ElementMode::Active{ table_idx, offset } => {
        todo!()
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
