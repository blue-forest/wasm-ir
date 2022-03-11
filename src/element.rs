use crate::{Compilable, Instruction};
use crate::code::reference::RefInstruction;
use crate::values::from_u32;

pub struct Element {
  type_:     u8,
  init_expr: Vec<Box<dyn RefInstruction>>,
  init_func: Vec<u32>,
  mode:      ElementMode,
}

impl Element {
  pub fn with_expr(
    type_: u8,
    init:  Vec<Box<dyn RefInstruction>>,
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
    /*
    let (expr_bit, init) = if self.init_expr.is_empty() {
      (0x00, self.init_expr.iter() as &dyn Iter)
    } else {
      (0x04, self.init_func.iter())
    };
    if self.type_ != 0 {
      todo!()
    }
    */
    match &self.mode {
      ElementMode::Passive => {
        let mut byte = 0x01;
        if !self.init_expr.is_empty() {
          byte += 0x04;
        }
        buf.push(byte);
        todo!()
      }
      ElementMode::Active{ table_idx, offset } => {
        let mut byte = 0x00;
        if !self.init_expr.is_empty() {
          byte += 0x04;
        }
        if *table_idx != 0 {
          buf.push(byte + 0x02);
          buf.extend(&from_u32(*table_idx));
        } else {
          buf.push(byte);
        }
        offset.compile(buf);
        buf.push(0x0b);
        if *table_idx != 0 {
          buf.push(self.type_);
        }
        if self.init_expr.is_empty() {
          buf.extend(&from_u32(self.init_func.len() as u32));
          for function_idx in self.init_func.iter() {
            buf.extend(&from_u32(*function_idx));
          }
        } else {
          buf.push(0x01);
          for instruction in self.init_expr.iter() {
            instruction.compile(buf);
          }
          buf.push(0x0b);
        }
      }
      ElementMode::Declarative => {
        let mut byte = 0x03;
        if !self.init_expr.is_empty() {
          byte += 0x04;
        }
        buf.push(byte);
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
