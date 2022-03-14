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

use crate::{Compilable, Instruction};
use crate::values::from_u32;

pub struct CallIndirect {
  type_idx:     u32,
  table_idx:    u32,
  parameters:   Vec<Box<dyn Instruction>>,
  function_idx: Option<Box<dyn Instruction>>,
}

impl CallIndirect {
  pub fn with_operands(
    type_idx:     u32,
    table_idx:    u32,
    parameters:   Vec<Box<dyn Instruction>>,
    function_idx: Box<dyn Instruction>,
  ) -> Box<dyn Instruction> {
    Box::new(Self{
      type_idx,
      table_idx,
      parameters,
      function_idx: Some(function_idx),
    })
  }

  pub fn with_stack(
    type_idx:     u32,
    table_idx:    u32,
  ) -> Box<dyn Instruction> {
    Box::new(Self{
      type_idx,
      table_idx,
      parameters: Vec::new(),
      function_idx: None,
    })
  }
}

impl Compilable for CallIndirect {
  fn compile(&self, buf: &mut Vec<u8>) {
    if let Some(function_idx) = &self.function_idx {
      function_idx.compile(buf);
    }
    for parameter in self.parameters.iter() {
      parameter.compile(buf);
    }
    buf.push(0x11);
    buf.extend(&from_u32(self.type_idx));
    buf.extend(&from_u32(self.table_idx));
  }
}

impl Instruction for CallIndirect {}
