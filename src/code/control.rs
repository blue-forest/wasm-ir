// This code is free software distributed under GPLv3 by Blue Forest.

use crate::Compilable;
use crate::values::from_u32;

use super::Instruction;

pub struct Call {
  function_idx: u32,
  parameters:   Vec<Box<dyn Instruction>>
}

impl Call {
  pub fn new(
    function_idx: u32,
    parameters:   Vec<Box<dyn Instruction>>
  ) -> Box<Self> {
    Box::new(Self{ function_idx, parameters })
  }

  pub fn new_stacked(function_idx: u32) -> Box<Self> {
    Box::new(Self{ function_idx, parameters: Vec::new() })
  }
}

impl Compilable for Call {
  fn compile(&self, buf: &mut Vec<u8>) {
    for parameter in self.parameters.iter() {
      parameter.compile(buf);
    }
    buf.push(0x10);
    buf.extend(&from_u32(self.function_idx));
  }
}

impl Instruction for Call {}

pub struct CallIndirect {
  type_idx:     u32,
  table_idx:    u32,
  parameters:   Vec<Box<dyn Instruction>>,
  function_idx: Option<Box<dyn Instruction>>,
}

impl CallIndirect {
  pub fn new(
    type_idx:     u32,
    table_idx:    u32,
    parameters:   Vec<Box<dyn Instruction>>,
    function_idx: Box<dyn Instruction>,
  ) -> Box<Self> {
    Box::new(Self{
      type_idx,
      table_idx,
      parameters,
      function_idx: Some(function_idx),
    })
  }

  pub fn new_stacked(
    type_idx:     u32,
    table_idx:    u32,
  ) -> Box<Self> {
    Box::new(Self{
      type_idx,
      table_idx,
      parameters: Vec::new(),
      function_idx: None,
    })
  }
}

impl Compilable for CallIndirect {
  fn compile(&self, buf: &mut Vec<u8>) {
    for parameter in self.parameters.iter() {
      parameter.compile(buf);
    }
    if let Some(function_idx) = &self.function_idx {
      function_idx.compile(buf);
    }
    buf.push(0x11);
    buf.extend(&from_u32(self.type_idx));
    buf.extend(&from_u32(self.table_idx));
  }
}

impl Instruction for CallIndirect {}
