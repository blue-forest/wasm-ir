use crate::{
  Compilable,
  Export,
  ExportDescription,
};
use crate::values::from_u32;

pub struct ModuleExport {
  pub export:      Export,
  pub description: ExportDescription,
}

impl Compilable for ModuleExport {
  fn compile(&self, buf: &mut Vec<u8>) {
    self.export.compile(buf);
    match self.description {
      ExportDescription::Func(type_idx) => {
        buf.push(0x00);
        buf.extend(&from_u32(type_idx));
      }
      ExportDescription::Table(table_idx) => {
        buf.push(0x01);
        buf.extend(&from_u32(table_idx));
      }
      ExportDescription::Mem(mem_idx) => {
        buf.push(0x02);
        buf.extend(&from_u32(mem_idx));
      }
      _ => { todo!() }
    }
  }
}
