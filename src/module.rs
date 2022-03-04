use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

use crate::{
  Body,
  Compilable,
  Data,
  Export,
  ExportDescription,
  Function,
  FunctionType,
  Import,
  ImportDescription,
  Limit,
};
use crate::values::from_u32;

pub struct Module {
  sec_type:   Vec<Box<dyn Compilable>>,
  sec_import: Vec<Box<dyn Compilable>>,
  sec_func:   Vec<Box<dyn Compilable>>,
  // sec_table:  Vec<Box<dyn Compilable>>,
  sec_mem:    Vec<Box<dyn Compilable>>,
  // sec_global: Vec<Box<dyn Compilable>>,
  sec_export: Vec<Box<dyn Compilable>>,
  // sec_start:  Vec<Box<dyn Compilable>>,
  // sec_elem:   Vec<Box<dyn Compilable>>,
  // data_count: Vec<Box<dyn Compilable>>,
  sec_code:   Vec<Box<dyn Compilable>>,
  sec_data:   Vec<Box<dyn Compilable>>,
  // sec_custom: Vec<Box<dyn Compilable>>,
}

impl Module {
  pub fn new() -> Self {
    Self{
      sec_type:   Vec::new(),
      sec_import: Vec::new(),
      sec_func:   Vec::new(),
      // sec_table:  Vec::new(),
      sec_mem:    Vec::new(),
      // sec_global: Vec::new(),
      sec_export: Vec::new(),
      // sec_start:  Vec::new(),
      // sec_elem:   Vec::new(),
      // data_count: Vec::new(),
      sec_code:   Vec::new(),
      sec_data:   Vec::new(),
      // sec_custom: Vec::new(),
    }
  }

  pub fn import_function(
    &mut self,
    profile: FunctionType,
    import:  Import,
  ) -> u32 {
    let type_idx = self.sec_type.len() as u32;
    self.sec_type.push(Box::new(profile));
    self.sec_import.push(Box::new(ModuleImport{
      import,
      description: ImportDescription::Func(type_idx),
    }));
    type_idx
  }

  pub fn add_function(
    &mut self,
    profile: FunctionType,
    body:    Body,
  ) -> u32 {
    let type_idx = self.sec_type.len() as u32;
    self.sec_type.push(Box::new(profile));
    self.sec_func.len() as u32;
    // FIXME: catch `as u32` overflow
    self.sec_func.push(Box::new(Function::new(type_idx)));
    self.sec_code.push(Box::new(body));
    type_idx
  }

  pub fn add_exported_function(
    &mut self,
    profile: FunctionType,
    body:    Body,
    name:    String,
  ) -> u32 {
    let type_idx = self.add_function(profile, body);
    self.sec_export.push(Box::new(ModuleExport{
      export:      Export::new(name),
      description: ExportDescription::Func(type_idx),
    }));
    type_idx
  }

  pub fn add_data(&mut self, data: Data) {
    self.sec_data.push(Box::new(data));
  }

  pub fn set_memory(&mut self, limit: Limit) {
    let mem_idx = self.sec_mem.len() as u32;
    self.sec_mem.push(Box::new(limit));
    self.sec_export.push(Box::new(ModuleExport{
      export:      Export::new("memory".to_string()),
      description: ExportDescription::Mem(mem_idx),
    }));
  }

  pub fn write(&self, filename: &Path) -> Result<()> {
    let mut file = File::create(filename)?;
    // magic
    file.write_all(&[0x00, 0x61, 0x73, 0x6d])?;
    // version
    file.write_all(&[0x01, 0x00, 0x00, 0x00])?;

    compile_section(&mut file, &self.sec_type,   0x01)?;
    compile_section(&mut file, &self.sec_import, 0x02)?;
    compile_section(&mut file, &self.sec_func,   0x03)?;
    compile_section(&mut file, &self.sec_mem,    0x05)?;
    compile_section(&mut file, &self.sec_export, 0x07)?;
    compile_section(&mut file, &self.sec_code,   0x0a)?;
    compile_section(&mut file, &self.sec_data,   0x0b)?;
    Ok(())
  }
}

struct ModuleImport {
  pub import:      Import,
  pub description: ImportDescription,
}

impl Compilable for ModuleImport {
  fn compile(&self, buf: &mut Vec<u8>) {
    self.import.compile(buf);
    match self.description {
      ImportDescription::Func(type_idx) => {
        buf.push(0x00);
        buf.extend(&from_u32(type_idx));
      }
    }
  }
}

struct ModuleExport {
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
      ExportDescription::Mem(mem_idx) => {
        buf.push(0x02);
        buf.extend(&from_u32(mem_idx));
      }
      _ => { todo!() }
    }
  }
}

fn compile_section(
  file:       &mut File,
  section:    &Vec<Box<dyn Compilable>>,
  section_id: u8,
) -> Result<()> {
  if !section.is_empty() {
    let mut section_content = Vec::new();
    for entry in section.iter() {
      entry.compile(&mut section_content);
    }
    file.write_all(&[section_id])?;
    let vec_len = from_u32(section.len() as u32);
    file.write_all(&from_u32(
      (section_content.len() + vec_len.len()) as u32
    ))?;
    file.write_all(&vec_len)?;
    file.write_all(&section_content)?;
  }
  Ok(())
}
