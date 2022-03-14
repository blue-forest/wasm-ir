mod common;

mod name_from_another_module;
use name_from_another_module::name_from_another_module;

use wasm_ir::ElementMode;
use wasm_ir::code::numeric::I32Const;

#[test]
fn elem_func_active_table0() {
  name_from_another_module(false, ElementMode::Active{
    table_idx: 0, offset: I32Const::create(0),
  }, "8420");
}

#[test]
fn elem_func_passive() {
  name_from_another_module(false, ElementMode::Passive, "8421");
}

#[test]
fn elem_func_active_table1() {
  name_from_another_module(false, ElementMode::Active{
    table_idx: 1, offset: I32Const::create(0),
  }, "8422");
}

#[test]
fn elem_expr_active_table0() {
  name_from_another_module(true, ElementMode::Active{
    table_idx: 0, offset: I32Const::create(0),
  }, "8424");
}

#[test]
fn elem_expr_passive() {
  name_from_another_module(true, ElementMode::Passive, "8425");
}

#[test]
fn elem_expr_active_table1() {
  name_from_another_module(true, ElementMode::Active{
    table_idx: 1, offset: I32Const::create(0),
  }, "8426");
}

