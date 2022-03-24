use crate::Compilable;
use crate::values::from_u32;

#[derive(Debug)]
pub struct Local {
  n:       u32,
  valtype: u8,
}

impl Local {
  pub fn new(n: u32, valtype: u8) -> Self {
    Self{ n, valtype }
  }
}

impl Compilable for Local {
  fn compile(&self, buf: &mut Vec<u8>) {
    buf.extend(&from_u32(self.n));
    buf.push(self.valtype);
  }
}

#[derive(Debug)]
pub struct LocalBuilder {
}

/*
impl LocalBuilder {
  fn new() -> Self {
    Self{}
  }

  fn add(&mut self, valtype: u8) -> u32 {
    todo!()
  }

  fn build(&self, type_: FunctionType) -> Vec<Local> {
    todo!()
  }
}
*/

/* TODO: use locals
fn test() {
  use crate::I32;
  let mut builder = LocalBuilder::new();
  let test1 = builder.add(I32);
  LocalGet::create(test);
}
*/

