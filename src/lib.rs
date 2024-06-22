use num::FromPrimitive;
use num::ToPrimitive;
pub mod bytecode;
use bytecode::{ BINOP, OPTCODE };
use module::Function;
use module::Module;
extern crate serde;
extern crate serde_json;
use num::bigint;
use num::BigInt;
use rand::prelude::*;

extern crate js_sys;
pub mod compiletime_helper;
pub mod block;
pub mod module;
pub mod vm;
use vm::vm::Variable;
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectFieldType {
    pub name: String,
    pub data_type: BUILTIN_TYPES,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BUILTIN_TYPES {
    MAGIC_INT,
    BOOL,
    STRING,
    OBJECT {
        fields: Vec<ObjectFieldType>,
    },
    FLOAT,
}

#[derive(Clone, Debug)]
pub enum SpecialFunctions {
    PRINT {
        newline: bool,
    },
    INPUT,
    RANDOM,
}
pub struct CelsiumProgram {
    modules: Vec<Module>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub ast_id: usize,
    pub module_path: String,
}

impl Scope {
    pub fn change_ast_id(&mut self, new_id: usize) -> Scope {
        let mut new = self.clone();
        new.ast_id = new_id;
        return new;
    }
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

    pub fn run(&mut self, vm: &mut VM, bytecode: &Vec<OPTCODE>) {
        let mut index: isize = 0;
        while index < (bytecode.len() as isize) {
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
                        index += *steps as isize;
                    }
                }
                OPTCODE::JUMP { steps } => {
                    index += *steps as isize;
                }
                OPTCODE::JUMP_BACK { steps } => {
                    index -= *steps as isize;
                }
                OPTCODE::LESS_THAN => vm.aritmethics("<"),
                OPTCODE::LARGER_THAN => vm.aritmethics(">"),
                OPTCODE::LESS_OR_EQ => vm.aritmethics("<="),
                OPTCODE::LARGER_OR_EQ => vm.aritmethics(">="),
                OPTCODE::NOT_EQ => vm.aritmethics("!="),
                OPTCODE::EQ => vm.aritmethics("=="),
                OPTCODE::OR => vm.aritmethics("or"),
                OPTCODE::AND => vm.aritmethics("and"),
                OPTCODE::XOR => vm.aritmethics("xor"),
                OPTCODE::DEFINE_VAR { id } => vm.define_var(*id),
                OPTCODE::DefineObject { id, field_names } => {
                    let object = vm.pop();
                    let _ = vm.variables.insert(*id, Variable { id: *id, value: object });
                }
                OPTCODE::LOAD_VAR { id } => vm.load_var(*id),
                OPTCODE::ASSIGN_VAR { id } => vm.assign_var(*id),
                OPTCODE::DefineArray { id, init_values_count } => {
                    let mut init_values: Vec<StackValue> = vec![];
                    for _ in 0..*init_values_count {
                        init_values.push(vm.pop());
                    }
                    init_values.reverse();
                    vm.stack.push_back(StackValue::ARRAY { value: init_values });
                    vm.define_var(*id);
                }
                OPTCODE::GET_FROM_ARRAY { id } => vm.get_from_array(*id),
                OPTCODE::PUSH_TO_ARRAY { id } => vm.push_to_array(*id),
                OPTCODE::GET_ARRAY_LENGTH { id } => vm.get_array_length(*id),
                OPTCODE::DEFINE_FUNCTION { body_block, visibility, signature } =>
                    self.modules[0].functions.push(Function {
                        signature: signature.clone(),
                        body: body_block.clone(),
                        visibility: visibility.clone(),
                    }),

                OPTCODE::CallSpecialFunction { function } =>
                    match function {
                        SpecialFunctions::PRINT { newline } => {
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
                                    _ => panic!(),
                                };
                                let min = match vm.pop() {
                                    StackValue::BIGINT { value } => truncate_biguint_to_i64(&value),
                                    _ => panic!(),
                                };
                                bigint::BigInt
                                    ::from_i64(rand::thread_rng().gen_range(min..max))
                                    .unwrap()
                            };
                            vm.push_stackvalue(StackValue::BIGINT {
                                value: value,
                            });
                        }
                    }
                OPTCODE::ASSIGN_AT_ARRAY_INDEX { id } => vm.set_at_array(*id),
                OPTCODE::SimpleLoop { body_block } =>
                    vm.simple_loop(self, body_block.bytecode.clone()),
                OPTCODE::CreateObject { field_names } => {
                    let mut fields = vec![];
                    let mut field_names_reversed = field_names.clone();
                    field_names_reversed.reverse();
                    for fieldname in field_names_reversed {
                        fields.push(ObjectField { name: fieldname.to_string(), value: vm.pop() });
                    }
                    vm.push_stackvalue(StackValue::OBJECT { value: fields.clone() })
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
