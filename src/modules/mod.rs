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

use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

use crate::{Compilable, StartFunction};

mod code;
use code::CodeSection;
mod data;
use data::DataSection;
mod debug;
mod elements;
use elements::ElementSection;
mod exports;
use exports::ExportSection;
mod functions;
use functions::FunctionSection;
mod imports;
use imports::ImportSection;
mod memories;
use memories::MemorySection;
mod sections;
pub use sections::Section;
mod tables;
use tables::TableSection;
mod types;
use types::TypeSection;

#[derive(Debug)]
pub struct Module {
  sec_type:    TypeSection,
  sec_import:  ImportSection,
  sec_func:    FunctionSection,
  sec_table:   TableSection,
  sec_mem:     MemorySection,
  // sec_global:  Vec<Box<dyn Compilable>>,
  sec_export:  ExportSection,
  sec_start:   Option<StartFunction>,
  sec_elem:    ElementSection,
  // data_count:  Vec<Box<dyn Compilable>>,
  sec_code:    CodeSection,
  sec_data:    DataSection,
  sec_name:    Vec<Box<dyn Compilable>>,
  // sec_custom:  Vec<Box<dyn Compilable>>,
  table_count: u32,
  func_count:  u32,
}

impl Module {
  pub fn new() -> Self {
    Self{
      sec_type:    TypeSection::new(),
      sec_import:  ImportSection::new(),
      sec_func:    FunctionSection::new(),
      sec_table:   TableSection::new(),
      sec_mem:     MemorySection::new(),
      // sec_global:  Vec::new(),
      sec_export:  ExportSection::new(),
      sec_start:   None,
      sec_elem:    ElementSection::new(),
      // data_count:  Vec::new(),
      sec_code:    CodeSection::new(),
      sec_data:    DataSection::new(),
      sec_name:    Vec::new(),
      // sec_custom:  Vec::new(),
      table_count: 0,
      func_count:  0,
    }
  }

  pub fn compile(&self) -> Vec<u8> {
    let mut result = vec![
      0x00, 0x61, 0x73, 0x6d, // magic
      0x01, 0x00, 0x00, 0x00, // version
    ];
    self.sec_type.compile(self, &mut result);
    self.sec_import.compile(self, &mut result);
    self.sec_func.compile(self, &mut result);
    self.sec_table.compile(self, &mut result);
    self.sec_mem.compile(self, &mut result);
    self.sec_export.compile(self, &mut result);
    if let Some(start) = &self.sec_start {
      start.compile(&mut result);
    }
    self.sec_elem.compile(self, &mut result);
    self.sec_code.compile(self, &mut result);
    self.sec_data.compile(self, &mut result);
    result
  }

  pub fn write(&self, filename: &Path) -> Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(&self.compile())?;
    Ok(())
  }
}

impl Default for Module {
  fn default() -> Self { Self::new() }
}

