# A rust library for building interpreters

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
