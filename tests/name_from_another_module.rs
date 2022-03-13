use std::io::Read;

use wasm_ir::{
  Body,
  Data,
  DataMode,
  ElementMode,
  Export,
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
use wasm_ir::code::reference::RefFunc;
use wasm_ir::code::table::TableInit;
use wasm_ir::code::variable::{LocalGet, LocalSet};

mod common;

fn name_from_another_module(
  is_expr: bool,
  mode: ElementMode,
  port: &str,
) {
  let mut imported = Module::new().with_name("imported".to_string());
  imported.set_memory(Limit::new(1, Some(1)));
  if let ElementMode::Active{ table_idx, offset: _ } = mode {
    if table_idx == 1 {
      imported.export_table(
        Limit::new(1, Some(1)),
        Export::new("wrong_table".to_string()),
      );
    }
  }
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
  let imported_type = || FunctionType::new(Vec::new(), vec![I32, I32]);
  let imported_body = Body::new(Vec::new(), vec![
    I32Const::new(test_address),
    I32Const::new(test_str.len() as u32)
  ]);
  let is_passive = if let ElementMode::Passive = mode { true } else { false };
  if is_expr {
    let (_, function_idx) = imported.add_function(
      imported_type(), imported_body,
    );
    imported.add_expression_element(vec![
      RefFunc::new(function_idx),
    ], mode);
  } else {
    imported.add_function_element(
      imported_type(), imported_body, mode,
    );
  };
  if is_passive {
    imported.add_exported_function(
      FunctionType::new(Vec::new(), Vec::new()),
      Body::new(Vec::new(), vec![
        TableInit::new(0, 0,
          I32Const::new(0), // region start
          I32Const::new(0), // offset
          I32Const::new(1), // region size
        ),
      ]),
      "init".to_string(),
    );
  }
  // use std::path::Path;
  // if port == "8421" {
  //   imported.write_debug(Path::new("imported.wasm")).unwrap();
  // }

  let mut main = Module::new().with_name("main".to_string());
  main.import_memory(
    Import::new("env".to_string(), "memory".to_string()),
    Limit::new(1, Some(1)),
  );
  let table_idx = main.import_table(
    Limit::new(1, Some(1)),
    Import::new("env".to_string(), "table".to_string()),
  );
  let fd_write_type = FunctionType::new(
    vec![I32, I32, I32, I32],
    vec![I32],
  );
  let (_, fd_write_idx) = main.import_function(fd_write_type, Import::new(
    "wasi_unstable".to_string(), "fd_write".to_string()
  ));

  let imported_type_idx = main.add_function_type(imported_type());
  let mut main_instructions: Vec<Box<dyn Instruction>> = Vec::new();
  if is_passive {
    let (_, init_idx) = main.import_function(
      FunctionType::new(Vec::new(),Vec::new()),
      Import::new(
        "env".to_string(), "init".to_string()
      ),
    );
     main_instructions.push(Call::new(init_idx, Vec::new()));
  }
  main_instructions.push(I32Const::new(0)); // iovs base address
  main_instructions.push(CallIndirect::new(
    imported_type_idx, table_idx, Vec::new(), I32Const::new(0),
  )); // get_test() -> iovs.base, iovs.length
  main_instructions.push(LocalSet::new(0)); // set iovs.length
  main_instructions.push(I32Store::new_stacked(2, 0)); // store iovs.base
  main_instructions.push(I32Const::new(4)); // iovs length address
  main_instructions.push(LocalGet::new(0)); // get iovs.length
  main_instructions.push(I32Store::new_stacked(2, 0)); // store iovs.length
  main_instructions.push(Call::new(fd_write_idx, vec![
    I32Const::new(1),  // file_descriptor - 1 for stdout
    I32Const::new(0),  // iovs address
    I32Const::new(1),  // iovs len
    I32Const::new(8),  // nwritten
  ]));
  main_instructions.push(DropStack::new());
  main.add_exported_function(
    FunctionType::new(Vec::new(), Vec::new()),
    Body::new(
      vec![
        Local::new(1, I32),
      ],
      main_instructions,
    ), "_start".to_string(),
  );
  // if port == "8421" {
  //   main.write_debug(Path::new("main.wasm")).unwrap();
  // }

  let mut embedder = common::Embedder::new(port);
  let imported_instance = embedder.instantiate(imported.compile_debug());
  embedder.define_from_instance(imported_instance, "table");
  embedder.define_from_instance(imported_instance, "memory");
  if is_passive {
    embedder.define_from_instance(imported_instance, "init");
  }
  // println!("imported ok");
  embedder.run(main.compile_debug());
  let mut stream = embedder.listener.incoming().next().unwrap().unwrap();
  let mut buf: [u8; 8] = [0; 8];
  stream.read(&mut buf).unwrap();
  assert_eq!(std::str::from_utf8(&buf).unwrap(), "test ok\n");
}

#[test]
fn elem00() {
  name_from_another_module(false, ElementMode::Active{
    table_idx: 0, offset: I32Const::new(0),
  }, "8420");
}

#[test]
fn elem01() {
  name_from_another_module(false, ElementMode::Passive, "8421");
}

#[test]
fn elem02() {
  name_from_another_module(false, ElementMode::Active{
    table_idx: 1, offset: I32Const::new(0),
  }, "8422");
}

#[test]
fn elem04() {
  name_from_another_module(true, ElementMode::Active{
    table_idx: 0, offset: I32Const::new(0),
  }, "8424");
}

#[test]
fn elem05() {
  name_from_another_module(true, ElementMode::Passive, "8425");
}

#[test]
fn elem06() {
  name_from_another_module(true, ElementMode::Active{
    table_idx: 1, offset: I32Const::new(0),
  }, "8426");
}

