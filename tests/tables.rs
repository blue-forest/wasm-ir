use wasm_ir::{
  Body,
  Data,
  DataMode,
  FunctionType,
  I32,
  Import,
  Limit,
  Module,
};
use wasm_ir::code::control::{Call, CallIndirect};
use wasm_ir::code::parametric::DropStack;
use wasm_ir::code::numeric::I32Const;
use wasm_ir::code::memory::I32Store;

#[test]
fn tables() {
  let mut imported = Module::new();
  imported.set_memory(Limit::new(1, Some(1)));
  imported.add_data(Data::new(
    "test ok\n".to_string().into_bytes(),
    DataMode::Active(
      I32Const::new(420),
    ),
  ));
  imported.add_exported_function(
    FunctionType::new(vec![], vec![I32]),
    Body::new(vec![
      I32Const::new(420),
    ]),
    "get_test".to_string(),
  );

  let mut main = Module::new();
  let table_idx = main.import_table(Limit::new(1, Some(1)), Import::new(
    "test".to_string(), "table".to_string(),
  ));
  let fd_write_type = FunctionType::new(
    vec![I32, I32, I32, I32],
    vec![I32],
  );
  let (_, fd_write_idx) = main.import_function(fd_write_type, Import::new(
    "wasi_unstable".to_string(), "fd_write".to_string()
  ));

  let imported_type = main.add_function_type(
    FunctionType::new(vec![I32], vec![]),
  );
  main.add_exported_function(FunctionType::new(vec![], vec![]), Body::new(vec![
    I32Store::new(2, 0,
      I32Const::new(0), // address
      CallIndirect::new(
        imported_type, table_idx, vec![I32Const::new(1)], I32Const::new(0),
      ), // get_test() -> iovs.base
    ),
    Call::new(fd_write_idx, vec![
      I32Const::new(1),  // file_descriptor - 1 for stdout
      I32Const::new(0),  // nwritten
    ]),
    DropStack::new(),
  ]), "_start".to_string());
}
