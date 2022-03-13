use crate::{
  Compilable,
  Import,
  ImportDescription,
};
use crate::values::from_u32;

pub struct ModuleImport {
  pub import:      Import,
  pub description: ImportDescription,
}

impl Compilable for ModuleImport {
  fn compile(&self, buf: &mut Vec<u8>) {
    self.import.compile(buf);
    match &self.description {
      ImportDescription::Func(type_idx) => {
        buf.push(0x00);
        buf.extend(&from_u32(*type_idx));
      }
      ImportDescription::Table(table) => {
        buf.push(0x01);
        table.compile(buf);
      }
      ImportDescription::Mem(limit) => {
        buf.push(0x02);
        limit.compile(buf);
      }
    }
  }
}
