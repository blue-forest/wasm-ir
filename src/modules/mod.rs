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

use crate::Compilable;
use crate::values::from_u32;

mod debug;
mod exports;
mod functions;
mod imports;
mod memories;
mod segments;
mod tables;

#[derive(Debug)]
pub struct Module {
  sec_type:    Vec<Box<dyn Compilable>>,
  sec_import:  Vec<Box<dyn Compilable>>,
  sec_func:    Vec<Box<dyn Compilable>>,
  sec_table:   Vec<Box<dyn Compilable>>,
  sec_mem:     Vec<Box<dyn Compilable>>,
  // sec_global:  Vec<Box<dyn Compilable>>,
  sec_export:  Vec<Box<dyn Compilable>>,
  // sec_start:   Vec<Box<dyn Compilable>>,
  sec_elem:    Vec<Box<dyn Compilable>>,
  // data_count:  Vec<Box<dyn Compilable>>,
  sec_code:    Vec<Box<dyn Compilable>>,
  sec_data:    Vec<Box<dyn Compilable>>,
  sec_name:    Vec<Box<dyn Compilable>>,
  // sec_custom:  Vec<Box<dyn Compilable>>,
  table_count: u32,
  func_count:  u32,
}

impl Module {
  pub fn new() -> Self {
    Self{
      sec_type:    Vec::new(),
      sec_import:  Vec::new(),
      sec_func:    Vec::new(),
      sec_table:   Vec::new(),
      sec_mem:     Vec::new(),
      // sec_global:  Vec::new(),
      sec_export:  Vec::new(),
      // sec_start:   Vec::new(),
      sec_elem:    Vec::new(),
      // data_count:  Vec::new(),
      sec_code:    Vec::new(),
      sec_data:    Vec::new(),
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
    compile_section(&mut result, &self.sec_type,   0x01);
    compile_section(&mut result, &self.sec_import, 0x02);
    compile_section(&mut result, &self.sec_func,   0x03);
    compile_section(&mut result, &self.sec_table,  0x04);
    compile_section(&mut result, &self.sec_mem,    0x05);
    compile_section(&mut result, &self.sec_export, 0x07);
    compile_section(&mut result, &self.sec_elem,   0x09);
    compile_section(&mut result, &self.sec_code,   0x0a);
    compile_section(&mut result, &self.sec_data,   0x0b);
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

fn compile_section(
  buf:        &mut Vec<u8>,
  section:    &[Box<dyn Compilable>],
  section_id: u8,
) {
  if !section.is_empty() {
    let mut section_content = Vec::new();
    for entry in section.iter() {
      entry.compile(&mut section_content);
    }
    buf.push(section_id);
    let vec_len = from_u32(section.len() as u32);
    buf.extend(&from_u32(
      (section_content.len() + vec_len.len()) as u32
    ));
    buf.extend(&vec_len);
    buf.extend(&section_content);
  }
}

