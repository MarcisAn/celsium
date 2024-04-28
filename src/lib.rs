use std::collections::HashMap;
use std::future::IntoFuture;
pub mod bytecode;
use block::Block;
use bytecode::{BINOP, OPTCODE};
use js_sys::Object;
use module::Function;
use module::FunctionReturnType;
use module::FunctionSignature;
use module::Module;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

extern crate js_sys;
pub mod block;
pub mod module;
mod vm;
use module::VISIBILITY;
use vm::vm::VM;
use vm::ObjectField;
use vm::StackValue;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn wasm_print(s: &str);
    async fn wasm_input() -> JsValue;
}

#[derive(Clone, Debug)]
pub enum BUILTIN_TYPES {
    MAGIC_INT,
    BOOL,
    STRING,
    OBJECT,
}

pub struct ObjectBuilder {
    name: String,
    fields: Vec<ObjectField>,
}

impl ObjectBuilder {
    pub fn new(object_name: String) -> ObjectBuilder {
        ObjectBuilder {
            name: object_name,
            fields: vec![],
        }
    }
}

pub struct CelsiumProgram {
    modules: Vec<Module>,
}

impl CelsiumProgram {
    pub fn new() -> CelsiumProgram {
        CelsiumProgram { modules: vec![] }
    }

    pub fn add_module(&mut self, module: &Module) {
        self.modules.push(module.clone());
    }

    pub fn run_program(&mut self) {
        let mut bytecode: Vec<OPTCODE> = vec![];
        for module in &self.modules {
            bytecode.append(&mut module.main_block.bytecode.clone());
        }
        let mut vm = VM::new();

        self.run(&mut vm, &bytecode);
    }

    pub fn run(&mut self, vm: &mut VM, bytecode: &Vec<OPTCODE>) -> Result<(), String> {
        let mut index: isize = 0;
        while index < bytecode.len() as isize {
            let optcode = &bytecode[index as usize];
            //println!("running optcode {:?}", optcode);
            match optcode {
                OPTCODE::LOAD_CONST { data_type, data } => vm.push(&data_type, &data),
                OPTCODE::CALL_FUNCTION { name } => {
                    vm.call_function(name, self);
                }
                OPTCODE::RETURN_FROM_FUNCTION => {
                    break;
                }
                OPTCODE::CALL_FUNCTION_WITH_BYTECODE { bytecode } => {
                    panic!();
                }
                OPTCODE::ADD => vm.aritmethics("+"),
                OPTCODE::SUBTRACT => vm.aritmethics("-"),
                OPTCODE::MULTIPLY => vm.aritmethics("*"),
                OPTCODE::DIVIDE => vm.aritmethics("/"),
                OPTCODE::REMAINDER => vm.aritmethics("%"),
                OPTCODE::CALL_PRINT_FUNCTION { newline } => {
                    let printable = &vm.format_for_print(*newline);
                    #[cfg(target_family = "wasm")]
                    wasm_print(printable);
                    print!("{}", printable);
                }
                OPTCODE::JUMP_IF_FALSE { steps } => {
                    if (vm.must_jump()) {
                        index += *steps as isize
                    }
                }
                OPTCODE::JUMP { steps } => index += *steps as isize,
                OPTCODE::JUMP_BACK { steps } => index -= *steps as isize,
                OPTCODE::LESS_THAN => vm.aritmethics("<"),
                OPTCODE::LARGER_THAN => vm.aritmethics(">"),
                OPTCODE::LESS_OR_EQ => vm.aritmethics("<="),
                OPTCODE::LARGER_OR_EQ => vm.aritmethics(">="),
                OPTCODE::NOT_EQ => vm.aritmethics("!="),
                OPTCODE::EQ => vm.aritmethics("=="),
                OPTCODE::OR => vm.aritmethics("or"),
                OPTCODE::AND => vm.aritmethics("and"),
                OPTCODE::XOR => vm.aritmethics("xor"),
                OPTCODE::DEFINE_VAR {
                    data_type,
                    visibility,
                    name,
                } => vm.define_var(0, name.to_string(), visibility),
                OPTCODE::LOAD_VAR { name } => vm.load_var(name)?,
                OPTCODE::ASSIGN_VAR { name } => vm.assign_var(name),
                OPTCODE::DEFINE_ARRAY {
                    visibility,
                    name,
                    init_values_count,
                } => vm.define_array(0, name.to_string(), visibility, *init_values_count),
                OPTCODE::GET_FROM_ARRAY { name } => vm.get_from_array(name),
                OPTCODE::PUSH_TO_ARRAY { name } => vm.push_to_array(name),
                OPTCODE::GET_ARRAY_LENGTH { name } => vm.get_array_length(name),
                OPTCODE::DEFINE_FUNCTION {
                    body_block,
                    visibility,
                    signature,
                } => self.modules[0].functions.push(Function {
                    signature: signature.clone(),
                    body: body_block.clone(),
                    visibility: visibility.clone(),
                }),
                OPTCODE::CALL_INPUT => {
                    #[cfg(target_family = "wasm")]
                    vm.push(&BUILTIN_TYPES::STRING, &"asdfghjkl".to_string());
                    #[cfg(target_family = "wasm")]
                    async {
                        let value = &wasm_input().await.as_string().unwrap();
                        vm.push(&BUILTIN_TYPES::STRING, value);
                    };
                    #[cfg(not(target_family = "wasm"))]
                    vm.input("");
                }
                OPTCODE::CREATE_OBJECT { name, field_names } => {
                    let mut fields: Vec<ObjectField> = vec![];
                    let mut field_names_mut = field_names.clone();
                    field_names_mut.reverse();
                    for name in field_names_mut {
                        fields.push(ObjectField {
                            name: name.to_string(),
                            value: vm.pop(),
                        })
                    }
                    fields.reverse();
                    vm.push_stackvalue(StackValue::OBJECT {
                        name: name.to_string(),
                        value: fields,
                    });
                }
            }
            index += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use self::module::{FunctionSignature, VISIBILITY};

    use super::*;

    #[test]
    fn it_works() {
        let mut celsium = CelsiumProgram::new();
        let mut main_module = Module::new("main", &mut celsium);
        let mut main_block = Block::new();

        main_block.load_const(BUILTIN_TYPES::STRING, "John");
        main_block.load_const(BUILTIN_TYPES::MAGIC_INT, "37");

        main_block.create_object("Person", vec!["name", "age"]);
        main_block.define_variable(BUILTIN_TYPES::OBJECT, VISIBILITY::PUBLIC, "person_1");
        main_block.load_variable("person_1");
        main_block.call_print_function(true);

        let mut i = 0;
        while i < main_block.bytecode.len() {
            println!("{} {:?}", i, main_block.bytecode[i]);
            i += 1;
        }
        main_module.add_main_block(main_block);
        celsium.add_module(&main_module);
        celsium.run_program();
    }
}
