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
  Export,
  ExportDescription,
};
use crate::values::from_u32;
use super::{Module, Section};

#[derive(Debug)]
pub struct ModuleExport {
  pub export:      Export,
  pub description: ExportDescription,
}

impl Compilable for ModuleExport {
  fn compile(&self, buf: &mut Vec<u8>) {
    self.export.compile(buf);
    match self.description {
      ExportDescription::Func(type_idx) => {
        buf.push(0x00);
        buf.extend(&from_u32(type_idx));
      }
      ExportDescription::Table(table_idx) => {
        buf.push(0x01);
        buf.extend(&from_u32(table_idx));
      }
      ExportDescription::Mem(mem_idx) => {
        buf.push(0x02);
        buf.extend(&from_u32(mem_idx));
      }
      _ => { todo!() }
    }
  }
}

#[derive(Debug)]
pub struct ExportSection {
  exports: Vec<ModuleExport>,
}

impl ExportSection {
  pub fn new() -> Self {
    Self{ exports: Vec::new() }
  }

  pub fn push(&mut self, export: ModuleExport) {
    self.exports.push(export);
  }
}

impl Default for ExportSection {
  fn default() -> Self { Self::new() }
}

impl Section for ExportSection {
  fn section_id(&self) -> u8 { 0x07 }

  fn len(&self) -> u32 { self.exports.len() as u32 }

  fn content(&self, _module: &Module) -> Vec<u8> {
    let mut result = Vec::new();
    for export in self.exports.iter() {
      export.compile(&mut result);
    }
    result
  }
}
