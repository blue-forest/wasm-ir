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

use wasmtime::*;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::sync::net::TcpStream;

use cap_std;
use std::net;
use std::net::TcpListener;

pub struct Embedder {
  pub engine:   Engine,
  pub linker:   Linker<WasiCtx>,
  pub listener: TcpListener,
  pub store:    Store<WasiCtx>,
}

impl Embedder {
  pub fn new(port: &str) -> Self {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    let stream = cap_std::net::TcpStream::from_std(
      net::TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap()
    );
    let wasi_ctx = WasiCtxBuilder::new()
      .stdout(Box::new(TcpStream::from_cap_std(stream)))
      .build();

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
    let store = Store::new(&engine, wasi_ctx);
    Embedder{ listener, engine, linker, store }
  }

  pub fn define_from_instance(&mut self, instance: Instance, name: &str) {
    let exported = instance.get_export(&mut self.store, name).unwrap();
    self.linker.define("env", name, exported).unwrap();
  }
  
  pub fn instantiate(&mut self, binary: Vec<u8>) -> Instance {
    let module = Module::new(&self.engine, binary).unwrap();
    self.linker.instantiate(
      &mut self.store, &module,
    ).unwrap()
  }

  pub fn run(&mut self, binary: Vec<u8>) {
    self.instantiate(binary);
    /*
    let start = self.instantiate(binary).get_typed_func::<(), (), _>(
      &mut self.store, "_start"
    ).unwrap();
    start.call(&mut self.store, ()).unwrap();
    */
  }
}
