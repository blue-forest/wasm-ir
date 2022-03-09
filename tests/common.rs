use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::sync::net::TcpStream;

use cap_std;
use std::net;
use std::net::TcpListener;

pub fn run(binary: Vec<u8>) -> TcpListener {
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
  let mut store = Store::new(&engine, wasi_ctx);

  let test = Module::new(&engine, binary).unwrap();
  let instance = linker.instantiate(&mut store, &test).unwrap();
  let start = instance.get_typed_func::<(), (), _>(
    &mut store, "_start"
  ).unwrap();
  start.call(&mut store, ()).unwrap();
  listener
}
