---
sidebar_position: 1
---

# Getting started

```rust
extern crate celsium;
use celsium::{block::Block, CelsiumProgram, Scope, bytecode::BINOP };

fn main() {
    let mut main_block = Block::new(Scope{ast_id: 0, module_path: "".to_string()});
    main_block.load_int(2);
    main_block.load_int(2);
    main_block.binop(BINOP::Add);
    main_block.call_special_function(celsium::SpecialFunctions::Print { newline: true });

    let mut program = CelsiumProgram::new(main_block, vec![]);
    program.run_program();
}

```

## Blocks

Blocks are the main building block (*haha*) of the program. They are intended to directly represent syntactic blocks (if statement body, loop body, function body), but they are just used to hold program logic, so there are no restrictions. In the above example we created a main block that includes the whole file.

When blocks are defined, a `Scope` is assigned to them. This data is ment to be used for variable scoping, which is not handled by celsium.