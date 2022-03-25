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
  Module,
};
use crate::values::from_u32;
use super::Section;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ImportSection {
  imports: Vec<ModuleImport>,
}

impl ImportSection {
  pub fn new() -> Self {
    Self{ imports: Vec::new() }
  }

  pub fn push(&mut self, import: ModuleImport) {
    self.imports.push(import);
  }
}

impl Default for ImportSection {
  fn default() -> Self { Self::new() }
}

impl Section for ImportSection {
  fn section_id(&self) -> u8 { 0x02 }

  fn len(&self) -> u32 { self.imports.len() as u32 }

  fn content(&self, _module: &Module) -> Vec<u8> {
    let mut result = Vec::new();
    for import in self.imports.iter() {
      import.compile(&mut result);
    }
    result
  }
}
