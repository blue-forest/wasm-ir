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

#[derive(Debug)]
pub struct Data {
  byte: Vec<u8>,
  mode: DataMode,
}

impl Data {
  pub fn new(byte: Vec<u8>, mode: DataMode) -> Self {
    Self{ byte, mode }
  }
}

impl Compilable for Data {
  fn compile(&self, buf: &mut Vec<u8>) {
    self.mode.compile(buf);
    buf.extend(&from_u32(self.byte.len() as u32));
    buf.extend(&self.byte);
  }
}

#[derive(Debug)]
pub enum DataMode {
  Passive,
  Active(Box<dyn Instruction>)
}

impl Compilable for DataMode {
  fn compile(&self, buf: &mut Vec<u8>) {
    match self {
      Self::Passive             => buf.push(0x01),
      Self::Active(instruction) => {
        buf.push(0x00);
        instruction.compile(buf);
        buf.push(0x0b); // end
      }
    }
  }
}
