// This code is free software distributed under GPLv3 by Blue Forest.

use crate::Compilable;
use crate::values::from_u32;

pub mod control;
pub mod memory;
pub mod numeric;
pub mod parametric;
pub mod reference;
pub mod table;
pub mod variable;

pub struct Local {
  n:       u32,
  valtype: u8,
}

impl Local {
  pub fn new(n: u32, valtype: u8) -> Self {
    Self{ n, valtype }
  }
}

impl Compilable for Local {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.extend(&from_u32(self.n));
    buf.push(self.valtype);
  }
}

pub struct Body {
  locals:       Vec<Local>,
  instructions: Vec<Box<dyn Instruction>>,
}

impl Body {
  pub fn new(
    locals:       Vec<Local>,
    instructions: Vec<Box<dyn Instruction>>
  ) -> Self {
    Self{ locals, instructions }
  }
}

impl Compilable for Body {
  fn compile(&self, buf: &mut Vec<u8>) {
    // TODO: no allocation ?
    let mut code = Vec::new(); // locals
    code.extend(&from_u32(self.locals.len() as u32));
    for local in self.locals.iter() {
      local.compile(&mut code);
    }
    for instruction in self.instructions.iter() {
      instruction.compile(&mut code);
    }

    buf.extend(&from_u32((code.len() + 1) as u32)); // +1 for the end
    buf.extend(code);
    buf.push(0x0b); // end
  }
}

pub trait Instruction: Compilable {}
