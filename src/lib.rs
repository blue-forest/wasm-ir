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

use std::fmt::Debug;

pub mod code;
pub use code::{Body, ConstInstruction, Instruction, LocalBuilder};

mod data;
pub use data::{Data, DataMode};

mod elements;
pub use elements::{Element, ElementMode};

mod exports;
pub use exports::{Export, ExportDescription};

mod functions;
pub use functions::StartFunction;

mod imports;
pub use imports::{Import, ImportDescription};

mod modules;
pub use modules::{Module, Section};

mod tables;
pub use tables::Table;

mod types;
pub use types::*;

pub mod values;

pub trait Compilable: Debug {
  fn compile(&self, buf: &mut Vec<u8>);
}

