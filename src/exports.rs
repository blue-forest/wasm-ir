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

pub struct Export {
  name: String,
}

impl Export {
  pub fn new(name: String) -> Self {
    Self{ name }
  }
}

impl Compilable for Export {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.extend(&from_u32(self.name.len() as u32));
    buf.extend(self.name.as_bytes());
  }
}

pub enum ExportDescription {
  Func(u32),
  Table(u32),
  Mem(u32),
  Global(u32),
}
