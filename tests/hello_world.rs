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

use std::io::Read;

use wasm_ir::{Body, Data, DataMode, FunctionType, I32, Import, Limit, Module};
use wasm_ir::code::control::Call;
use wasm_ir::code::numeric::I32Const;
use wasm_ir::code::memory::I32Store;
use wasm_ir::code::parametric::DropStack;

#[allow(dead_code)] // not used every function is used in this test...
mod common;

fn generate_hello_world() -> Module {
  let mut ir = Module::new();
  ir.export_memory(Limit::new(1, None));
  ir.add_data(Data::new(
    "hello world\n".to_string().into_bytes(),
    DataMode::Active(
      I32Const::create(8),
    ),
  ));

  let fd_write_type = FunctionType::new(
    vec![I32, I32, I32, I32],
    vec![I32],
  );
  let (_, fd_write_idx) = ir.import_function(Import::new(
    "wasi_unstable".to_string(), "fd_write".to_string()
  ), fd_write_type);

  let start_type = FunctionType::new(vec![], vec![]);
  let start_body = Body::new(Vec::new(), vec![
    I32Store::with_operands(2, 0,
      I32Const::create(0),
      I32Const::create(8),
    ),
    I32Store::with_operands(2, 0,
      I32Const::create(4),
      I32Const::create(12),
    ),
    Call::with_operands(
      fd_write_idx,
      vec![
        I32Const::create(1),  // file_descriptor - 1 for stdout
        I32Const::create(0),  // *iovs
        I32Const::create(1),  // iovs_len
        I32Const::create(20), // nwritten
      ],
    ),
    DropStack::create(),
  ]);
  ir.export_function("_start".to_string(), start_type, start_body);
  ir
}

#[test]
fn hello_world() {
  let ir = generate_hello_world();
  let binary = ir.compile();
  let mut embedder = common::Embedder::new("8420");
  embedder.run(binary);
  let mut stream = embedder.listener.incoming().next().unwrap().unwrap();
  let mut buf: [u8; 12] = [0; 12];
  stream.read(&mut buf).unwrap();
  assert_eq!(std::str::from_utf8(&buf).unwrap(), "hello world\n");
}
