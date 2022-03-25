use std::sync::Arc;

use crate::{
  I32,
  I64,
  EXTERN_REF,
  F32,
  F64,
  FUNC_REF,
  FunctionType,
  V128,
};
use crate::values::from_u32;

#[derive(Debug)]
pub struct Local {
  idx:     u32,
  valtype: u8, // 0 means param
}

impl Local {
  fn new(idx: u32, valtype: u8) -> Self {
    Self{ idx, valtype }
  }

  pub fn with_param(idx: u32) -> Arc<Self> {
    Arc::new(Self{ idx, valtype: 0 })
  }
}

#[derive(Debug)]
pub struct LocalBuilder {
  i32_:       u32,
  i64_:       u32,
  f32_:       u32,
  f64_:       u32,
  v128:       u32,
  func_ref:   u32,
  extern_ref: u32,
}

impl LocalBuilder {
  pub fn new() -> Self {
    Self{
      i32_:       0,
      i64_:       0,
      f32_:       0,
      f64_:       0,
      v128:       0,
      func_ref:   0,
      extern_ref: 0,
    }
  }

  pub fn add(&mut self, valtype: u8) -> Arc<Local> {
    let idx = match valtype {
      I32        => {
        self.i32_ += 1;
        self.i32_ - 1
      }
      I64        => {
        self.i64_ += 1;
        self.i64_ - 1
      }
      F32        => {
        self.f32_ += 1;
        self.f32_ - 1
      }
      F64        => {
        self.f64_ += 1;
        self.f64_ - 1
      }
      V128       => {
        self.v128 += 1;
        self.v128 - 1
      }
      FUNC_REF   => {
        self.func_ref += 1;
        self.func_ref - 1
      }
      EXTERN_REF => {
        self.extern_ref += 1;
        self.extern_ref - 1
      }
      _ => { panic!("unkown valtype: {}", valtype) }
    };
    Arc::new(Local::new(idx, valtype))
  }

  fn get_idx_start(&self, valtype: u8) -> u32 {
    let mut result = 0;
    if valtype == I32 {
      return result;
    }
    result += self.i32_;
    if valtype == I64 {
      return result;
    }
    result += self.i64_;
    if valtype == F32 {
      return result;
    }
    result += self.f32_;
    if valtype == F64 {
      return result;
    }
    result += self.f64_;
    if valtype == V128 {
      return result;
    }
    result += self.v128;
    if valtype == FUNC_REF {
      return result;
    }
    result += self.func_ref;
    if valtype == EXTERN_REF {
      return result;
    }
    result += self.extern_ref;
    result
  }

  pub fn compile<'a>(
    &'a self,
    buf: &mut Vec<u8>,
    type_: &'a FunctionType,
  ) -> Locals<'a> {
    let mut n_locals = 0;
    let mut content = Vec::new();
    if self.i32_ > 0 {
      n_locals +=1;
      content.extend(&from_u32(self.i32_));
      content.push(I32);
    }
    if self.i64_ > 0 {
      n_locals +=1;
      content.extend(&from_u32(self.i64_));
      content.push(I64);
    }
    if self.f32_ > 0 {
      n_locals +=1;
      content.extend(&from_u32(self.f32_));
      content.push(F32);
    }
    if self.f64_ > 0 {
      n_locals +=1;
      content.extend(&from_u32(self.f64_));
      content.push(F64);
    }
    if self.v128 > 0 {
      n_locals +=1;
      content.extend(&from_u32(self.v128));
      content.push(V128);
    }
    if self.func_ref > 0 {
      n_locals +=1;
      content.extend(&from_u32(self.func_ref));
      content.push(FUNC_REF);
    }
    if self.extern_ref > 0 {
      n_locals +=1;
      content.extend(&from_u32(self.extern_ref));
      content.push(EXTERN_REF);
    }
    buf.extend(&from_u32(n_locals));
    buf.extend(&content);
    Locals::new(self, type_)
  }
}

pub struct Locals<'a> {
  builder: &'a LocalBuilder,
  type_:   &'a FunctionType,
}

impl<'a> Locals<'a> {
  fn new(builder: &'a LocalBuilder, type_: &'a FunctionType) -> Self {
    Self{ builder, type_ }
  }

  pub fn get_idx(&self, local: &Local) -> u32 {
    if local.valtype == 0 {
      local.idx
    } else {
      local.idx
        + self.type_.n_params()
        + self.builder.get_idx_start(local.valtype)
    }
  }
}

