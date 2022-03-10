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
  pub fn new() -> Self {
    let listener = TcpListener::bind("127.0.0.1:8420").unwrap();
    let stream = cap_std::net::TcpStream::from_std(
      net::TcpStream::connect("127.0.0.1:8420").unwrap()
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
    let start = self.instantiate(binary).get_typed_func::<(), (), _>(
      &mut self.store, "_start"
    ).unwrap();
    start.call(&mut self.store, ()).unwrap();
  }
}