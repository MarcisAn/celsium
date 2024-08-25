extern crate celsium;
use celsium::{block::Block, stores::variable_store::VariableStore, typestack::TypeStack, CelsiumProgram, Scope };

fn main() {
    let mut typestack = TypeStack::new();
    let mut varstore = VariableStore::new();
    let mut main_block = Block::new(Scope{ast_id: 0, module_path: "".to_string()});
    main_block.load_int(2);
    typestack.push(celsium::BuiltinTypes::Int);
    let varid = varstore.define_variable("testvar".to_string(), Scope { ast_id: 0, module_path: "".to_string() }, celsium::BuiltinTypes::Int);

    main_block.define_variable(varid);
    let resolved_varid = varstore.find_variable("testvar".to_string(), Scope { ast_id: 0, module_path: "".to_string() });
    main_block.load_variable(resolved_varid.unwrap().id);
    main_block.call_special_function(celsium::SpecialFunctions::Print { newline: true });
    let mut program = CelsiumProgram::new(main_block, vec![]);
    program.run_program();

}
