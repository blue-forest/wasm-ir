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

use crate::Compilable;
use crate::values::from_u32;

pub mod control;
pub mod memory;
pub mod numeric;
pub mod parametric;
pub mod reference;
pub mod table;
pub mod variable;

#[derive(Debug)]
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

#[derive(Debug)]
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

pub trait Instruction: Compilable + Debug {}
