use crate::Compilable;
use crate::values::from_u32;
use super::Instruction;

pub trait RefInstruction: Instruction {}

pub struct RefFunc {
  function_idx: u32,
}

impl RefFunc {
  pub fn new(function_idx: u32) -> Box<dyn RefInstruction> {
    Box::new(Self{ function_idx })
  }
}

impl Compilable for RefFunc {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0xd2);
    buf.extend(&from_u32(self.function_idx));
  }
}

impl Instruction for RefFunc {}

impl RefInstruction for RefFunc {}

pub struct RefIsNull{}

impl RefIsNull {
  pub fn new() -> Box<dyn Instruction> {
    Box::new(Self{})
  }
}

impl Compilable for RefIsNull {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0xd1);
  }
}

impl Instruction for RefIsNull {}

pub struct RefNull {
  type_: u8,
}

impl RefNull {
  pub fn new(type_: u8) -> Box<dyn RefInstruction> {
    Box::new(Self{ type_ })
  }
}

impl Compilable for RefNull {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.push(0xd0);
    buf.push(self.type_);
  }
}

impl Instruction for RefNull {}

impl RefInstruction for RefNull {}

