use std::path::Path;

use wasm_ir::{Body, Data, DataMode, FunctionType, I32, Import, Limit, Module};
use wasm_ir::code::control::Call;
use wasm_ir::code::numeric::I32Const;
use wasm_ir::code::memory::I32Store;
use wasm_ir::code::parametric::DropStack;

fn main() {
  let mut ir = Module::new();
  ir.set_memory(Limit::new(1, None));
  ir.add_data(Data::new(
    "hello world\n".to_string().into_bytes(),
    DataMode::Active(
      Box::new(I32Const::new(8)),
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
    Box::new(
      I32Store::new(2, 0,
        Box::new(I32Const::new(0)),
        Box::new(I32Const::new(8)),
      ),
    ),
    Box::new(
      I32Store::new(2, 0,
        Box::new(I32Const::new(4)),
        Box::new(I32Const::new(12)),
      ),
    ),
    Box::new(Call::new(
      fd_write_idx,
      vec![
        Box::new(I32Const::new(1)),
        Box::new(I32Const::new(0)),
        Box::new(I32Const::new(1)),
        Box::new(I32Const::new(20)),
      ],
    )),
    Box::new(DropStack{}),
  ]);
  ir.add_exported_function(start_type, start_body, "_start".to_string());
  ir.write(Path::new("gen.wasm")).unwrap();
}
