use std::io::Read;

use wasm_ir::{
  Body,
  Data,
  DataMode,
  FunctionType,
  I32,
  I64,
  Import,
  Instruction,
  Limit,
  Local,
  LocalBuilder,
  Module
};
use wasm_ir::code::control::Call;
use wasm_ir::code::memory::I32Store;
use wasm_ir::code::numeric::{I32Const, I64Const, I32WrapI64};
use wasm_ir::code::parametric::DropStack;
use wasm_ir::code::variable::{LocalGet, LocalSet};

#[allow(dead_code)] // not used every function is used in this test...
mod common;

#[test]
fn locals() {
  let mut ir = Module::new();

  let (_, fd_write_idx) = ir.import_function(
    Import::new(
      "wasi_unstable".to_string(), "fd_write".to_string()
    ),
    FunctionType::new(
      vec![I32, I32, I32, I32],
      vec![I32],
    ),
  );

  ir.export_memory(Limit::new(1, None));
  let mut local_builder = LocalBuilder::new();

  ir.add_data(Data::new(
    "param 1\n".to_string().into_bytes(),
    DataMode::Active(
      I32Const::with_const(8),
    ),
  ));
  ir.add_data(Data::new(
    "local 1\n".to_string().into_bytes(),
    DataMode::Active(
      I32Const::with_const(16),
    ),
  ));
  let local1 = local_builder.add(I32);
  ir.add_data(Data::new(
    "local 2\n".to_string().into_bytes(),
    DataMode::Active(
      I32Const::with_const(24),
    ),
  ));
  let local2 = local_builder.add(I64);
  ir.add_data(Data::new(
    "local 3\n".to_string().into_bytes(),
    DataMode::Active(
      I32Const::with_const(32),
    ),
  ));
  let local3 = local_builder.add(I32);

  let mut called_instructions = vec![
    LocalSet::with_operands(local1.clone(), I32Const::create(16)),
    LocalSet::with_operands(local2.clone(), I64Const::create(24)),
    LocalSet::with_operands(local3.clone(), I32Const::create(32)),
    I32Store::with_operands(2, 0,
      I32Const::create(4),
      I32Const::create(8),
    ),
  ];
  called_instructions.extend(print_local(
    fd_write_idx, LocalGet::create(local1),
  ));
  called_instructions.extend(print_local(
    fd_write_idx, LocalGet::create(Local::with_param(0)),
  ));
  called_instructions.extend(print_local(
    fd_write_idx, I32WrapI64::with_operands(LocalGet::create(local2)),
  ));
  called_instructions.extend(print_local(
    fd_write_idx, LocalGet::create(local3),
  ));
  let (_, called_idx) = ir.add_function(
    FunctionType::new(vec![I32], Vec::new()),
    Body::new(local_builder, called_instructions),
  );
  
  let (_, start_idx) = ir.export_function(
    "_start".to_string(),
    FunctionType::new(Vec::new(), Vec::new()),
    Body::new(
      LocalBuilder::new(),
      vec![
        Call::with_operands(called_idx, vec![I32Const::create(8)]),
      ],
    ),
  );
  ir.set_start(start_idx);
  ir.write_debug(&std::path::Path::new("gen.wasm")).unwrap();
  let binary = ir.compile();
  let mut embedder = common::Embedder::new("8420");

  embedder.run(binary);
  let mut stream = embedder.listener.incoming().next().unwrap().unwrap();
  let mut buf: [u8; 8] = [0; 8];
  stream.read(&mut buf).unwrap();
  assert_eq!(std::str::from_utf8(&buf).unwrap(), "local 1\n");
  stream.read(&mut buf).unwrap();
  assert_eq!(std::str::from_utf8(&buf).unwrap(), "param 1\n");
  stream.read(&mut buf).unwrap();
  assert_eq!(std::str::from_utf8(&buf).unwrap(), "local 2\n");
  stream.read(&mut buf).unwrap();
  assert_eq!(std::str::from_utf8(&buf).unwrap(), "local 3\n");
}

fn print_local(
  fd_write_idx: u32,
  local: Box<dyn Instruction>,
) -> Vec<Box<dyn Instruction>> {
  vec![
    I32Store::with_operands(2, 0,
      I32Const::create(0), local,
    ),
    Call::with_operands(
      fd_write_idx,
      vec![
        I32Const::create(1),  // file_descriptor - 1 for stdout
        I32Const::create(0),  // *iovs
        I32Const::create(1),  // iovs_len
        I32Const::create(40), // nwritten
      ],
    ),
    DropStack::create(),
  ]
}
