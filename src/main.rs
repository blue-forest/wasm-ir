use std::path::Path;

use wasm_ir::{Body, FunctionType, I32, Import, Limit, Module};
use wasm_ir::code::numeric::I32Const;
use wasm_ir::code::memory::I32Store;

fn main() {
  let mut ir = Module::new();
  ir.set_memory(Limit::new(1, None));

  let fd_write_type = FunctionType::new(
    vec![I32, I32, I32, I32],
    vec![I32],
  );
  ir.import_function(fd_write_type, Import::new(
    "wasi_unstable".to_string(), "fd_write".to_string()
  ));

  let start_type = FunctionType::new(vec![], vec![]);
  let start_body = Body::new(vec![
    Box::new(
      I32Store::new(2, 0,
        Box::new(I32Const::new(0)),
        Box::new(I32Const::new(8)),
      ),
    ),
  ]);

  ir.add_exported_function(start_type, start_body, "_start".to_string());
  ir.write(Path::new("gen.wasm")).unwrap();
}
