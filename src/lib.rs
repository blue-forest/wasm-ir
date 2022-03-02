// mod function;
// pub use function::Function;
// 
// mod import;
// pub use import::{Import, ImportDescription};
// 
// mod module;
// pub use module::{Module};

mod types;
pub use types::*;

// pub mod values;

pub trait Section {
  fn compile(&self) -> Vec<u8>;
}

