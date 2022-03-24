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
  FUNC_REF,
  Import,
  ImportDescription,
  Limit,
  Table,
};
use super::{Module, Section};
use super::exports::ModuleExport;
use super::imports::ModuleImport;

impl Module {
  pub fn export_table(&mut self, name: String, limit: Limit) -> u32 {
    let table_idx = self.table_count;
    self.sec_table.push(Table::new(FUNC_REF, limit));
    self.sec_export.push(ModuleExport{
      export:      Export::new(name),
      description: ExportDescription::Table(table_idx),
    });
    self.table_count += 1;
    table_idx
  }

  pub fn import_table(&mut self, import: Import, limit: Limit) -> u32 {
    self.sec_import.push(ModuleImport{
      import,
      description: ImportDescription::Table(Table::new(FUNC_REF, limit)),
    });
    let table_idx = self.table_count;
    self.table_count += 1;
    table_idx
  }
}

#[derive(Debug)]
pub struct TableSection {
  tables: Vec<Table>,
}

impl TableSection {
  pub fn new() -> Self {
    Self{ tables: Vec::new() }
  }

  pub fn push(&mut self, table: Table) {
    self.tables.push(table);
  }
}

impl Default for TableSection {
  fn default() -> Self { Self::new() }
}

impl Section for TableSection {
  fn section_id(&self) -> u8 { 0x04 }

  fn len(&self) -> u32 { self.tables.len() as u32 }

  fn content(&self, _module: &Module) -> Vec<u8> {
    let mut result = Vec::new();
    for table in self.tables.iter() {
      table.compile(&mut result);
    }
    result
  }
}
