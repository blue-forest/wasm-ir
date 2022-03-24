use crate::{
  Body,
  Compilable,
  Element,
  ElementMode,
  FUNC_REF,
  FunctionType,
};
use crate::code::reference::RefInstruction;
use super::{Module, Section};

impl Module {
  pub fn add_expression_element(
    &mut self,
    expr:         Vec<Box<dyn RefInstruction>>,
    element_mode: ElementMode,
  ) -> u32 {
    let element_idx = self.sec_elem.len() as u32;
    self.sec_elem.push(Element::with_expr(
      FUNC_REF, expr, element_mode,
    ));
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
    self.sec_elem.push(Element::with_func(
      vec![function_idx],
      element_mode,
    ));
    (type_idx, function_idx, element_idx)
  }
}

#[derive(Debug)]
pub struct ElementSection {
  elements: Vec<Element>,
}

impl ElementSection {
  pub fn new() -> Self {
    Self{ elements: Vec::new() }
  }

  pub fn push(&mut self, element: Element) {
    self.elements.push(element);
  }
}

impl Default for ElementSection {
  fn default() -> Self { Self::new() }
}

impl Section for ElementSection {
  fn section_id(&self) -> u8 { 0x09 }

  fn len(&self) -> u32 { self.elements.len() as u32 }

  fn content(&self, _module: &Module) -> Vec<u8> {
    let mut result = Vec::new();
    for element in self.elements.iter() {
      element.compile(&mut result);
    }
    result
  }
}
