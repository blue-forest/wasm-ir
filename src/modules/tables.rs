use crate::{
  Export,
  ExportDescription,
  FUNC_REF,
  Import,
  ImportDescription,
  Limit,
  Table,
};
use super::Module;
use super::exports::ModuleExport;
use super::imports::ModuleImport;

impl Module {
  pub fn export_table(&mut self, name: String, limit: Limit) -> u32 {
    let table_idx = self.table_count;
    self.sec_table.push(Box::new(Table::new(FUNC_REF, limit)));
    self.sec_export.push(Box::new(ModuleExport{
      export:      Export::new(name),
      description: ExportDescription::Table(table_idx),
    }));
    self.table_count += 1;
    table_idx
  }

  pub fn import_table(&mut self, import: Import, limit: Limit) -> u32 {
    self.sec_import.push(Box::new(ModuleImport{
      import,
      description: ImportDescription::Table(Table::new(FUNC_REF, limit)),
    }));
    let table_idx = self.table_count;
    self.table_count += 1;
    table_idx
  }
}
