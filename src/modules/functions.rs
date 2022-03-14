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
  Body,
  Import,
  ImportDescription,
  Export,
  ExportDescription,
  Function,
  FunctionType,
};

use super::Module;
use super::exports::ModuleExport;
use super::imports::ModuleImport;

impl Module {
  pub fn add_function(
    &mut self,
    profile: FunctionType,
    body:    Body,
  ) -> (u32, u32) {
    let type_idx = self.add_type(profile);
    let function_idx = self.set_function_body(type_idx, body);
    (type_idx, function_idx)
  }

  pub fn add_type(
    &mut self,
    profile: FunctionType,
  ) -> u32 {
    let type_idx = self.sec_type.len() as u32;
    self.sec_type.push(Box::new(profile));
    type_idx
  }

  pub fn export_function(
    &mut self,
    name:    String,
    profile: FunctionType,
    body:    Body,
  ) -> (u32, u32) {
    let (type_idx, function_idx) = self.add_function(profile, body);
    self.sec_export.push(Box::new(ModuleExport{
      export:      Export::new(name),
      description: ExportDescription::Func(function_idx),
    }));
    (type_idx, function_idx)
  }

  pub fn import_function(
    &mut self,
    import:  Import,
    profile: FunctionType,
  ) -> (u32, u32) {
    let type_idx = self.sec_type.len() as u32;
    self.sec_type.push(Box::new(profile));
    self.sec_import.push(Box::new(ModuleImport{
      import,
      description: ImportDescription::Func(type_idx),
    }));
    let function_idx = self.func_count;
    self.func_count += 1;
    (type_idx, function_idx)
  }

  fn set_function_body(&mut self, type_idx: u32, body: Body) -> u32 {
    self.sec_func.push(Box::new(Function::new(type_idx)));
    self.sec_code.push(Box::new(body));
    let function_idx = self.func_count;
    self.func_count += 1;
    function_idx
  }
}
