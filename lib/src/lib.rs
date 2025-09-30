pub mod bytecode_parser;
pub mod bytecode;

use block::Block;
use bytecode::{ BINOP, OPTCODE };
use module::Function;
extern crate serde;
extern crate serde_json;
use rand::prelude::*;

extern crate js_sys;
pub mod compiletime_helper;
pub mod block;
pub mod module;
pub mod vm;
pub mod typestack;
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub struct ObjectFieldType {
    pub name: String,
    pub data_type: BuiltinTypes,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum BuiltinTypes {
    Int,
    Bool,
    String,
    Object {
        fields: Vec<ObjectFieldType>,
    },
    Array {
        element_type: Box<BuiltinTypes>,
        length: Option<usize>,
    },
    Float,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum SpecialFunctions {
    Print {
        newline: bool,
    },
    Input,
    Random,
    Length,
}
pub struct CelsiumProgram {
    main_block: Block,
    functions: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
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
    pub fn new(main_block: Block, functions: Vec<Function>) -> CelsiumProgram {
        CelsiumProgram { main_block, functions }
    }

    pub fn run_program(&mut self) -> Vec<StackValue> {
        let global_bytecode: Vec<OPTCODE> = self.main_block.bytecode.clone();
        let mut vm = VM::new();

        self.run(&mut vm, &global_bytecode);

        return vm.testing_stack;
    }

    pub fn get_bytecode(mut self) -> String {
        let global_bytecode: Vec<OPTCODE> = self.main_block.bytecode.clone();
        let json_bytecode = serde_json::to_string(&global_bytecode).unwrap();
        json_bytecode
    }

    fn run(&mut self, vm: &mut VM, bytecode: &Vec<OPTCODE>) {
        let mut index: isize = 0;
        while index < (bytecode.len() as isize) {
            let optcode = &bytecode[index as usize];
            match optcode {
                OPTCODE::PushToTestingStack { duplicate_stackvalue } =>
                    vm.push_to_testing_stack(*duplicate_stackvalue),
                OPTCODE::CallFunction { name } => {
                    vm.call_function(name, self);
                }
                OPTCODE::ReturnFromFunction => {
                    break;
                }
                OPTCODE::CallFunctionWithBytecode { bytecode: _ } => {
                    panic!();
                }
                OPTCODE::Add { span } => {
                    vm.aritmethics("+");
                    println!(
                        "value:{} line:{} col:{}, span:{}",
                        vm::format_for_print::format_for_print(
                            &vm.stack.back().unwrap().clone(),
                            false
                        ),
                        span.line,
                        span.col_start,
                        span.length
                    );
                }
                OPTCODE::Subtract => vm.aritmethics("-"),
                OPTCODE::Multiply => vm.aritmethics("*"),
                OPTCODE::Divide => vm.aritmethics("/"),
                OPTCODE::Remainder => vm.aritmethics("%"),
                OPTCODE::JumpIfFalse { steps, jump_target_column, jump_target_line } => {
                    if vm.must_jump() {
                        println!("line: {}, col: {}", jump_target_line, jump_target_column);
                        index += *steps as isize;
                    }
                }
                OPTCODE::Jump { steps } => {
                    index += *steps as isize;
                }
                OPTCODE::JumpBack { steps } => {
                    index -= *steps as isize;
                }
                OPTCODE::LessThan { span } => {
                    vm.aritmethics("<");
                    println!(
                        "value:{} line:{} col:{}, span:{}",
                        vm::format_for_print::format_for_print(
                            &vm.stack.back().unwrap().clone(),
                            false
                        ),
                        span.line,
                        span.col_start,
                        span.length
                    );
                }
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
                OPTCODE::LoadVar { id, span } => {
                    vm.load_var(*id);
                    println!(
                        "value:{} line:{} col:{}, span:{}",
                        vm::format_for_print::format_for_print(
                            &vm.stack.back().unwrap().clone(),
                            false
                        ),
                        span.line,
                        span.col_start,
                        span.length
                    );
                },
                OPTCODE::AssignVar { id } => vm.assign_var(*id),
                OPTCODE::CreateArray { init_values_count } => {
                    let mut init_values: Vec<StackValue> = vec![];
                    for _ in 0..*init_values_count {
                        init_values.push(vm.pop());
                    }
                    init_values.reverse();
                    vm.stack.push_back(StackValue::Array { value: init_values });
                }
                OPTCODE::GetFromArray { id } => vm.get_from_array(*id),
                OPTCODE::PushToArray { id } => vm.push_to_array(*id),
                OPTCODE::GettArrayLength { id } => vm.get_array_length(*id),
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
                                    StackValue::Int { value } => value,
                                    _ => panic!(),
                                };
                                let min = match vm.pop() {
                                    StackValue::Int { value } => value,
                                    _ => panic!(),
                                };
                                rand::thread_rng().gen_range(min..max)
                            };
                            vm.push_stackvalue(StackValue::Int {
                                value,
                            });
                        }
                        SpecialFunctions::Length => {
                            let value = vm.pop();
                            let length_value = match value {
                                StackValue::Bool { value } => 1,
                                StackValue::Int { value } => value.to_string().len(),
                                StackValue::Float { value } => value.to_string().len(),
                                StackValue::String { value } => value.len(),
                                StackValue::Array { value } => value.len(),
                                StackValue::Object { value } => value.len(),
                            };
                            vm.push_stackvalue(StackValue::Int { value: length_value as i64 });
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
                    vm.push_stackvalue(StackValue::Object { value: fields.clone() });
                }
                OPTCODE::LoadInt { value } => vm.push_stackvalue(StackValue::Int { value: *value }),
                OPTCODE::LoadBool { value } =>
                    vm.push_stackvalue(StackValue::Bool { value: *value }),
                OPTCODE::LoadString { value } =>
                    vm.push_stackvalue(StackValue::String { value: value.to_string() }),
                OPTCODE::LoadFloat { value } =>
                    vm.push_stackvalue(StackValue::Float { value: *value }),
            }
            index += 1;
        }
    }
}
