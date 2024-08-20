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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord,serde::Deserialize, serde::Serialize)]
pub struct ObjectFieldType {
    pub name: String,
    pub data_type: BuiltinTypes,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord,serde::Deserialize, serde::Serialize)]
pub enum BuiltinTypes {
    MagicInt,
    Bool,
    String,
    Object {
        fields: Vec<ObjectFieldType>,
    },
    Array {element_type: Box<BuiltinTypes>},
    Float,
}

#[derive(Clone, Debug,serde::Deserialize, serde::Serialize)]
pub enum SpecialFunctions {
    Print {
        newline: bool,
    },
    Input,
    Random,
}
pub struct CelsiumProgram {
    modules: Vec<Module>,
}

#[derive(Debug, Clone, PartialEq,serde::Deserialize, serde::Serialize)]
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

    pub fn run_program(&mut self) -> Vec<StackValue> {
        let mut global_bytecode: Vec<OPTCODE> = vec![];
        for module in &self.modules {
            global_bytecode.append(&mut module.clone().main_block.unwrap().bytecode.clone());
        }
        let mut vm = VM::new();

        self.run(&mut vm, &global_bytecode);
        let json_bytecode = serde_json::to_string(&global_bytecode).unwrap();
        println!("{}", json_bytecode);
        return vm.testing_stack;
    }

    pub fn run(&mut self, vm: &mut VM, bytecode: &Vec<OPTCODE>) {
        let mut index: isize = 0;
        while index < (bytecode.len() as isize) {
            let optcode = &bytecode[index as usize];
            //println!("running optcode {:?}", optcode);
            match optcode {
                OPTCODE::PushToTestingStack{duplicate_stackvalue} => vm.push_to_testing_stack(*duplicate_stackvalue),
                OPTCODE::LoadConst { data_type, data } => vm.push(&data_type, &data),
                OPTCODE::CallFunction { name } => {
                    vm.call_function(name, self);
                }
                OPTCODE::ReturnFromFunction => {
                    break;
                }
                OPTCODE::CallFunctionWithBytecode { bytecode: _ } => {
                    panic!();
                }
                OPTCODE::Add => vm.aritmethics("+"),
                OPTCODE::Subtract => vm.aritmethics("-"),
                OPTCODE::Multiply => vm.aritmethics("*"),
                OPTCODE::Divide => vm.aritmethics("/"),
                OPTCODE::Remainder => vm.aritmethics("%"),
                OPTCODE::JumpIfFalse { steps } => {
                    if vm.must_jump() {
                        index += *steps as isize;
                    }
                }
                OPTCODE::Jump { steps } => {
                    index += *steps as isize;
                }
                OPTCODE::JumpBack { steps } => {
                    index -= *steps as isize;
                }
                OPTCODE::LessThan => vm.aritmethics("<"),
                OPTCODE::LargerThan => vm.aritmethics(">"),
                OPTCODE::LessOrEq => vm.aritmethics("<="),
                OPTCODE::LargerOrEq => vm.aritmethics(">="),
                OPTCODE::NotEq => vm.aritmethics("!="),
                OPTCODE::Eq => vm.aritmethics("=="),
                OPTCODE::Or => vm.aritmethics("or"),
                OPTCODE::And => vm.aritmethics("and"),
                OPTCODE::Xor => vm.aritmethics("xor"),
                OPTCODE::DefineVar { id } => vm.define_var(*id),
                OPTCODE::DefineObject { id } => {
                    let object = vm.pop();
                    let _ = vm.variables.insert(*id, Variable { id: *id, value: object });
                }
                OPTCODE::GetObjectField { field_name } => vm.get_object_field(field_name),
                OPTCODE::LoadVar { id } => vm.load_var(*id),
                OPTCODE::AssignVar { id } => vm.assign_var(*id),
                OPTCODE::DefineArray { id, init_values_count } => {
                    let mut init_values: Vec<StackValue> = vec![];
                    for _ in 0..*init_values_count {
                        init_values.push(vm.pop());
                    }
                    init_values.reverse();
                    vm.stack.push_back(StackValue::ARRAY { value: init_values });
                    vm.define_var(*id);
                }
                OPTCODE::GetFromArray { id } => vm.get_from_array(*id),
                OPTCODE::PushToArray { id } => vm.push_to_array(*id),
                OPTCODE::GettArrayLength { id } => vm.get_array_length(*id),
                OPTCODE::DefineFunction { body_block, visibility:_, signature } =>
                    self.modules[0].functions.push(Function {
                        signature: signature.clone(),
                        body: body_block.clone(),
                    }),

                OPTCODE::CallSpecialFunction { function } =>
                    match function {
                        SpecialFunctions::Print { newline } => {
                            let printable = &vm.format_for_print(*newline);
                            #[cfg(target_family = "wasm")]
                            wasm_print(printable);
                            print!("{}", printable);
                        }
                        SpecialFunctions::Input => {
                            #[cfg(target_family = "wasm")]
                            vm.push(&BuiltinTypes::String, &"asdfghjkl".to_string());
                            #[cfg(target_family = "wasm")]
                            async {
                                let value = &wasm_input().await.as_string().unwrap();
                                vm.push(&BuiltinTypes::String, value);
                            };
                            #[cfg(not(target_family = "wasm"))]
                            vm.input("");
                        }
                        SpecialFunctions::Random => {
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
                OPTCODE::AssignAtArrayIndex { id } => vm.set_at_array(*id),
                OPTCODE::SimpleLoop { body_block } =>
                    vm.simple_loop(self, body_block.bytecode.clone()),
                OPTCODE::CreateObject { field_names } => {
                    let mut fields = vec![];
                    let mut field_names_reversed = field_names.clone();
                    field_names_reversed.reverse();
                    for fieldname in field_names_reversed {
                        fields.push(ObjectField { name: fieldname.to_string(), value: vm.pop() });
                    }
                    vm.push_stackvalue(StackValue::Object { value: fields.clone() })
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
