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
}

pub(super) fn run(bytecode: &Vec<OPTCODE>, config: &CelsiumConfig) {
    let mut vm = VM::new();

    let mut index = 0;
    while index < bytecode.len() {
        let optcode = &bytecode[index];
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
                    index += steps
                }
            }
            OPTCODE::JUMP { steps } => index += steps,
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
        }
        index += 1;
    }
}
