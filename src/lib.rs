pub mod code;
pub use code::{Body, Instruction};

mod function;
pub use function::Function;

mod import_export;
pub use import_export::{Export, Import, ImportExportDescription};

mod module;
pub use module::Module;

mod types;
pub use types::*;

pub mod values;

pub trait Compilable {
  fn compile(&self, buf: &mut Vec<u8>);
}

