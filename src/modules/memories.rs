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
  Limit,
  Import,
  ImportDescription,
};

use super::{Module, Section};
use super::exports::ModuleExport;
use super::imports::ModuleImport;

impl Module {
  pub fn export_memory(&mut self, limit: Limit) {
    let mem_idx = self.sec_mem.len() as u32;
    self.sec_mem.push(limit);
    self.sec_export.push(ModuleExport{
      export:      Export::new("memory".to_string()),
      description: ExportDescription::Mem(mem_idx),
    });
  }

  pub fn import_memory(&mut self, import: Import, limit: Limit) {
    let mem_idx = self.sec_mem.len() as u32;
    self.sec_import.push(ModuleImport{
      import,
      description: ImportDescription::Mem(limit),
    });
    self.sec_export.push(ModuleExport{
      export:      Export::new("memory".to_string()),
      description: ExportDescription::Mem(mem_idx),
    });
  }
}

#[derive(Debug)]
pub struct MemorySection {
  memories: Vec<Limit>,
}

impl MemorySection {
  pub fn new() -> Self {
    Self{ memories: Vec::new() }
  }

  pub fn push(&mut self, memory: Limit) {
    self.memories.push(memory);
  }
}

impl Default for MemorySection {
  fn default() -> Self { Self::new() }
}

impl Section for MemorySection {
  fn section_id(&self) -> u8 { 0x05 }

  fn len(&self) -> u32 { self.memories.len() as u32 }

  fn content(&self, _module: &Module) -> Vec<u8> {
    let mut result = Vec::new();
    for memory in self.memories.iter() {
      memory.compile(&mut result);
    }
    result
  }
}
