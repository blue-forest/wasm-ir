use wasm_ir::{
  Body,
  Data,
  DataMode,
  ElementMode,
  FunctionType,
  I32,
  Limit,
  Module,
};
use wasm_ir::code::numeric::I32Const;
use wasm_ir::code::reference::RefFunc;
use wasm_ir::code::table::TableInit;

#[inline(always)]
fn get_name(is_expr: bool, module: &mut Module, mode: ElementMode) {
  let test_address = 420;
  let test_str = "test ok\n".to_string().into_bytes();
  module.add_data(Data::new(
    test_str.clone(),
    DataMode::Active(
      I32Const::new(test_address),
    ),
  ));

  let body = Body::new(Vec::new(), vec![
    I32Const::new(test_address),
    I32Const::new(test_str.len() as u32)
  ]);
  if is_expr {
    let (_, function_idx) = module.add_function(
      type_(), body,
    );
    module.add_expression_element(vec![
      RefFunc::new(function_idx),
    ], mode);
  } else {
    module.add_function_element(
      type_(), body, mode,
    );
  };
}

#[inline(always)]
fn init(module: &mut Module) {
  module.export_function(
    "init".to_string(),
    FunctionType::new(Vec::new(), Vec::new()),
    Body::new(Vec::new(), vec![
      TableInit::new(0, 0,
        I32Const::new(0), // region start
        I32Const::new(0), // offset
        I32Const::new(1), // region size
      ),
    ]),
  );
}

pub fn module(is_expr: bool, mode: ElementMode) -> Module {
  let is_passive = matches!(mode, ElementMode::Passive);
  let mut result = Module::new().with_name("imported".to_string());
  result.export_memory(Limit::new(1, Some(1)));
  tables(&mut result, &mode);
  get_name(is_expr, &mut result, mode);
  if is_passive {
    init(&mut result);
  }
  result
}

#[inline(always)]
fn tables(module: &mut Module, mode: &ElementMode) {
  if let ElementMode::Active{ table_idx, offset: _ } = mode {
    if *table_idx == 1 {
      module.export_table(
        "wrong_table".to_string(),
        Limit::new(1, Some(1)),
      );
    }
  }
  module.export_table(
    "table".to_string(),
    Limit::new(1, Some(1)),
  );
}

pub fn type_() -> FunctionType {
  FunctionType::new(Vec::new(), vec![I32, I32])
}
