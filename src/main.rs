use std::path::Path;

use wasm_ir::{Body, FunctionType, I32, Import, Module};
use wasm_ir::code::numeric::I32Const;
use wasm_ir::code::memory::I32Store;

fn main() {
  let mut ir = Module::new();
  let fd_write_type = FunctionType::new(
    vec![I32, I32, I32, I32],
    vec![I32],
  );
  let start_type = FunctionType::new(vec![], vec![]);
  let start_body = Body::new(vec![
    Box::new(
      I32Store::new(2, 0,
        Box::new(I32Const::new(0)),
        Box::new(I32Const::new(8)),
      ),
    ),
  ]);
  ir.import_function(fd_write_type, Import::new(
    "wasi_unstable".to_string(), "fd_write".to_string()
  ));
  ir.add_function(start_type, start_body);
  ir.write(Path::new("gen.wasm")).unwrap();
}
