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
  Export,
  ExportDescription,
  FUNC_REF,
  Import,
  ImportDescription,
  Limit,
  Table,
};
use super::Module;
use super::exports::ModuleExport;
use super::imports::ModuleImport;

impl Module {
  pub fn export_table(&mut self, name: String, limit: Limit) -> u32 {
    let table_idx = self.table_count;
    self.sec_table.push(Box::new(Table::new(FUNC_REF, limit)));
    self.sec_export.push(Box::new(ModuleExport{
      export:      Export::new(name),
      description: ExportDescription::Table(table_idx),
    }));
    self.table_count += 1;
    table_idx
  }

  pub fn import_table(&mut self, import: Import, limit: Limit) -> u32 {
    self.sec_import.push(Box::new(ModuleImport{
      import,
      description: ImportDescription::Table(Table::new(FUNC_REF, limit)),
    }));
    let table_idx = self.table_count;
    self.table_count += 1;
    table_idx
  }
}
