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
    pub data_type: BuiltinTypes,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BuiltinTypes {
    MagicInt,
    Bool,
    String,
    Object {
        fields: Vec<ObjectFieldType>,
    },
    Array {
        element_type: Box<BuiltinTypes>,
    },
    Float,
}

#[derive(Clone, Debug)]
pub enum SpecialFunctions {
    Print {
        newline: bool,
    },
    Input,
    Random {
        min: usize,
        max: usize,
    },
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

    pub fn run_program(&mut self, register_count: usize) -> Vec<StackValue> {
        let mut global_bytecode: Vec<OPTCODE> = vec![];
        for module in &self.modules {
            global_bytecode.append(&mut module.clone().main_block.unwrap().bytecode.clone());
        }
        let mut vm = VM::new(register_count);

        let test_bytecode = vec![
            OPTCODE::LoadConst {
                data_type: BuiltinTypes::MagicInt,
                data: "2".to_string(),
                register: 0,
            },
            OPTCODE::LoadConst {
                data_type: BuiltinTypes::MagicInt,
                data: "200".to_string(),
                register: 1,
            },
            OPTCODE::Binop { a_reg: 0, b_reg: 1, result_reg: 1, binop: crate::BINOP::LargerThan },
            OPTCODE::JumpIfFalse{register: 1, steps: 4},
            OPTCODE::CallSpecialFunction { function: crate::SpecialFunctions::Print { newline: true }, register: 0 },
            OPTCODE::LoadConst { data_type: BuiltinTypes::MagicInt, data: "1".to_string(), register: 2 },
            OPTCODE::Binop { a_reg: 0, b_reg: 2, result_reg: 0, binop: crate::BINOP::Add },
            OPTCODE::JumpBack { steps: 7 }


        ];

        self.run(&mut vm, &test_bytecode);
        return vm.testing_stack;
    }

    pub fn run(&mut self, vm: &mut VM, bytecode: &Vec<OPTCODE>) {
        let mut index: isize = 0;
        while index < (bytecode.len() as isize) {
            let optcode = &bytecode[index as usize];
            match optcode {
                OPTCODE::PushToTestingStack { duplicate_stackvalue, register } =>
                    vm.push_to_testing_stack(*register),
                OPTCODE::LoadConst { data_type, data, register } =>
                    vm.push(&data_type, &data, *register),
                OPTCODE::CallFunction { name } => {
                    vm.call_function(name, self);
                }
                OPTCODE::ReturnFromFunction => {
                    break;
                }
                OPTCODE::CallFunctionWithBytecode { bytecode: _ } => {
                    panic!();
                }
                OPTCODE::Binop { a_reg, b_reg, result_reg, binop } =>
                    vm.aritmethics(binop.clone(), *a_reg, *b_reg, *result_reg),
                OPTCODE::JumpIfFalse { steps, register } => {
                    if vm.must_jump(*register) {
                        index += *steps as isize;
                    }
                }
                OPTCODE::Jump { steps } => {
                    index += *steps as isize;
                }
                OPTCODE::JumpBack { steps } => {
                    index -= *steps as isize;
                }
                OPTCODE::DefineVar { id, register } => vm.define_var(*id, *register),
                OPTCODE::DefineObject { id, register } => {
                    let object = vm.get_register(*register);
                    let _ = vm.variables.insert(*id, Variable { id: *id, value: object });
                }
                OPTCODE::GetObjectField { field_name, object_register } =>
                    vm.get_object_field(field_name, *object_register),
                OPTCODE::LoadVar { id, register } => vm.load_var(*id, *register),
                OPTCODE::AssignVar { id, register } => vm.assign_var(*id, *register),
                OPTCODE::DefineArray { id, init_values } => {
                    let mut init_stackvalues: Vec<StackValue> = vec![];
                    for reg in init_values {
                        init_stackvalues.push(vm.get_register(*reg));
                    }
                    init_stackvalues.reverse();
                    let value = StackValue::ARRAY { value: init_stackvalues };
                    vm.define_var_with_stackvalue(*id, value);
                }
                OPTCODE::GetFromArray { id, register } => vm.get_from_array(*id, *register),
                OPTCODE::PushToArray { id, register } => vm.push_to_array(*id, *register),
                OPTCODE::GetArrayLength { id, register } => vm.get_array_length(*id, *register),
                OPTCODE::DefineFunction { body_block, visibility: _, signature } =>
                    self.modules[0].functions.push(Function {
                        signature: signature.clone(),
                        body: body_block.clone(),
                    }),

                OPTCODE::CallSpecialFunction { function, register } =>
                    match function {
                        SpecialFunctions::Print { newline } => {
                            let printable = &vm.format_for_print(*newline, *register);
                            #[cfg(target_family = "wasm")]
                            wasm_print(printable);
                            print!("{}", printable);
                        }
                        SpecialFunctions::Input => {
                            #[cfg(target_family = "wasm")]
                            async {
                                let value = &wasm_input().await.as_string().unwrap();
                                vm.push(&BuiltinTypes::String, value);
                            };
                            #[cfg(not(target_family = "wasm"))]
                            vm.input("", *register);
                        }
                        SpecialFunctions::Random { min, max } => {
                            let value = {
                                let max = match vm.get_register(*max) {
                                    StackValue::BIGINT { value } => truncate_biguint_to_i64(&value),
                                    _ => panic!(),
                                };
                                let min = match vm.get_register(*min) {
                                    StackValue::BIGINT { value } => truncate_biguint_to_i64(&value),
                                    _ => panic!(),
                                };
                                bigint::BigInt
                                    ::from_i64(rand::thread_rng().gen_range(min..max))
                                    .unwrap()
                            };
                            vm.registers[*register] = Some(StackValue::BIGINT {
                                value: value,
                            });
                        }
                    }
                OPTCODE::AssignAtArrayIndex { id, value_reg, index_reg } =>
                    vm.set_at_array(*id, *value_reg, *index_reg),
                OPTCODE::SimpleLoop { body_block, count_reg } =>
                    vm.simple_loop(self, body_block.bytecode.clone(), *count_reg),
                OPTCODE::CreateObject { field_names, target_reg, field_regs } => {
                    let mut fields = vec![];
                    let mut field_names_reversed = field_names.clone();
                    field_names_reversed.reverse();
                    let mut counter = 0;
                    for fieldname in field_names_reversed {
                        fields.push(ObjectField {
                            name: fieldname.to_string(),
                            value: vm.registers[counter].clone().unwrap(),
                        });
                        counter += 1;
                    }
                    vm.registers[*target_reg] = Some(StackValue::Object { value: fields.clone() });
                }
            }
            index += 1;
        }
    }
}

fn truncate_biguint_to_i64(a: &BigInt) -> i64 {
    let mask = BigInt::from(u64::MAX);
    (a & mask).to_i64().unwrap()
}
