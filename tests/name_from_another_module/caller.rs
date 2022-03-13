use wasm_ir::{
  Body,
  FunctionType,
  I32,
  Import,
  Instruction,
  Limit,
  Local,
  Module,
};
use wasm_ir::code::control::{Call, CallIndirect};
use wasm_ir::code::numeric::I32Const;
use wasm_ir::code::memory::I32Store;
use wasm_ir::code::parametric::DropStack;
use wasm_ir::code::variable::{LocalGet, LocalSet};

use super::imported;

pub fn module(is_passive: bool) -> Module {
  let mut result = Module::new().with_name("caller".to_string());
  result.import_memory(
    Import::new("env".to_string(), "memory".to_string()),
    Limit::new(1, Some(1)),
  );
  let table_idx = result.import_table(
    Limit::new(1, Some(1)),
    Import::new("env".to_string(), "table".to_string()),
  );
  let (_, fd_write_idx) = result.import_function(
    FunctionType::new(
      vec![I32, I32, I32, I32],
      vec![I32],
    ),
    Import::new("wasi_unstable".to_string(), "fd_write".to_string()),
  );

  let imported_type_idx = result.add_function_type(imported::type_());
  let instructions = start_instructions(
    &mut result,
    fd_write_idx,
    imported_type_idx,
    table_idx,
    is_passive,
  );
  result.add_exported_function(
    FunctionType::new(Vec::new(), Vec::new()),
    Body::new(
      vec![
        Local::new(1, I32),
      ],
      instructions,
    ), "_start".to_string(),
  );
  result
}

#[inline(always)]
fn start_instructions(
  module: &mut Module,
  fd_write_idx: u32,
  imported_type_idx: u32,
  table_idx: u32,
  is_passive: bool,
) -> Vec<Box<dyn Instruction>> {
  let mut result: Vec<Box<dyn Instruction>> = Vec::new();
  if is_passive {
    let (_, init_idx) = module.import_function(
      FunctionType::new(Vec::new(),Vec::new()),
      Import::new("env".to_string(), "init".to_string()),
    );
     result.push(Call::new(init_idx, Vec::new()));
  }
  result.extend(vec![
    I32Const::new(0), // iovs base address
    CallIndirect::new(
      imported_type_idx, table_idx, Vec::new(), I32Const::new(0),
    ), // get_test() -> iovs.base, iovs.length
    LocalSet::new(0), // set iovs.length
    I32Store::new_stacked(2, 0), // store iovs.base
    I32Const::new(4), // iovs length address
    LocalGet::new(0), // get iovs.length
    I32Store::new_stacked(2, 0), // store iovs.length
    Call::new(fd_write_idx, vec![
      I32Const::new(1),  // file_descriptor - 1 for stdout
      I32Const::new(0),  // iovs address
      I32Const::new(1),  // iovs len
      I32Const::new(8),  // nwritten
    ]),
    DropStack::new(),
  ]);
  result
}
