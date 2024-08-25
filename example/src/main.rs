extern crate celsium;
use celsium::{block::Block, bytecode::BINOP, typestack::TypeStack, CelsiumProgram, Scope };

fn main() {
    let mut typestack = TypeStack::new();
    let mut main_block = Block::new(Scope{ast_id: 0, module_path: "".to_string()});
    main_block.load_int(2);
    typestack.push(celsium::BuiltinTypes::Int);
    main_block.load_int(2);
    typestack.push(celsium::BuiltinTypes::Int);

    main_block.binop(BINOP::Add);
    main_block.call_special_function(celsium::SpecialFunctions::Print { newline: true });

    let mut program = CelsiumProgram::new(main_block, vec![]);
    program.run_program();
}
