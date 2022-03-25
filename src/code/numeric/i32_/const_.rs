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

use crate::{ConstInstruction, Instruction};
use crate::values::from_u32;

#[derive(Debug)]
pub struct I32Const(u32);

impl I32Const {
  pub fn create(value: u32) -> Box<dyn Instruction> {
    Box::new(Self(value))
  }

  pub fn with_const(value: u32) -> Box<dyn ConstInstruction> {
    Box::new(Self(value))
  }
}

impl ConstInstruction for I32Const {
  fn const_compile(&self, buf: &mut Vec<u8>) {
    buf.push(0x41);
    buf.extend(from_u32(self.0));
  }
}

