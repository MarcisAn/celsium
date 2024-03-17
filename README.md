# A rust library for building interpreters

![Crates.io Version](https://img.shields.io/crates/v/celsium)

## Basic usage

```rust
use celsium::block::Block;
use celsium::module::Module;
use celsium::{CelsiumProgram, BINOP, BUILTIN_TYPES};

fn main() {
    let mut celsium = CelsiumProgram::new();
    let mut main_module = Module::new("main", &mut celsium);

    let mut main_block = Block::new();
    {
        main_block.load_const(BUILTIN_TYPES::MAGIC_INT, "1");
        main_block.load_const(BUILTIN_TYPES::MAGIC_INT, "2");
        main_block.binop(BINOP::ADD);
        main_block.call_print_function(true);
    }
    //println!("{:?}", main_block.bytecode);
    main_module.add_block(main_block);

    celsium.add_module(&main_module);

    celsium.run_program();
}

```

## If statements

```rust
let mut celsius = CelsiumProgram::new();
let mut main_module = Module::new("main", &mut celsius);
let mut main_block = Block::new();
{
    main_block.load_const(BUILTIN_TYPES::BOOL, "1");
    let mut if_block = Block::new();
    {
        if_block.load_const(BUILTIN_TYPES::STRING, "executed if block");
        if_block.call_print_function(true);
    }
    main_block.define_if_block(if_block);
}
```

## If Else statements

```rust
let mut celsius = CelsiumProgram::new();
let mut main_module = Module::new("main", &mut celsius);
let mut main_block = Block::new();
{
    main_block.load_const(BUILTIN_TYPES::BOOL, "1");
    let mut if_block = Block::new();
    {
        if_block.load_const(BUILTIN_TYPES::STRING, "executed if block");
        if_block.call_print_function(true);
    }
    let mut else_block = Block::new();
    {
        else_block.load_const(BUILTIN_TYPES::STRING, "executed else block");
        else_block.call_print_function(true);
    }
    main_block.define_if_else_block(if_block, else_block);
}
```
