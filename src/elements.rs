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

  fn compile_init(&self, buf: &mut Vec<u8>) {
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

  fn leading_byte(&self, mode_byte: u8) -> u8 {
    if !self.init_expr.is_empty() {
      mode_byte + 0x04
    } else {
      mode_byte
    }
  }
}

impl Compilable for Element {
  fn compile(&self, buf: &mut Vec<u8>) {
    match &self.mode {
      ElementMode::Passive => {
        buf.push(self.leading_byte(0x01));
        buf.push(self.type_);
        self.compile_init(buf);
      }
      ElementMode::Active{ table_idx, offset } => {
        let byte = self.leading_byte(0x00);
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
        self.compile_init(buf);
      }
      ElementMode::Declarative => {
        buf.push(self.leading_byte(0x03));
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
