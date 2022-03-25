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

use wasm_ir::{
  Body,
  FunctionType,
  I32,
  Import,
  Instruction,
  Limit,
  LocalBuilder,
  Module,
};
use wasm_ir::code::control::{Call, CallIndirect};
use wasm_ir::code::numeric::I32Const;
use wasm_ir::code::memory::I32Store;
use wasm_ir::code::parametric::DropStack;
use wasm_ir::code::variable::{LocalGet, LocalSet};

use super::imported;

pub fn module(is_passive: bool) -> Module {
  let mut result = Module::with_name("caller".to_string());
  result.import_memory(
    Import::new("env".to_string(), "memory".to_string()),
    Limit::new(1, Some(1)),
  );
  let table_idx = result.import_table(
    Import::new("env".to_string(), "table".to_string()),
    Limit::new(1, Some(1)),
  );
  let (_, fd_write_idx) = result.import_function(
    Import::new("wasi_unstable".to_string(), "fd_write".to_string()),
    FunctionType::new(
      vec![I32, I32, I32, I32],
      vec![I32],
    ),
  );

  let imported_type_idx = result.add_type(imported::type_());
  let mut local_builder = LocalBuilder::new();
  let instructions = start_instructions(
    &mut result,
    &mut local_builder,
    fd_write_idx,
    imported_type_idx,
    table_idx,
    is_passive,
  );
  let (_, start_idx) = result.export_function(
    "_start".to_string(),
    FunctionType::new(Vec::new(), Vec::new()),
    Body::new(local_builder, instructions),
  );
  result.set_start(start_idx);
  result
}

#[inline(always)]
fn start_instructions(
  module: &mut Module,
  local_builder: &mut LocalBuilder,
  fd_write_idx: u32,
  imported_type_idx: u32,
  table_idx: u32,
  is_passive: bool,
) -> Vec<Box<dyn Instruction>> {
  let mut result: Vec<Box<dyn Instruction>> = Vec::new();
  if is_passive {
    let (_, init_idx) = module.import_function(
      Import::new("env".to_string(), "init".to_string()),
      FunctionType::new(Vec::new(),Vec::new()),
    );
     result.push(Call::with_stack(init_idx));
  }
  let local = local_builder.add(I32);
  result.extend(vec![
    I32Const::create(0), // iovs base address
    CallIndirect::with_operands(
      imported_type_idx, table_idx, Vec::new(), I32Const::create(0),
    ), // get_test() -> iovs.base, iovs.length
    LocalSet::with_stack(local.clone()), // set iovs.length
    I32Store::with_stack(2, 0), // store iovs.base
    I32Const::create(4), // iovs length address
    LocalGet::create(local), // get iovs.length
    I32Store::with_stack(2, 0), // store iovs.length
    Call::with_operands(fd_write_idx, vec![
      I32Const::create(1),  // file_descriptor - 1 for stdout
      I32Const::create(0),  // iovs address
      I32Const::create(1),  // iovs len
      I32Const::create(8),  // nwritten
    ]),
    DropStack::create(),
  ]);
  result
}
