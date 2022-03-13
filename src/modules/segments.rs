use crate::{
  Body,
  Data,
  Element,
  ElementMode,
  FUNC_REF,
  FunctionType,
};
use crate::code::reference::RefInstruction;
use super::Module;

impl Module {
  pub fn add_expression_element(
    &mut self,
    expr:         Vec<Box<dyn RefInstruction>>,
    element_mode: ElementMode,
  ) -> u32 {
    let element_idx = self.sec_elem.len() as u32;
    self.sec_elem.push(Box::new(Element::with_expr(
      FUNC_REF, expr, element_mode,
    )));
    element_idx
  }

  pub fn add_function_element(
    &mut self,
    profile:      FunctionType,
    body:         Body,
    element_mode: ElementMode,
  ) -> (u32, u32, u32) {
    let (type_idx, function_idx) = self.add_function(profile, body);
    let element_idx = self.sec_elem.len() as u32;
    self.sec_elem.push(Box::new(Element::with_func(
      vec![function_idx],
      element_mode,
    )));
    (type_idx, function_idx, element_idx)
  }

  pub fn add_data(&mut self, data: Data) {
    self.sec_data.push(Box::new(data));
  }
}
