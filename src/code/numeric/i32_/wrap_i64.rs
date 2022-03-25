use crate::Instruction;
use crate::code::Locals;

#[derive(Debug)]
pub struct I32WrapI64 {
  operand: Option<Box<dyn Instruction>>,
}

impl I32WrapI64 {
  pub fn with_stack() -> Box<dyn Instruction> {
    Box::new(Self{ operand: None })
  }

  pub fn with_operands(operand: Box<dyn Instruction>) -> Box<dyn Instruction> {
    Box::new(Self{ operand: Some(operand) })
  }
}

impl Instruction for I32WrapI64 {
  fn compile<'a>(&self, buf: &mut Vec<u8>, locals: &Locals<'a>) {
    if let Some(operand) = &self.operand {
      operand.compile(buf, locals);
    }
    buf.push(0xa7);
  }
}

