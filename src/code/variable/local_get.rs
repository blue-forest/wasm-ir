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

use std::sync::Arc;

use crate::Instruction;
use crate::code::Locals;
use crate::values::from_u32;
use super::Local;

#[derive(Debug)]
pub struct LocalGet {
  local: Arc<Local>,
}

impl LocalGet {
  pub fn create(local: Arc<Local>) -> Box<dyn Instruction> {
    Box::new(Self{ local })
  }
}

impl Instruction for LocalGet {
  fn compile<'a>(&self, buf: &mut Vec<u8>, locals: &Locals<'a>) {
    buf.push(0x20);
    buf.extend(&from_u32(locals.get_idx(&self.local)));
  }
}
