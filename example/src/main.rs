extern crate celsium;
use celsium::{
    block::Block,
    stores::variable_store::VariableStore,
    typestack::TypeStack,
    CelsiumProgram,
    Scope,
};
use celsium::ObjectFieldType;
fn main() {
    let mut typestack = TypeStack::new();
    let mut varstore = VariableStore::new();
    let mut main_block = Block::new(Scope { ast_id: 0, module_path: "".to_string() });
    main_block.load_int(2);
    typestack.push(celsium::BuiltinTypes::Int);
    main_block.load_string("aaaa");
    typestack.push(celsium::BuiltinTypes::String);

    let field_types = typestack.pop_multiple(2).unwrap();
    let field_names = vec!["somenumber", "somestring"];

    let mut fields: Vec<ObjectFieldType> = vec![];
    for (index, name) in field_names.into_iter().enumerate().rev() {
        fields.push(ObjectFieldType {
            name: name.to_string(),
            data_type: field_types[index].clone(),
        });
    }

    let ids = varstore.define_object(
        "objectname".to_string(),
        Scope { ast_id: 0, module_path: "".to_string() },
        fields
    );

    for id in ids {
        main_block.define_variable(id);
    }

    let varid = varstore
        .get_object_field(
            "objectname".to_string(),
            Scope { ast_id: 0, module_path: "".to_string() },
            "somenumber".to_string()
        )
        .unwrap();
    main_block.load_variable(varid);

    main_block.call_special_function(celsium::SpecialFunctions::Print { newline: true });
    let mut program = CelsiumProgram::new(main_block, vec![]);
    program.run_program();
}
