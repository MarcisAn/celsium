use crate::OPTCODE;
pub mod vm;
use num::bigint::{BigInt, ToBigInt};
use vm::VM;

#[derive(Debug)]
pub(crate) enum StackValue {
    BOOL { value: bool },
    BIGINT { value: BigInt },
    STRING { value: String },
}

pub(super) fn run(bytecode: &Vec<OPTCODE>) {
    let mut vm = VM::new();

    for optcode in bytecode {
        match optcode {
            OPTCODE::LOAD_CONST { data_type, data } => vm.push(data_type, data),
            OPTCODE::CALL_FUNCTION { name } => todo!(),
            OPTCODE::ADD => vm.aritmethics("+"),
            OPTCODE::SUBTRACT => vm.aritmethics("-"),
            OPTCODE::MULTIPLY => vm.aritmethics("*"),
            OPTCODE::DIVIDE => vm.aritmethics("/"),
            OPTCODE::REMAINDER => vm.aritmethics("%"),
            OPTCODE::CALL_PRINT_FUNCTION => vm.print_function(),
        }
    }
}
