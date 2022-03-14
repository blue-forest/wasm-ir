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

pub struct TableInit {
  element_idx: u32,
  table_idx:   u32,
  destination: Option<Box<dyn Instruction>>,
  offset:      Option<Box<dyn Instruction>>,
  size:        Option<Box<dyn Instruction>>,
}

impl TableInit {
  pub fn with_operands(
    element_idx: u32,
    table_idx:   u32,
    destination: Box<dyn Instruction>,
    offset:      Box<dyn Instruction>,
    size:        Box<dyn Instruction>,
  ) -> Box<dyn Instruction> {
    Box::new(Self{
      element_idx,
      table_idx,
      destination: Some(destination),
      offset:      Some(offset),
      size:        Some(size),
    })
  }

  pub fn with_stack(element_idx: u32, table_idx: u32) -> Box<dyn Instruction> {
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
