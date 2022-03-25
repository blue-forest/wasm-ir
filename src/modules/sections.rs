use std::fmt::Debug;

use crate::values::from_u32;
use super::Module;

pub trait Section: Debug {
  fn section_id(&self) -> u8;
  fn len(&self) -> u32;
  fn is_empty(&self) -> bool { self.len() == 0 }
  fn content(&self, module: &Module) -> Vec<u8>;
  fn compile(&self, module: &Module, buf: &mut Vec<u8>) {
    if !self.is_empty() {
      let section_content = self.content(module);
      buf.push(self.section_id());
      let vec_len = from_u32(self.len());
      buf.extend(&from_u32(
        (section_content.len() + vec_len.len()) as u32
      ));
      buf.extend(&vec_len);
      buf.extend(&section_content);
    }
  }
}
