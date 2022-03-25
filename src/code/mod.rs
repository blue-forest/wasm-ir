/*
 * wasm-ir - Intermediate Representation of WebAssembly
 * Copyright © 2019-2022 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::fmt::Debug;

use crate::FunctionType;
use crate::values::from_u32;

pub mod control;
pub mod memory;
pub mod numeric;
pub mod parametric;
pub mod reference;
pub mod table;
pub mod variable;
pub use variable::{Local, LocalBuilder};
use variable::Locals;

#[derive(Debug)]
pub struct Body {
  locals:       LocalBuilder,
  instructions: Vec<Box<dyn Instruction>>,
}

impl Body {
  pub fn new(
    locals:       LocalBuilder,
    instructions: Vec<Box<dyn Instruction>>
  ) -> Self {
    Self{ locals, instructions }
  }

  pub fn compile(&self, buf: &mut Vec<u8>, type_: &FunctionType) {
    let mut code = Vec::new();
    let locals = self.locals.compile(&mut code, type_);
    for instruction in self.instructions.iter() {
      instruction.compile(&mut code, &locals);
    }
    buf.extend(&from_u32((code.len() + 1) as u32)); // +1 for the end
    buf.extend(code);
    buf.push(0x0b); // end
  }
}

pub trait Instruction: Debug {
  fn compile<'a>(&self, buf: &mut Vec<u8>, locals: &Locals<'a>);
}

pub trait ConstInstruction: Instruction {
  fn const_compile(&self, buf: &mut Vec<u8>);
}

impl<I: ConstInstruction> Instruction for I {
  fn compile<'a>(&self, buf: &mut Vec<u8>, _locals: &Locals<'a>) {
    self.const_compile(buf)
  }
}

