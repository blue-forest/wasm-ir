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

pub const I32        : u8 = 0x7f;
pub const I64        : u8 = 0x7e;
pub const F32        : u8 = 0x7d;
pub const F64        : u8 = 0x7c;
pub const V128       : u8 = 0x7b;
pub const FUNC_REF   : u8 = 0x70;
pub const EXTERN_REF : u8 = 0x6f;

