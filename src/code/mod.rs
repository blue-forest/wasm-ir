// This code is free software distributed under GPLv3 by Blue Forest.

use crate::Compilable;
use crate::values::from_u32;

pub mod control;
pub mod memory;
pub mod numeric;
pub mod parametric;

pub struct Body {
  // TODO: locals
  instructions: Vec<Box<dyn Instruction>>
}

impl Body {
  pub fn new(instructions: Vec<Box<dyn Instruction>>) -> Self {
    Self{ instructions }
  }
}

impl Compilable for Body {
  fn compile(&self, buf: &mut Vec<u8>) {
    // TODO: no allocation ?
    let mut code = vec![0x00]; // locals
    for instruction in self.instructions.iter() {
      instruction.compile(&mut code);
    }
    buf.extend(&from_u32((code.len() + 1) as u32)); // +1 for the end
    buf.extend(code);
    buf.push(0x0b); // end
  }
}

pub trait Instruction: Compilable {}
