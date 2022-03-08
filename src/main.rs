// This code is free software distributed under GPLv3 by Blue Forest.

use std::path::Path;

use wasm_ir::{Body, Data, DataMode, FunctionType, I32, Import, Limit, Module};
use wasm_ir::code::control::Call;
use wasm_ir::code::numeric::I32Const;
use wasm_ir::code::memory::I32Store;
use wasm_ir::code::parametric::DropStack;

fn main() {
  let mut ir = Module::new();
  ir.set_memory(Limit::new(1, None));
  ir.import_table(Limit::new(1, None));
  ir.add_data(Data::new(
    "hello world\n".to_string().into_bytes(),
    DataMode::Active(
      I32Const::new(8),
    ),
  ));

  let fd_write_type = FunctionType::new(
    vec![I32, I32, I32, I32],
    vec![I32],
  );
  let fd_write_idx = ir.import_function(fd_write_type, Import::new(
    "wasi_unstable".to_string(), "fd_write".to_string()
  ));

  let start_type = FunctionType::new(vec![], vec![]);
  let start_body = Body::new(vec![
    I32Store::new(2, 0,
      I32Const::new(0),
      I32Const::new(8),
    ),
    I32Store::new(2, 0,
      I32Const::new(4),
      I32Const::new(12),
    ),
    Call::new(
      fd_write_idx,
      vec![
        I32Const::new(1),
        I32Const::new(0),
        I32Const::new(1),
        I32Const::new(20),
      ],
    ),
    DropStack::new(),
  ]);
  ir.add_exported_function(start_type, start_body, "_start".to_string());
  ir.write(Path::new("gen.wasm")).unwrap();
}
