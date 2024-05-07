use num::FromPrimitive;
use num::ToPrimitive;
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
use num::bigint;
use num::bigint::RandBigInt;
use num::BigInt;
use num::BigUint;
use rand::prelude::*;
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
    FLOAT
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

#[derive(Clone, Debug)]
pub enum SpecialFunctions {
    PRINT {newline: bool},
    INPUT,
    RANDOM,
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
        let mut global_bytecode: Vec<OPTCODE> = vec![];
        for module in &self.modules {
            global_bytecode.append(&mut module.clone().main_block.unwrap().bytecode.clone());
        }
        let mut vm = VM::new();

        self.run(&mut vm, &global_bytecode);
    }

    pub fn run(&mut self, vm: &mut VM, bytecode: &Vec<OPTCODE>)  {
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
                OPTCODE::CALL_FUNCTION_WITH_BYTECODE { bytecode: _ } => {
                    panic!();
                }
                OPTCODE::ADD => vm.aritmethics("+"),
                OPTCODE::SUBTRACT => vm.aritmethics("-"),
                OPTCODE::MULTIPLY => vm.aritmethics("*"),
                OPTCODE::DIVIDE => vm.aritmethics("/"),
                OPTCODE::REMAINDER => vm.aritmethics("%"),
                OPTCODE::JUMP_IF_FALSE { steps } => {
                    if vm.must_jump() {
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
                    data_type: _,
                    visibility,
                    name,
                } => vm.define_var(0, name.to_string(), visibility),
                OPTCODE::LOAD_VAR { name } => vm.load_var(name),
                OPTCODE::ASSIGN_VAR { name } => vm.assign_var(name),
                OPTCODE::DefineArray {
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
                OPTCODE::CALL_SPECIAL_FUNCTION { function } => match function {
                    SpecialFunctions::PRINT{newline} => {
                        let printable = &vm.format_for_print(*newline);
                        #[cfg(target_family = "wasm")]
                        wasm_print(printable);
                        print!("{}", printable);
                    }
                    SpecialFunctions::INPUT => {
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
                    SpecialFunctions::RANDOM => {
                        let value = {
                            let max = match vm.pop() {
                                StackValue::BIGINT { value } => truncate_biguint_to_i64(&value),
                                _ => panic!()
                            };
                            let min = match vm.pop() {
                                StackValue::BIGINT { value } => truncate_biguint_to_i64(&value),
                                _ => panic!()
                            };
                            bigint::BigInt::from_i64(rand::thread_rng().gen_range(min..max)).unwrap()};
                        vm.push_stackvalue(StackValue::BIGINT {
                        value: value,
                    })},
                },
            }
            index += 1;
        }
    }
}

fn truncate_biguint_to_i64(a: &BigInt) -> i64 {
    let mask = BigInt::from(u64::MAX);
    (a & mask).to_i64().unwrap()
}

#[cfg(test)]
mod tests {
    use self::module::VISIBILITY;

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
        main_block.call_special_function(SpecialFunctions::PRINT{newline: true});

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
