---
sidebar_position: 2
---

# Variables

At the lowest level, variables are stored globaly with a nummeric id.

```rust
extern crate celsium;
use celsium::{block::Block, CelsiumProgram, Scope, bytecode::BINOP };

fn main() {
    let mut main_block = Block::new(Scope{ast_id: 0, module_path: "".to_string()});
    main_block.load_int(2);
    
    main_block.define_variable(0);
    //the variable is now defined
    main_block.load_variable(0);
    //the value is now put on the top of the stack
    main_block.call_special_function(celsium::SpecialFunctions::Print { newline: true });

    let mut program = CelsiumProgram::new(main_block, vec![]);
    program.run_program();
}
```
## Variable store

In most cases you might want to store more intricate data about variables. In that case there is a ready-made solution.

```rust
extern crate celsium;
use celsium::{block::Block, stores::variable_store::VariableStore, typestack::TypeStack, CelsiumProgram, Scope };

fn main() {
    let mut varstore = VariableStore::new();
    let mut main_block = Block::new(Scope{ast_id: 0, module_path: "".to_string()});
    main_block.load_int(2);
    let varid = varstore.define_variable("testvar".to_string(), Scope { ast_id: 0, module_path: "".to_string() }, celsium::BuiltinTypes::Int);

    main_block.define_variable(varid);
    //the variable is now defined
    let found_id = varstore.find_variable("testvar".to_string(), Scope { ast_id: 0, module_path: "".to_string() });
    main_block.load_variable(found.unwrap().id);
    main_block.call_special_function(celsium::SpecialFunctions::Print { newline: true });

    let mut program = CelsiumProgram::new(main_block, vec![]);
    program.run_program();

}
```

When you define a variable in the variable store, it gives you a id to define the variable for real.

## Reassign variable

```rust
extern crate celsium;
use celsium::{block::Block, stores::variable_store::VariableStore, typestack::TypeStack, CelsiumProgram, Scope };

fn main() {
    let mut varstore = VariableStore::new();
    let mut main_block = Block::new(Scope{ast_id: 0, module_path: "".to_string()});
    main_block.load_int(2);

    main_block.define_variable(0);
    //the variable is now defined
    main_block.load_int(5);
    //the new value is now at the top of the stack
    main_block.assign_var(0);
    //the new value is now assigned
    main_block.call_special_function(celsium::SpecialFunctions::Print { newline: true });

    let mut program = CelsiumProgram::new(main_block, vec![]);
    program.run_program();

}
```