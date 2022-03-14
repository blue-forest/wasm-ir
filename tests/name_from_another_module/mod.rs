use std::io::Read;

use wasm_ir::ElementMode;

use super::common;

mod caller;
mod imported;

pub fn name_from_another_module(
  is_expr: bool,
  mode: ElementMode,
  port: &str,
) {
  let is_passive = matches!(mode, ElementMode::Passive);
  let imported_module = imported::module(is_expr, mode);
  // use std::path::Path;
  // if port == "8421" {
  //   imported.write_debug(Path::new("imported.wasm")).unwrap();
  // }

  let caller_module = caller::module(is_passive);
  // if port == "8421" {
  //   caller_module.write_debug(Path::new("caller.wasm")).unwrap();
  // }

  let mut embedder = common::Embedder::new(port);
  let imported_instance = embedder.instantiate(imported_module.compile_debug());
  embedder.define_from_instance(imported_instance, "table");
  embedder.define_from_instance(imported_instance, "memory");
  if is_passive {
    embedder.define_from_instance(imported_instance, "init");
  }
  // println!("imported ok");
  embedder.run(caller_module.compile_debug());
  let mut stream = embedder.listener.incoming().next().unwrap().unwrap();
  let mut buf: [u8; 8] = [0; 8];
  stream.read(&mut buf).unwrap();
  assert_eq!(std::str::from_utf8(&buf).unwrap(), "test ok\n");
}

