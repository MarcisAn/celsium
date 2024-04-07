use crate::{CelsiumConfig, OPTCODE};
pub mod vm;
use num::bigint::{BigInt, ToBigInt};
use vm::VM;
mod math_operators;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn wasm_print(s: &str);
}

#[derive(Debug, PartialEq, Clone)]
pub enum StackValue {
    BOOL { value: bool },
    BIGINT { value: BigInt },
    STRING { value: String },
    ARRAY { value: Vec<StackValue> },
}

pub(super) fn run(bytecode: &Vec<OPTCODE>, config: &CelsiumConfig) {
    let mut vm = VM::new();

    let mut index: isize = 0;
    while index < bytecode.len() as isize {
        let optcode = &bytecode[index as usize];
        //println!("running optcode {:?}", optcode);
        match optcode {
            OPTCODE::LOAD_CONST { data_type, data } => vm.push(&data_type, &data),
            OPTCODE::CALL_FUNCTION { name } => {
                panic!("Non-replaced call function optcode in bytecode")
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
            OPTCODE::LOAD_VAR { name } => vm.load_var(name),
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
            } => panic!("Define function should not be in bytecoed"),
        }
        index += 1;
    }
}
