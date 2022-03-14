/*
 * wasm-ir - Intermediate Representation of WebAssembly
 * Copyright © 2019-2022 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

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
