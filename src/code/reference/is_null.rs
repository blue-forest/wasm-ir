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

use crate::Instruction;
use crate::code::Locals;

#[derive(Debug)]
pub struct RefIsNull{}

impl RefIsNull {
  pub fn create() -> Box<dyn Instruction> {
    Box::new(Self{})
  }
}

impl Instruction for RefIsNull {
  fn compile<'a>(&self, buf: &mut Vec<u8>, _locals: &Locals<'a>) {
    buf.push(0xd1);
  }
}
