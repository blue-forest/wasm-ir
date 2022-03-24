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
use crate::types::Limit;

#[derive(Debug)]
pub struct Table {
  ref_type: u8,
  limit:    Limit,
}

impl Table {
  pub fn new(ref_type: u8, limit: Limit) -> Self {
    Self{ ref_type, limit }
  }
}

impl Compilable for Table {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(self.ref_type);
    self.limit.compile(buf);
  }
}

