use std::io::{self, Result};

use crate::{BINOP, BUILTIN_TYPES, OPTCODE};

#[derive(Clone)]
pub struct Block {
    pub bytecode: Vec<OPTCODE>,
}

pub struct FunctionSignature {
    pub(crate) name: String,
    pub(crate) return_type: String,

    pub(crate) args: Vec<FuncArg>,
}
pub struct FuncArg {
    name: String,
    arg_type: String,
}
pub enum FUNC_VISIBILITY {
    PRIVATE,
    PUBLIC,
}

impl Block {
    pub fn new() -> Block {
        Block { bytecode: vec![] }
    }
    pub fn load_const(&mut self, data_type: BUILTIN_TYPES, value: &str) {
        self.bytecode.push(OPTCODE::LOAD_CONST {
            data: value.to_owned(),
            data_type,
        });
    }
    pub fn binop(&mut self, operator: BINOP) {
        self.bytecode.push(match operator {
            BINOP::ADD => OPTCODE::ADD,
            BINOP::SUBTRACT => OPTCODE::SUBTRACT,
            BINOP::MULTIPLY => OPTCODE::MULTIPLY,
            BINOP::DIVIDE => OPTCODE::DIVIDE,
            BINOP::REMAINDER => OPTCODE::REMAINDER,
        });
    }
    pub fn call_function(&mut self, name: &str) {
        self.bytecode.push(OPTCODE::CALL_FUNCTION {
            name: name.to_string(),
        });
    }
    pub fn call_print_function(&mut self) {
        self.bytecode.push(OPTCODE::CALL_PRINT_FUNCTION);
    }
}
