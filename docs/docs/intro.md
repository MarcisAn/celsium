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
## Let's look at this example

- Create a block with a dummy scope.
- Load an integer to the top of the stack twice.
- Call the addition operator that pops the last two items off the stack. Adds them and puts the result back on the stack.
- Call the print function that prints the item at the sop of the stack
- Create a programm with this block as the main block. The second argument is the defined functions. Let's leave that empty.
- Run the programm. 

## Blocks

Blocks are the main building block (*haha*) of the program. They are intended to directly represent syntactic blocks (if statement body, loop body, function body), but they are just used to hold program logic, so there are no restrictions. In the above example we created a main block that includes the whole file.

When blocks are defined, a `Scope` is assigned to them. This data is ment to be used for variable scoping, which is not handled by celsium, since that depends on the language frontend.