// This code is free software distributed under GPLv3 by Blue Forest.

pub mod code;
pub use code::{Body, Instruction};

mod data;
pub use data::{Data, DataMode};

mod exports;
pub use exports::{Export, ExportDescription};

mod functions;
pub use functions::Function;

mod imports;
pub use imports::{Import, ImportDescription};

mod modules;
pub use modules::Module;

mod tables;
pub use tables::Table;

mod types;
pub use types::*;

pub mod values;

pub trait Compilable {
  fn compile(&self, buf: &mut Vec<u8>);
}

