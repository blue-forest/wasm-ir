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

use crate::{
  Compilable,
  Import,
  ImportDescription,
};
use crate::values::from_u32;

pub struct ModuleImport {
  pub import:      Import,
  pub description: ImportDescription,
}

impl Compilable for ModuleImport {
  fn compile(&self, buf: &mut Vec<u8>) {
    self.import.compile(buf);
    match &self.description {
      ImportDescription::Func(type_idx) => {
        buf.push(0x00);
        buf.extend(&from_u32(*type_idx));
      }
      ImportDescription::Table(table) => {
        buf.push(0x01);
        table.compile(buf);
      }
      ImportDescription::Mem(limit) => {
        buf.push(0x02);
        limit.compile(buf);
      }
    }
  }
}
