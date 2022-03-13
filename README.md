# WebAssembly Intermediate Representation

![GPL-3.0](https://img.shields.io/github/license/theblueforest/wasm-ir)
![Maintained](https://img.shields.io/badge/maintained-yes-green.svg)
![Unstable](https://img.shields.io/badge/stable-no-red.svg)
![Crates.io latest version](https://img.shields.io/crates/v/wasm-ir)
![Crates.io total downloads](https://img.shields.io/crates/d/wasm-ir)

This is an Intermediate Representation which can be compiled into WebAssembly.

The source code is in an experimental state and will be fully documented in its final version.

Warning : **we do not recommend to use this code in a production environment**, even if it theoretically works we haven't finished writing tests to make sure everything works and avoid regressions.

The codebase is [licensed under GNU General Public License v3](./LICENSE), you have the right to exploit this source code but it must remain open-source and under the same license, thanks for your support !

If you want to join the development and contribute to the project, please reach us at contact@blueforest.cc.


## Todo

### Values

- [x] u32
- [ ] u64
- [ ] s32
- [ ] s64
- [ ] f32
- [ ] f64
- [ ] v128
- [ ] name

### Types

- [x] u32
- [ ] u64
- [ ] s32
- [ ] s64
- [x] f32
- [x] f64
- [x] v128
- [ ] name
- [x] reference
  - [x] function
  - [x] extern
- [x] limit
- [x] function

### Instructions

#### Control

- [ ] unreachable
- [ ] nop
- [ ] block
- [ ] if
- [ ] else
- [ ] br
- [ ] br\_if
- [ ] br\_table
- [ ] return
- [x] call
- [x] call\_indirect

#### Reference

- [x] ref.null
- [x] ref.is\_null
- [x] ref.func

#### Parametric

- [x] drop
- [ ] select
- [ ] select t

#### Variable

- [x] local.get
- [x] local.set
- [ ] local.tee
- [ ] global.get
- [ ] global.set

#### Table

- [ ] table.get
- [ ] table.set
- [x] table.init
- [ ] elem.drop
- [ ] table.copy
- [ ] table.grow
- [ ] table.size
- [ ] table.fill

#### Memory

- [ ] i32.load
- [ ] i64.load
- [ ] f32.load
- [ ] f64.load
- [ ] i32.load8\_s
- [ ] i32.load8\_u
- [ ] i32.load16\_s
- [ ] i32.load16\_u
- [ ] i64.load8\_s
- [ ] i64.load8\_u
- [ ] i64.load16\_s
- [ ] i64.load16\_u
- [ ] i64.load32\_s
- [ ] i64.load32\_u
- [x] i32.store
- [ ] i64.store
- [ ] f32.store
- [ ] f64.store
- [ ] i32.store8
- [ ] i32.store16
- [ ] i64.store8
- [ ] i64.store16
- [ ] i64.store32
- [ ] memory.size
- [ ] memory.grow
- [ ] memory.init
- [ ] data.drop
- [ ] memory.copy
- [ ] memory.fill

#### Numeric

- [x] i32.const
- [ ] i64.const
- [ ] f32.const
- [ ] f64.const
- [ ] i32.eqz
- [ ] i32.eq
- [ ] i32.ne
- [ ] i32.lt\_s
- [ ] i32.lt\_u
- [ ] i32.gt\_s
- [ ] i32.gt\_u
- [ ] i32.le\_s
- [ ] i32.le\_u
- [ ] i32.ge\_s
- [ ] i32.ge\_u
- [ ] i64.eqz
- [ ] i64.eq
- [ ] i64.ne
- [ ] i64.lt\_s
- [ ] i64.lt\_u
- [ ] i64.gt\_s
- [ ] i64.gt\_u
- [ ] i64.le\_s
- [ ] i64.le\_u
- [ ] i64.ge\_s
- [ ] i64.ge\_u
- [ ] f32.eq
- [ ] f32.ne
- [ ] f32.lt
- [ ] f32.gt
- [ ] f32.le
- [ ] f32.ge
- [ ] f64.eq
- [ ] f64.ne
- [ ] f64.lt
- [ ] f64.gt
- [ ] f64.le
- [ ] f64.ge
- [ ] i32.clz
- [ ] i32.ctz
- [ ] i32.popcnt
- [ ] i32.add
- [ ] i32.sub
- [ ] i32.mul
- [ ] i32.div\_s
- [ ] i32.div\_u
- [ ] i32.rem\_s
- [ ] i32.rem\_u
- [ ] i32.and
- [ ] i32.or
- [ ] i32.xor
- [ ] i32.shl
- [ ] i32.shr\_s
- [ ] i32.shr\_u
- [ ] i32.rotl
- [ ] i32.rotr
- [ ] i64.clz
- [ ] i64.ctz
- [ ] i64.popcnt
- [ ] i64.add
- [ ] i64.sub
- [ ] i64.mul
- [ ] i64.div\_s
- [ ] i64.div\_u
- [ ] i64.rem\_s
- [ ] i64.rem\_u
- [ ] i64.and
- [ ] i64.or
- [ ] i64.xor
- [ ] i64.shl
- [ ] i64.shr\_s
- [ ] i64.shr\_u
- [ ] i64.rotl
- [ ] i64.rotr
- [ ] f32.abs
- [ ] f32.neg
- [ ] f32.ceil
- [ ] f32.floor
- [ ] f32.trunc
- [ ] f32.nearest
- [ ] f32.sqrt
- [ ] f32.add
- [ ] f32.sub
- [ ] f32.mul
- [ ] f32.div
- [ ] f32.min
- [ ] f32.max
- [ ] f32.copysign
- [ ] f64.abs
- [ ] f64.neg
- [ ] f64.ceil
- [ ] f64.floor
- [ ] f64.trunc
- [ ] f64.nearest
- [ ] f64.sqrt
- [ ] f64.add
- [ ] f64.sub
- [ ] f64.mul
- [ ] f64.div
- [ ] f64.min
- [ ] f64.max
- [ ] f64.copysign
- [ ] i32.wrap\_i64
- [ ] i32.trunc\_f32\_s
- [ ] i32.trunc\_f32\_u
- [ ] i32.trunc\_f64\_s
- [ ] i32.trunc\_f64\_u
- [ ] i64.extend\_i32\_s
- [ ] i64.extend\_i32\_u
- [ ] i64.trunc\_f32\_s
- [ ] i64.trunc\_f32\_u
- [ ] i64.trunc\_f64\_s
- [ ] i64.trunc\_f64\_u
- [ ] f32.convert\_i32\_s
- [ ] f32.convert\_i32\_u
- [ ] f32.convert\_i64\_s
- [ ] f32.convert\_i64\_u
- [ ] f32.demote\_f64
- [ ] f64.convert\_i32\_s
- [ ] f64.convert\_i32\_u
- [ ] f64.convert\_i64\_s
- [ ] f64.convert\_i64\_u
- [ ] f64.promote\_f32
- [ ] i32.reinterpret\_f32
- [ ] i64.reinterpret\_f64
- [ ] f32.reinterpret\_i32
- [ ] f64.reinterpret\_i64
- [ ] i32.extend8\_s
- [ ] i32.extend16\_s
- [ ] i64.extend8\_s
- [ ] i64.extend16\_s
- [ ] i64.extend32\_s
- [ ] i32.trunc\_sat\_f32\_s
- [ ] i32.trunc\_sat\_f32\_u
- [ ] i32.trunc\_sat\_f64\_s
- [ ] i32.trunc\_sat\_f64\_u
- [ ] i64.trunc\_sat\_f32\_s
- [ ] i64.trunc\_sat\_f32\_u
- [ ] i64.trunc\_sat\_f64\_s
- [ ] i64.trunc\_sat\_f64\_u
- [ ] v128.load
- [ ] v128.load8x8\_s
- [ ] v128.load8x8\_u
- [ ] v128.load16x4\_s
- [ ] v128.load16x4\_u
- [ ] v128.load32x2\_s
- [ ] v128.load32x2\_u
- [ ] v128.load8\_splat
- [ ] v128.load16\_splat
- [ ] v128.load32\_splat
- [ ] v128.load64\_splat
- [ ] v128.load32\_zero
- [ ] v128.load64\_zero
- [ ] v128.store
- [ ] v128.load8\_lane
- [ ] v128.load16\_lane
- [ ] v128.load32\_lane
- [ ] v128.load64\_lane
- [ ] v128.store8\_lane
- [ ] v128.store16\_lane
- [ ] v128.store32\_lane
- [ ] v128.store64\_lane
- [ ] v128.const
- [ ] i8x16.shuffle
- [ ] i8x16.extract\_lane\_s
- [ ] i8x16.extract\_lane\_u
- [ ] i8x16.replace\_lane
- [ ] i16x8.extract\_lane\_s
- [ ] i16x8.extract\_lane\_u
- [ ] i16x8.replace\_lane
- [ ] i32x4.extract\_lane
- [ ] i32x4.replace\_lane
- [ ] i64x2.extract\_lane
- [ ] i64x2.replace\_lane
- [ ] f32x4.extract\_lane
- [ ] f32x4.replace\_lane
- [ ] f64x2.extract\_lane
- [ ] f64x2.replace\_lane
- [ ] i8x16.swizzle
- [ ] i8x16.splat
- [ ] i16x8.splat
- [ ] i32x4.splat
- [ ] i64x2.splat
- [ ] f32x4.splat
- [ ] f64x2.splat
- [ ] i8x16.eq
- [ ] i8x16.ne
- [ ] i8x16.lt\_s
- [ ] i8x16.lt\_u
- [ ] i8x16.gt\_s
- [ ] i8x16.gt\_u
- [ ] i8x16.le\_s
- [ ] i8x16.le\_u
- [ ] i8x16.ge\_s
- [ ] i8x16.ge\_u
- [ ] i16x8.eq
- [ ] i16x8.ne
- [ ] i16x8.lt\_s
- [ ] i16x8.lt\_u
- [ ] i16x8.gt\_s
- [ ] i16x8.gt\_u
- [ ] i16x8.le\_s
- [ ] i16x8.le\_u
- [ ] i16x8.ge\_s
- [ ] i16x8.ge\_u
- [ ] i32x4.eq
- [ ] i32x4.ne
- [ ] i32x4.lt\_s
- [ ] i32x4.lt\_u
- [ ] i32x4.gt\_s
- [ ] i32x4.gt\_u
- [ ] i32x4.le\_s
- [ ] i32x4.le\_u
- [ ] i32x4.ge\_s
- [ ] i32x4.ge\_u
- [ ] i64x2.eq
- [ ] i64x2.ne
- [ ] i64x2.lt\_s
- [ ] i64x2.gt\_s
- [ ] i64x2.le\_s
- [ ] i64x2.ge\_s
- [ ] f32x4.eq
- [ ] f32x4.ne
- [ ] f32x4.lt
- [ ] f32x4.gt
- [ ] f32x4.le
- [ ] f32x4.ge
- [ ] f64x2.eq
- [ ] f64x2.ne
- [ ] f64x2.lt
- [ ] f64x2.gt
- [ ] f64x2.le
- [ ] f64x2.ge
- [ ] v128.not
- [ ] v128.and
- [ ] v128.andnot
- [ ] v128.or
- [ ] v128.xor
- [ ] v128.bitselect
- [ ] v128.any\_true
- [ ] i8x16.abs
- [ ] i8x16.neg
- [ ] i8x16.popcnt
- [ ] i8x16.all\_true
- [ ] i8x16.bitmask
- [ ] i8x16.narrow\_i16x8\_s
- [ ] i8x16.narrow\_i16x8\_u
- [ ] i8x16.shl
- [ ] i8x16.shr\_s
- [ ] i8x16.shr\_u
- [ ] i8x16.add
- [ ] i8x16.add\_sat\_s
- [ ] i8x16.add\_sat\_u
- [ ] i8x16.sub
- [ ] i8x16.sub\_sat\_s
- [ ] i8x16.sub\_sat\_u
- [ ] i8x16.min\_s
- [ ] i8x16.min\_u
- [ ] i8x16.max\_s
- [ ] i8x16.max\_u
- [ ] i8x16.avgr\_u
- [ ] i16x8.extadd\_pairwise\_i8x16\_s
- [ ] i16x8.extadd\_pairwise\_i8x16\_u
- [ ] i16x8.abs
- [ ] i16x8.neg
- [ ] i16x8.q15mulr\_sat\_s
- [ ] i16x8.all\_true
- [ ] i16x8.bitmask
- [ ] i16x8.narrow\_i32x4\_s
- [ ] i16x8.narrow\_i32x4\_u
- [ ] i16x8.extend\_low\_i8x16\_s
- [ ] i16x8.extend\_high\_i8x16\_s
- [ ] i16x8.extend\_low\_i8x16\_u
- [ ] i16x8.extend\_high\_i8x16\_u
- [ ] i16x8.shl
- [ ] i16x8.shr\_s
- [ ] i16x8.shr\_u
- [ ] i16x8.add
- [ ] i16x8.add\_sat\_s
- [ ] i16x8.add\_sat\_u
- [ ] i16x8.sub
- [ ] i16x8.sub\_sat\_s
- [ ] i16x8.sub\_sat\_u
- [ ] i16x8.mul
- [ ] i16x8.min\_s
- [ ] i16x8.min\_u
- [ ] i16x8.max\_s
- [ ] i16x8.max\_u
- [ ] i16x8.avgr\_u
- [ ] i16x8.extmul\_low\_i8x16\_s
- [ ] i16x8.extmul\_high\_i8x16\_s
- [ ] i16x8.extmul\_low\_i8x16\_u
- [ ] i16x8.extmul\_high\_i8x16\_u
- [ ] i32x4.extadd\_pairwise\_i8x16\_s
- [ ] i32x4.extadd\_pairwise\_i8x16\_u
- [ ] i32x4.abs
- [ ] i32x4.neg
- [ ] i32x4.all\_true
- [ ] i32x4.bitmask
- [ ] i32x4.extend\_low\_i16x8\_s
- [ ] i32x4.extend\_high\_i16x8\_s
- [ ] i32x4.extend\_low\_i16x8\_u
- [ ] i32x4.extend\_high\_i16x8\_u
- [ ] i32x4.shl
- [ ] i32x4.shr\_s
- [ ] i32x4.shr\_u
- [ ] i32x4.add
- [ ] i32x4.sub
- [ ] i32x4.mul
- [ ] i32x4.min\_s
- [ ] i32x4.min\_u
- [ ] i32x4.max\_s
- [ ] i32x4.max\_u
- [ ] i32x4.dot\_i16x8\_s
- [ ] i32x4.extmul\_low\_i16x8\_s
- [ ] i32x4.extmul\_high\_i16x8\_s
- [ ] i32x4.extmul\_low\_i16x8\_u
- [ ] i32x4.extmul\_high\_i16x8\_u
- [ ] i64x2.abs
- [ ] i64x2.neg
- [ ] i64x2.all\_true
- [ ] i64x2.bitmask
- [ ] i64x2.extend\_low\_i32x4\_s
- [ ] i64x2.extend\_high\_i32x4\_s
- [ ] i64x2.extend\_low\_i32x4\_u
- [ ] i64x2.extend\_high\_i32x4\_u
- [ ] i64x2.shl
- [ ] i64x2.shr\_s
- [ ] i64x2.shr\_u
- [ ] i64x2.add
- [ ] i64x2.sub
- [ ] i64x2.mul
- [ ] i64x2.extmul\_low\_i32x4\_s
- [ ] i64x2.extmul\_high\_i32x4\_s
- [ ] i64x2.extmul\_low\_i32x4\_u
- [ ] i64x2.extmul\_high\_i32x4\_u
- [ ] f32x4.ceil
- [ ] f32x4.floor
- [ ] f32x4.trunc
- [ ] f32x4.nearest
- [ ] f32x4.abs
- [ ] f32x4.neg
- [ ] f32x4.sqrt
- [ ] f32x4.add
- [ ] f32x4.sub
- [ ] f32x4.mul
- [ ] f32x4.div
- [ ] f32x4.min
- [ ] f32x4.max
- [ ] f32x4.pmin
- [ ] f32x4.pmax
- [ ] f64x2.ceil
- [ ] f64x2.floor
- [ ] f64x2.trunc
- [ ] f64x2.nearest
- [ ] f64x2.abs
- [ ] f64x2.neg
- [ ] f64x2.sqrt
- [ ] f64x2.add
- [ ] f64x2.sub
- [ ] f64x2.mul
- [ ] f64x2.div
- [ ] f64x2.min
- [ ] f64x2.max
- [ ] f64x2.pmin
- [ ] f64x2.pmax
- [ ] i32x4.trunc\_sat\_f32x4\_s
- [ ] i32x4.trunc\_sat\_f32x4\_u
- [ ] f32x4.convert\_i32x4\_s
- [ ] f32x4.convert\_i32x4\_u
- [ ] i32x4.trunc\_sat\_f64x2\_s\_zero
- [ ] i32x4.trunc\_sat\_f64x2\_u\_zero
- [ ] f64x2.convert\_low\_i32x4\_s
- [ ] f64x2.convert\_low\_i32x4\_u
- [ ] f32x4.demote\_f64x2\_zero
- [ ] f64x2.promote\_low\_f32x4

### Module

- [x] Type
- [ ] Import
  - [x] Func
  - [x] Table
  - [x] Mem
  - [ ] Global
- [x] Function
- [x] Table
- [x] Memory
- [ ] Global
- [ ] Export
  - [x] Func
  - [x] Table
  - [x] Mem
  - [ ] Global
- [ ] Start
- [x] Element (except declarative)
- [ ] Data count
- [x] Code
- [x] Data
- [ ] Custom


## Contributors
- Nazim Lachter ([@n4zim](https://github.com/n4zim))
- Vulcain ([@vulc41n](https://github.com/vulc41n))
