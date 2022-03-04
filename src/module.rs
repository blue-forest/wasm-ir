use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

use crate::{
  Body,
  Compilable,
  Export,
  FunctionType,
  Import,
  ImportExportDescription,
};
use crate::values::from_u32;

pub struct Module {
  sec_type:   Vec<FunctionType>,
  sec_import: Vec<ModuleImport>,
  sec_func:   Vec<u32>,
  // sec_table:  Vec<Vec<u8>>,
  // sec_mem:    Vec<Vec<u8>>,
  // sec_global: Vec<Vec<u8>>,
  sec_export: Vec<ModuleExport>,
  // sec_start:  Vec<Vec<u8>>,
  // sec_elem:   Vec<Vec<u8>>,
  // data_count: Vec<Vec<u8>>,
  sec_code:   Vec<Body>,
  // sec_data:   Vec<Vec<u8>>,
  // sec_custom: Vec<Vec<u8>>,
}

impl Module {
  pub fn new() -> Self {
    Self{
      sec_type:   Vec::new(),
      sec_import: Vec::new(),
      sec_func:   Vec::new(),
      // sec_table:  Vec::new(),
      // sec_mem:    Vec::new(),
      // sec_global: Vec::new(),
      sec_export: Vec::new(),
      // sec_start:  Vec::new(),
      // sec_elem:   Vec::new(),
      // data_count: Vec::new(),
      sec_code:   Vec::new(),
      // sec_data:   Vec::new(),
      // sec_custom: Vec::new(),
    }
  }

  pub fn import_function(&mut self, profile: FunctionType, import: Import) {
    let type_idx = self.sec_type.len() as u32;
    self.sec_type.push(profile);
    self.sec_import.push(ModuleImport{
      import,
      description: ImportExportDescription::Func(type_idx),
    });
  }

  pub fn export_function(&mut self, profile: FunctionType, export: Export) {
    let type_idx = self.sec_type.len() as u32;
    self.sec_type.push(profile);
    self.sec_export.push(ModuleExport{
      export,
      description: ImportExportDescription::Func(type_idx),
    });
  }

  pub fn add_function(&mut self, profile: FunctionType, body: Body) -> u32 {
    let type_idx = self.sec_type.len() as u32;
    self.sec_type.push(profile);
    let function_idx = self.sec_func.len() as u32;
    // FIXME: catch `as u32` overflow
    self.sec_func.push(type_idx);
    self.sec_code.push(body);
    function_idx
  }

  pub fn write(&self, filename: &Path) -> Result<()> {
    let mut file = File::create(filename)?;
    // magic
    file.write_all(&[0x00, 0x61, 0x73, 0x6d])?;
    // version
    file.write_all(&[0x01, 0x00, 0x00, 0x00])?;
    if !self.sec_type.is_empty() {
      let mut section_content = Vec::new();
      for type_ in self.sec_type.iter() {
        type_.compile(&mut section_content);
      }
      file.write_all(&[0x01])?; // section id
      let vec_len = from_u32(self.sec_type.len() as u32);
      file.write_all(&from_u32(
        (section_content.len() + vec_len.len()) as u32
      ))?;
      file.write_all(&vec_len)?;
      file.write_all(&section_content)?;
    }
    if !self.sec_import.is_empty() {
      let mut section_content = Vec::new();
      for import in self.sec_import.iter() {
        import.compile(&mut section_content);
      }
      file.write_all(&[0x02])?; // section id
      let vec_len = from_u32(self.sec_import.len() as u32);
      file.write_all(&from_u32(
        (section_content.len() + vec_len.len()) as u32
      ))?;
      file.write_all(&vec_len)?;
      file.write_all(&section_content)?;
    }
    if !self.sec_func.is_empty() {
      let mut section_content = Vec::new();
      for function_idx in self.sec_func.iter() {
        section_content.extend(&from_u32(*function_idx));
      }
      file.write_all(&[0x03])?; // section id
      let vec_len = from_u32(self.sec_func.len() as u32);
      file.write_all(&from_u32(
        (section_content.len() + vec_len.len()) as u32
      ))?;
      file.write_all(&vec_len)?;
      file.write_all(&section_content)?;
    }
    if !self.sec_export.is_empty() {
      let mut section_content = Vec::new();
      for export in self.sec_export.iter() {
        export.compile(&mut section_content);
      }
      file.write_all(&[0x07])?; // section id
      let vec_len = from_u32(self.sec_export.len() as u32);
      file.write_all(&from_u32(
        (section_content.len() + vec_len.len()) as u32
      ))?;
      file.write_all(&vec_len)?;
      file.write_all(&section_content)?;
    }
    if !self.sec_code.is_empty() {
      let mut section_content = Vec::new();
      for body in self.sec_code.iter() {
        body.compile(&mut section_content);
      }
      file.write_all(&[0x0a])?; // section id
      let vec_len = from_u32(self.sec_code.len() as u32);
      file.write_all(&from_u32(
        (section_content.len() + vec_len.len()) as u32
      ))?;
      file.write_all(&vec_len)?;
      file.write_all(&section_content)?;
    }
    Ok(())
  }
}

struct ModuleImport {
  pub import:      Import,
  pub description: ImportExportDescription,
}

impl Compilable for ModuleImport {
  fn compile(&self, buf: &mut Vec<u8>) {
    self.import.compile(buf);
    match self.description {
      ImportExportDescription::Func(type_idx) => {
        buf.push(0x00);
        buf.extend(&from_u32(type_idx));
      }
    }
  }
}

struct ModuleExport {
  pub export:      Export,
  pub description: ImportExportDescription,
}

impl Compilable for ModuleExport {
  fn compile(&self, buf: &mut Vec<u8>) {
    self.export.compile(buf);
    match self.description {
      ImportExportDescription::Func(type_idx) => {
        buf.push(0x00);
        buf.extend(&from_u32(type_idx));
      }
    }
  }
}
