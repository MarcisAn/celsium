pub mod bytecode_parser;
pub mod bytecode;
pub mod std;
use ::std::collections::HashMap;

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

use crate::block::TextSpan;
use crate::vm::vm::CallStackItem;
use crate::std::*;

/// Macro to generate a match statement that calls functions based on string names
macro_rules! call_special_function {
    ($function:expr, $vm:expr, $(($name:literal => $func:ident)),* $(,)?) => {
        match $function.as_str() {
            $(
                $name => $func($vm),
            )*
            _ => unreachable!("Unknown special function: {}", $function),
        }
    };
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn wasm_print(s: &str);
    fn code_replace(replace_with: &str, line: usize, column: usize, span: usize);
    fn step();
    fn explain(a: &str, line: usize, column: usize, span: usize);
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

#[derive(Debug, Clone)]
pub struct CelsiumProgram {
    main_block: Block,
    functions: Vec<Function>,
    node_locations_by_id: HashMap<usize, TextSpan>,
    node_ids_by_line: HashMap<usize, Vec<usize>>,
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
    pub fn new(
        main_block: Block,
        functions: Vec<Function>,
        node_locations_by_id: HashMap<usize, TextSpan>,
        node_ids_by_line: HashMap<usize, Vec<usize>>
    ) -> CelsiumProgram {
        let mut bytecode = main_block.bytecode.clone();
        bytecode.push(OPTCODE::Return); // Return from the main function
        for function in &functions {
            let bytecode_index_of_this_function = &mut bytecode.len();
            bytecode.extend(function.body.bytecode.clone());
            bytecode.push(OPTCODE::Return); // Return from the user defined function
            let mut i = 0;
            while i < bytecode.len() {
                match &bytecode[i] {
                    OPTCODE::CallFunction { name } => {
                        if name == &function.signature.name {
                            bytecode[i] = OPTCODE::JumpToFunction {
                                target: *bytecode_index_of_this_function - 1,
                                function_name: Some(name.to_string()),
                            };
                        }
                    }
                    _ => (),
                }
                i += 1;
            }
        }
        let mut modified_block = main_block.clone();
        modified_block.bytecode = bytecode;
        CelsiumProgram {
            main_block: modified_block,
            functions,
            node_ids_by_line,
            node_locations_by_id,
        }
    }

    pub fn run_program(&mut self) -> Vec<StackValue> {
        let global_bytecode: Vec<OPTCODE> = self.main_block.bytecode.clone();
        let mut vm = VM::new();

        self.run(&mut vm, &global_bytecode);

        return vm.testing_stack;
    }

    pub fn get_bytecode_json(self) -> String {
        let global_bytecode: Vec<OPTCODE> = self.main_block.bytecode.clone();
        let json_bytecode = serde_json::to_string(&global_bytecode).unwrap();
        json_bytecode
    }

    pub fn get_bytecode(&self) -> Vec<OPTCODE> {
        self.main_block.bytecode.clone()
    }

    fn code_replace_calculate(&mut self, node_id: &usize, new_value: String) {
        let binding = self.clone();
        let span = binding.node_locations_by_id.get(node_id).unwrap();
        let delta_span = new_value.len() - span.length;
        #[cfg(target_family = "wasm")]
        code_replace(&new_value, span.line, span.col_start, span.length);

        let current_line = span.line;
        let other_nodes_on_same_line = self.node_ids_by_line.get(&current_line).unwrap();
        for node in other_nodes_on_same_line {
            let node_location = self.node_locations_by_id.get_mut(&node).unwrap();
            if span.col_start <= node_location.col_start && span.col_start + span.length >= node_location.col_start + node_location.length {
                node_location.length += delta_span;
            }
            if node_location.col_start > span.col_start + span.length {
                node_location.col_start += delta_span;
            }
        }
    }
    fn explain_process(&mut self, explanation: String, node_id: &usize) {
        let span = self.node_locations_by_id.get(node_id).unwrap();
        #[cfg(target_family = "wasm")]
        explain(&explanation, span.line, span.col_start, span.length);
    }

    fn run(&mut self, vm: &mut VM, bytecode: &Vec<OPTCODE>) {
        let mut index: usize = 0;

        while index < bytecode.len() {
            let optcode = &bytecode[index];
            let _ = match optcode {
                OPTCODE::Step => {
                    #[cfg(target_family = "wasm")]
                    step();
                }
                OPTCODE::PushToTestingStack { duplicate_stackvalue } =>
                    vm.push_to_testing_stack(*duplicate_stackvalue),
                OPTCODE::CallFunction { name } => {
                    vm.call_function(name, self);
                }
                OPTCODE::Add { node_id } => {
                    let (a, b, result) = vm.aritmethics("+");
                    let replace_value = vm::format_for_print::format_for_print(
                        &vm.stack.back().unwrap().clone(),
                        false
                    );
                    self.explain_process(
                        format!("Saskaitīšanas darbība: {} + {} = {}", a, b, result),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::Subtract { node_id } => {
                    let (a, b, result) = vm.aritmethics("-");
                    let replace_value = vm::format_for_print::format_for_print(
                        &vm.stack.back().unwrap().clone(),
                        false
                    );
                    self.explain_process(
                        format!("Atņemšanas darbība: {} - {} = {}", a, b, result),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::Multiply { node_id } => {
                    let (a, b, result) = vm.aritmethics("*");
                    let replace_value = vm::format_for_print::format_for_print(
                        &vm.stack.back().unwrap().clone(),
                        false
                    );
                    self.explain_process(
                        format!("Reizināšanas darbība: {} * {} = {}", a, b, result),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::Divide { node_id } => {
                    let (a, b, result) = vm.aritmethics("/");
                    let replace_value = vm::format_for_print::format_for_print(
                        &vm.stack.back().unwrap().clone(),
                        false
                    );
                    self.explain_process(
                        format!("Dalīšanas darbība: {} / {} = {}", a, b, result),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::Remainder { node_id } => {
                    let (a, b, result) = vm.aritmethics("%");
                    let replace_value = vm::format_for_print::format_for_print(
                        &vm.stack.back().unwrap().clone(),
                        false
                    );
                    self.explain_process(
                        format!("Atlikums {} dalot ar {} ir {}", a, b, result),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::JumpIfFalse {
                    steps,
                    jump_target_column: _,
                    jump_target_line: _,
                    is_skipable: _,
                } => {
                    if vm.must_jump() {
                        // println!("line: {}, col: {}", jump_target_line, jump_target_column);
                        index += *steps;
                    }
                }
                OPTCODE::Jump { steps } => {
                    index += *steps;
                }
                OPTCODE::JumpBack { steps } => {
                    index -= *steps;
                }
                OPTCODE::LessThan { node_id } => {
                    let (a, b, result) = vm.aritmethics("<");
                    let replace_value = vm::format_for_print::format_for_print(
                        &vm.stack.back().unwrap().clone(),
                        false
                    );
                    self.explain_process(
                        format!("Vai {} ir mazāks par {}? {}.", a, b, result),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::LargerThan { node_id } => {
                    let (a, b, result) = vm.aritmethics(">");
                    let replace_value = vm::format_for_print::format_for_print(
                        &vm.stack.back().unwrap().clone(),
                        false
                    );
                    self.explain_process(
                        format!("Vai {} ir lielāks par {}? {}.", a, b, result),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::LessOrEq { node_id } => {
                    let (a, b, result) = vm.aritmethics("<=");
                    let replace_value = vm::format_for_print::format_for_print(
                        &vm.stack.back().unwrap().clone(),
                        false
                    );
                    self.explain_process(
                        format!("Vai {} ir vienāds vai mazāks par {}? {}.", a, b, result),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::LargerOrEq { node_id } => {
                    let (a, b, result) = vm.aritmethics(">=");
                    let replace_value = vm::format_for_print::format_for_print(
                        &vm.stack.back().unwrap().clone(),
                        false
                    );
                    self.explain_process(
                        format!("Vai {} ir vienāds vai lielāks par {}? {}.", a, b, result),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::NotEq { node_id } => {
                    let (a, b, result) = vm.aritmethics("!=");
                    let replace_value = vm::format_for_print::format_for_print(
                        &vm.stack.back().unwrap().clone(),
                        false
                    );
                    self.explain_process(
                        format!("Vai {} ir nevienāds ar {}? {}.", a, b, result),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::Eq { node_id } => {
                    let (a, b, result) = vm.aritmethics("==");
                    let replace_value = vm::format_for_print::format_for_print(
                        &vm.stack.back().unwrap().clone(),
                        false
                    );
                    self.explain_process(
                        format!("Vai {} ir vienāds ar {}? {}.", a, b, result),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::Or { node_id } => {
                    let (a, b, result) = vm.aritmethics("or");
                    let replace_value = vm::format_for_print::format_for_print(
                        &vm.stack.back().unwrap().clone(),
                        false
                    );
                    self.explain_process(
                        format!(
                                "Vai {} un {} vismaz viena ir pateisa izteiksme? {}.",
                                a,
                                b,
                                result
                            ),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::And { node_id } => {
                    let (a, b, result) = vm.aritmethics("and");
                    let replace_value = vm::format_for_print::format_for_print(
                        &vm.stack.back().unwrap().clone(),
                        false
                    );
                    self.explain_process(
                        format!("Vai {} un {} ir patiesas izteksmes? {}.", a, b, result),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::Xor { node_id } => {
                    let (a, b, result) = vm.aritmethics("xor");
                    let replace_value = vm::format_for_print::format_for_print(&result, false);
                    self.explain_process(
                        format!(
                                "Vai {} vai {} ir patiesas izteksmes, bet ne abas? {}.",
                                a,
                                b,
                                result
                            ),
                        node_id
                    );
                    self.code_replace_calculate(node_id, replace_value);
                }
                OPTCODE::Not => vm.not(),
                OPTCODE::DefineVar { id } => vm.define_var(*id),
                OPTCODE::DefineObject { id } => {
                    let object = vm.pop();
                    let _ = vm.variables.insert(*id, Variable { id: *id, value: object });
                }
                OPTCODE::GetObjectField { field_name } => vm.get_object_field(field_name),
                OPTCODE::LoadVar { id, span: _ } => {
                    vm.load_var(*id);
                    // println!(
                    //     "value:{} line:{} col:{}, span:{}",
                    //     vm::format_for_print::format_for_print(
                    //         &vm.stack.back().unwrap().clone(),
                    //         false
                    //     ),
                    //     span.line,
                    //     span.col_start,
                    //     span.length
                    // );
                }
                OPTCODE::AssignVar { id } => vm.assign_var(*id),
                OPTCODE::CreateArray { init_values_count } => {
                    let mut init_values: Vec<StackValue> = vec![];
                    for _ in 0..*init_values_count {
                        init_values.push(vm.pop());
                    }
                    init_values.reverse();
                    vm.stack.push_back(StackValue::Array { value: init_values });
                }
                OPTCODE::GetIndex => vm.get_index(),
                OPTCODE::PushToArray { id } => vm.push_to_array(*id),
                OPTCODE::GettArrayLength { id } => vm.get_array_length(*id),
                OPTCODE::CallSpecialFunction { function } => {
                    call_special_function!(
                        function,
                        vm,
                        ("izvade" => izvade),
                        ("izvadetp" => izvadetp),
                        ("ievade" => ievade),
                        ("garums" => garums),
                        ("nejaušs" => nejauss),
                        ("nejaušs_robežās" => nejauss_robezas),
                    );
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
                OPTCODE::Break { span: _ } => todo!("Break should not appear in bytecode"),
                OPTCODE::Continue { span: _ } => todo!("Continue should not appear in bytecode"),
                OPTCODE::Return => {
                    let call_stack_item = vm.call_stack.pop_back();
                    if call_stack_item.is_none() {
                        break; //Programma beigusies
                    }
                    index = call_stack_item.unwrap().optode_index;
                }
                OPTCODE::JumpToFunction { target, function_name } => {
                    vm.call_stack.push_back(CallStackItem {
                        optode_index: index,
                        function_name: function_name.clone(),
                    });
                    index = *target;
                }
                OPTCODE::SetObjectField { id, field_name } => vm.set_object_field(*id, field_name),
                OPTCODE::CopyVariableValue { src_var_id, dst_var_id } => {
                    vm.copy_var_value(*src_var_id, *dst_var_id);
                }
            };
            index += 1;
        }
    }
}
