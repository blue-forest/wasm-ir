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
pub struct FunctionType {
  parameters: Vec<u8>,
  result:     Vec<u8>,
}

impl FunctionType {
  pub fn new(parameters: Vec<u8>, result: Vec<u8>) -> Self {
    Self{ parameters, result }
  }

  pub fn n_params(&self) -> u32 {
    self.parameters.len() as u32
  }
}

impl Compilable for FunctionType {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0x60);
    buf.extend(from_u32(self.parameters.len() as u32));
    buf.extend(&self.parameters);
    buf.extend(from_u32(self.result.len() as u32));
    buf.extend(&self.result);
  }
}

