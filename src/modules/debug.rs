use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

use crate::Compilable;
use crate::values::from_u32;
use super::Module;

impl Module {
  pub fn compile_debug(&self) -> Vec<u8> {
    let mut result = self.compile();
    if !self.sec_name.is_empty() {
      let mut section_content = Vec::new();
      for entry in self.sec_name.iter() {
        entry.compile(&mut section_content);
      }
      result.push(0x00); // custom section
      result.extend(&from_u32((section_content.len()+5) as u32)); // +5 for name
      result.push(0x04); // "name" length
      result.extend("name".as_bytes());
      result.extend(&section_content);
    }
    result
  }

  pub fn with_name(mut self, name: String) -> Self {
    self.sec_name.push(Box::new(ModuleName::new(name)));
    self
  }

  pub fn write_debug(&self, filename: &Path) -> Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(&self.compile_debug())?;
    Ok(())
  }
}

pub struct ModuleName {
  name: String,
}

impl ModuleName {
  pub fn new(name: String) -> Self {
    Self{ name }
  }
}

impl Compilable for ModuleName {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0x00); // module name
    buf.extend(&from_u32((self.name.len() + 1) as u32));
    buf.extend(&from_u32(self.name.len() as u32));
    buf.extend(self.name.as_bytes());
  }
}

