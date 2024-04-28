# A rust library for building interpreters with WASM support

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
    main_module.add_main_block(main_block);

    celsium.add_module(&main_module);

    celsium.run_program();
}
```

## If statements

```rust
let mut celsium = CelsiumProgram::new();
let mut main_module = Module::new("main", &mut celsium);
let mut main_block = Block::new();
{
    main_block.load_const(BUILTIN_TYPES::BOOL, "1");
    let mut if_block = Block::new();
    {
        if_block.load_const(BUILTIN_TYPES::STRING, "executed if block");
        if_block.call_print_function(true);
    }
    main_block.define_if_block(if_block);
    main_module.add_main_block(main_block);

    celsium.add_module(&main_module);

    celsium.run_program();
}
```

## If Else statements

```rust
let mut celsium = CelsiumProgram::new();
let mut main_module = Module::new("main", &mut celsium);
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
    main_module.add_main_block(main_block);

    celsium.add_module(&main_module);

    celsium.run_program();
}
```

## Define functions

```rust
use celsium::block::Block;
use celsium::module::Module;
use celsium::{CelsiumProgram, BINOP, BUILTIN_TYPES};

fn main() {
    let mut celsium = CelsiumProgram::new();
    let mut main_module = Module::new("main", &mut celsium);

    let mut main_block = Block::new();

    let mut fn_block = Block::new();
        fn_block.load_const(BUILTIN_TYPES::STRING, "aaa");
        fn_block.call_print_function(true);

        main_module.define_function(
            fn_block,
            VISIBILITY::PRIVATE,
            FunctionSignature {
                name: "testfunction".to_owned(),
                return_type: FUNCTION_RETURN_TYPE::NONE,
                args: vec![],
            },
        );

    main_block.call_function("testfunction");
    //println!("{:?}", main_block.bytecode);
    main_module.add_main_block(main_block);

    celsium.add_module(&main_module);

    celsium.run_program();
}
```

## Variables

```rust
use celsium::block::Block;
use celsium::module::Module;
use celsium::{CelsiumProgram, BINOP, BUILTIN_TYPES};

fn main() {
    let mut celsium = CelsiumProgram::new(false);
    let mut main_module = Module::new("main", &mut celsium);
    let mut main_block = Block::new();
    {
        main_block.load_const(BUILTIN_TYPES::MAGIC_INT, "2");
        main_block.define_variable(BUILTIN_TYPES::MAGIC_INT, VISIBILITY::PRIVATE,   "test_var");
        //variable is defined
        main_block.load_variable("test_var");
        main_block.call_print_function(true);
        //variable is printed
    }
    main_module.add_main_block(main_block);
    celsium.add_module(&main_module);
    celsium.run_program();
}
```

## Simple loops

```rust
use celsium::block::Block;
use celsium::module::Module;
use celsium::{CelsiumProgram, BINOP, BUILTIN_TYPES};

fn main() {
        let mut celsium = CelsiumProgram::new(false);
        let mut main_module = Module::new("main", &mut celsium);
        let mut main_block = Block::new();

        let mut loop_block = Block::new();
        {
            loop_block.load_const(BUILTIN_TYPES::STRING, "I'm printing many times");
            loop_block.call_print_function(true);
        }
        let mut loop_count_block = Block::new();
        {
            loop_count_block.load_const(BUILTIN_TYPES::MAGIC_INT, "3");
            loop_count_block.load_const(BUILTIN_TYPES::MAGIC_INT, "3");
            loop_count_block.binop(BINOP::ADD);
        }

        main_block.define_simple_loop(loop_block, loop_count_block);
}
```

## While loops

```rust
use celsium::block::Block;
use celsium::module::Module;
use celsium::{CelsiumProgram, BINOP, BUILTIN_TYPES};

fn main() {
    let mut celsium = CelsiumProgram::new();
    let mut main_module = Module::new("main", &mut celsium);
    let mut main_block = Block::new();

    let mut conditional_block = Block::new();
    {
        conditional_block.load_const(BUILTIN_TYPES::MAGIC_INT, "1");
        conditional_block.load_const(BUILTIN_TYPES::MAGIC_INT, "1");
        conditional_block.binop(BINOP::EQ);
    }
    let mut loop_block = Block::new();
    {
        loop_block.load_const(BUILTIN_TYPES::MAGIC_INT, "20");
        loop_block.call_print_function(true);
    }
    main_block.define_while_loop(loop_block, conditional_block);


    main_module.add_main_block(main_block);
    celsium.add_module(&main_module);
    celsium.run_program();
}
```

## Objects

```rust
use celsium::block::Block;
use celsium::module::Module;
use celsium::{CelsiumProgram, BINOP, BUILTIN_TYPES};

fn main() {
    let mut celsium = CelsiumProgram::new();
    let mut main_module = Module::new("main", &mut celsium);
    let mut main_block = Block::new();

    main_block.load_const(BUILTIN_TYPES::STRING, "John");
    main_block.load_const(BUILTIN_TYPES::MAGIC_INT, "37");

    main_block.create_object("Person", vec!["name", "age"]);
    main_block.define_variable(BUILTIN_TYPES::OBJECT, VISIBILITY::PUBLIC, "person_1");
    main_block.load_variable("person_1");
    main_block.call_print_function(true);


    main_module.add_main_block(main_block);
    celsium.add_module(&main_module);
    celsium.run_program();
}
```
