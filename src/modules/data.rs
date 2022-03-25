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

use crate::{Compilable, Data};
use super::{Module, Section};

impl Module {
  pub fn add_data(&mut self, data: Data) {
    self.sec_data.push(data);
  }
}

#[derive(Debug)]
pub struct DataSection {
  data: Vec<Data>,
}

impl DataSection {
  pub fn new() -> Self {
    Self{ data: Vec::new() }
  }

  pub fn push(&mut self, data: Data) {
    self.data.push(data);
  }
}

impl Default for DataSection {
  fn default() -> Self { Self::new() }
}

impl Section for DataSection {
  fn section_id(&self) -> u8 { 0x0b }

  fn len(&self) -> u32 { self.data.len() as u32 }

  fn content(&self, _module: &Module) -> Vec<u8> {
    let mut result = Vec::new();
    for data in self.data.iter() {
      data.compile(&mut result);
    }
    result
  }
}
