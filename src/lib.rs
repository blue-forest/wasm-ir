pub mod code;
pub use code::{Body, Instruction};

// mod import;
// pub use import::{Import, ImportDescription};

mod module;
pub use module::Module;

mod types;
pub use types::*;

pub mod values;

pub trait Compilable {
  fn compile(&self, buf: &mut Vec<u8>);
}

