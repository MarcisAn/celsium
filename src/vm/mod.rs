use crate::OPTCODE;
pub mod vm;
use num::bigint::{BigInt, ToBigInt};
use vm::VM;
mod math_operators;

#[derive(Debug, PartialEq)]
pub enum StackValue {
    BOOL { value: bool },
    BIGINT { value: BigInt },
    STRING { value: String },
}

pub(super) fn run(bytecode: &Vec<OPTCODE>) {
    println!("{:?}", bytecode);
    let mut vm = VM::new();

    let mut index = 0;
    while index < bytecode.len() {
        let optcode = &bytecode[index];
        match optcode {
            OPTCODE::LOAD_CONST { data_type, data } => vm.push(&data_type, &data),
            OPTCODE::CALL_FUNCTION { name } => panic!("Call function in bytecode"),
            OPTCODE::ADD => vm.aritmethics("+"),
            OPTCODE::SUBTRACT => vm.aritmethics("-"),
            OPTCODE::MULTIPLY => vm.aritmethics("*"),
            OPTCODE::DIVIDE => vm.aritmethics("/"),
            OPTCODE::REMAINDER => vm.aritmethics("%"),
            OPTCODE::CALL_PRINT_FUNCTION { newline } => vm.print_function(*newline),
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
        }
        index += 1;
    }
}
