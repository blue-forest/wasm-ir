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

use crate::Compilable;
use crate::values::from_u32;

#[derive(Debug)]
pub struct Function(u32);

impl Function {
  pub fn new(value: u32) -> Self {
    Self(value)
  }
}

impl Compilable for Function {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.extend(&from_u32(self.0));
  }
}

#[derive(Debug)]
pub struct StartFunction(Function);

impl StartFunction {
  pub fn new(value: u32) -> Self {
    Self(Function::new(value))
  }
}

impl Compilable for StartFunction {
  fn compile(&self, buf: &mut Vec<u8>) {
    let mut section = Vec::new();
    self.0.compile(&mut section);
    buf.push(0x08); // section id
    buf.extend(&from_u32(section.len() as u32));
    buf.extend(section);
  }
}
