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
pub struct LocalSet {
  local_idx: u32,
  value:     Option<Box<dyn Instruction>>,
}

impl LocalSet {
  pub fn with_stack(local_idx: u32) -> Box<dyn Instruction> {
    Box::new(Self{ local_idx, value: None })
  }

  pub fn with_operands(
    local_idx: u32,
    value: Box<dyn Instruction>
  ) -> Box<dyn Instruction> {
    Box::new(Self{ local_idx, value: Some(value) })
  }
}

impl Compilable for LocalSet {
  fn compile(&self, buf: &mut Vec<u8>) {
    if let Some(value) = &self.value {
      value.compile(buf);
    }
    buf.push(0x21);
    buf.extend(&from_u32(self.local_idx));
  }
}

impl Instruction for LocalSet {}
