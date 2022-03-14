use crate::{
  Export,
  ExportDescription,
  Limit,
  Import,
  ImportDescription,
};

use super::Module;
use super::exports::ModuleExport;
use super::imports::ModuleImport;

impl Module {
  pub fn export_memory(&mut self, limit: Limit) {
    let mem_idx = self.sec_mem.len() as u32;
    self.sec_mem.push(Box::new(limit));
    self.sec_export.push(Box::new(ModuleExport{
      export:      Export::new("memory".to_string()),
      description: ExportDescription::Mem(mem_idx),
    }));
  }

  pub fn import_memory(&mut self, import: Import, limit: Limit) {
    let mem_idx = self.sec_mem.len() as u32;
    self.sec_import.push(Box::new(ModuleImport{
      import,
      description: ImportDescription::Mem(limit),
    }));
    self.sec_export.push(Box::new(ModuleExport{
      export:      Export::new("memory".to_string()),
      description: ExportDescription::Mem(mem_idx),
    }));
  }
}
