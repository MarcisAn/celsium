---
sidebar_position: 1
---

# Getting started

```rust
let mut celsium = CelsiumProgram::new();
let mut main_module = Module::new("main", &mut celsium);

let main_block = Block::new(Scope { ast_id: 0, module_path: "" });

main_block.load_const(celsium::BuiltinTypes::Int, "2");
main_block.load_const(celsium::BuiltinTypes::Int, "2");

main_block.binop(celsium::bytecode::BINOP::Add);

main_block.call_special_function(celsium::SpecialFunctions::Print {
    newline: true,
});
```

## Blocks

Blocks are the main building block (*haha*) of the program. They are intended to directly represent syntactic blocks (if statement body, loop body, function body), but they are just used to hold program logic. In the above example we created a main block that includes the whole file.

When blocks are defined, a `Scope` is assigned to them. This data is not used internaly by celsium, instead is ment to be used for variable scoping, which is not handled by celsium.