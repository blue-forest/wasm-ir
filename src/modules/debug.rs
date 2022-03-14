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
use super::Module;

impl Module {
  pub fn with_name(name: String) -> Self {
    let mut result = Self::new();
    result.sec_name.push(Box::new(ModuleName::new(name)));
    result
  }

  pub fn compile_debug(&self) -> Vec<u8> {
    let mut result = self.compile();
    if !self.sec_name.is_empty() {
      let mut section_content = Vec::new();
      for entry in self.sec_name.iter() {
        entry.compile(&mut section_content);
      }
      result.push(0x00); // custom section
      result.extend(&from_u32((section_content.len()+5) as u32)); // +5 for name
      result.push(0x04); // "name" length
      result.extend("name".as_bytes());
      result.extend(&section_content);
    }
    result
  }

  pub fn write_debug(&self, filename: &Path) -> Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(&self.compile_debug())?;
    Ok(())
  }
}

pub struct ModuleName {
  name: String,
}

impl ModuleName {
  pub fn new(name: String) -> Self {
    Self{ name }
  }
}

impl Compilable for ModuleName {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0x00); // module name
    buf.extend(&from_u32((self.name.len() + 1) as u32));
    buf.extend(&from_u32(self.name.len() as u32));
    buf.extend(self.name.as_bytes());
  }
}

