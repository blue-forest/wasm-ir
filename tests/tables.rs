use wasm_ir::{
  Body,
  Data,
  DataMode,
  ElementMode,
  Export,
  FunctionType,
  I32,
  Import,
  Limit,
  Local,
  Module,
};
use wasm_ir::code::control::{Call, CallIndirect};
use wasm_ir::code::numeric::I32Const;
use wasm_ir::code::memory::I32Store;
use wasm_ir::code::parametric::DropStack;
use wasm_ir::code::variable::{LocalGet, LocalSet};

mod common;

#[test]
fn tables() {
  let mut imported = Module::new();
  imported.set_memory(Limit::new(1, Some(1)));
  imported.export_table(
    Limit::new(1, Some(1)),
    Export::new("table".to_string()),
  );
  let test_address = 420;
  let test_str = "test ok\n".to_string().into_bytes();
  imported.add_data(Data::new(
    test_str.clone(),
    DataMode::Active(
      I32Const::new(test_address),
    ),
  ));
  let imported_type = || FunctionType::new(vec![], vec![I32, I32]);
  imported.add_function_element(
    imported_type(),
    Body::new(Vec::new(), vec![
      I32Const::new(test_address),
      I32Const::new(test_str.len() as u32),
    ]),
    ElementMode::Active{
      table_idx: 0, offset: I32Const::new(0),
    },
  );

  let mut main = Module::new();
  main.import_memory(
    Import::new("env".to_string(), "memory".to_string()),
    Limit::new(1, Some(1)),
  );
  let table_idx = main.export_table(
    Limit::new(1, Some(1)),
    Export::new("table".to_string()),
  );
  let fd_write_type = FunctionType::new(
    vec![I32, I32, I32, I32],
    vec![I32],
  );
  let (_, fd_write_idx) = main.import_function(fd_write_type, Import::new(
    "wasi_unstable".to_string(), "fd_write".to_string()
  ));

  let imported_type_idx = main.add_function_type(imported_type());
  main.add_exported_function(FunctionType::new(vec![], vec![]), Body::new(
    vec![
      Local::new(1, I32),
    ],
    vec![
      I32Const::new(0), // iovs base address
      CallIndirect::new(
        imported_type_idx, table_idx, vec![I32Const::new(1)], I32Const::new(0),
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
      DropStack::new(),
    ],
  ), "_start".to_string());

  let mut embedder = common::Embedder::new();
  let imported_instance = embedder.instantiate(imported.compile());
  embedder.define_from_instance(imported_instance, "table");
  embedder.define_from_instance(imported_instance, "memory");
  use std::path::Path;
  main.write(Path::new("gen.wasm")).unwrap();
  embedder.run(main.compile());
}
