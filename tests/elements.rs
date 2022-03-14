/*
 * wasm-ir - Intermediate Representation of WebAssembly
 * Copyright © 2019-2022 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

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

