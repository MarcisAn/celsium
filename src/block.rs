use std::io::{self, Result};

use crate::{module::VISIBILITY, BINOP, BUILTIN_TYPES, OPTCODE};

#[derive(Clone, Debug)]
pub struct Block {
    pub bytecode: Vec<OPTCODE>,
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
            BINOP::LESS_THAN => OPTCODE::LESS_THAN,
            BINOP::LARGER_THAN => OPTCODE::LARGER_THAN,
            BINOP::LESS_OR_EQ => OPTCODE::LESS_OR_EQ,
            BINOP::LARGER_OR_EQ => OPTCODE::LARGER_OR_EQ,
            BINOP::NOT_EQ => OPTCODE::NOT_EQ,
            BINOP::EQ => OPTCODE::EQ,
            BINOP::AND => OPTCODE::AND,
            BINOP::OR => OPTCODE::OR,
            BINOP::XOR => OPTCODE::XOR,
        });
    }
    pub fn define_if_block(&mut self, block: Block) {
        let block_length = block.bytecode.len();
        self.bytecode.push(OPTCODE::JUMP_IF_FALSE {
            steps: block_length,
        });
        for optcode in block.bytecode {
            self.bytecode.push(optcode);
        }
    }
    pub fn define_if_else_block(&mut self, if_block: Block, else_block: Block) {
        //println!("{:?}", else_block);
        let if_block_length = if_block.bytecode.len();
        let else_block_length = else_block.bytecode.len();
        self.bytecode.push(OPTCODE::JUMP_IF_FALSE {
            steps: if_block_length + 1,
        });
        for optcode in if_block.bytecode {
            self.bytecode.push(optcode);
        }
        self.bytecode.push(OPTCODE::JUMP {
            steps: else_block_length,
        });
        for optcode in else_block.bytecode {
            self.bytecode.push(optcode);
        }
    }
    pub fn call_function(&mut self, name: &str) {
        self.bytecode.push(OPTCODE::CALL_FUNCTION {
            name: name.to_string(),
        });
    }
    pub fn call_print_function(&mut self, newline: bool) {
        self.bytecode
            .push(OPTCODE::CALL_PRINT_FUNCTION { newline: newline });
    }
    pub fn define_simple_loop(&mut self, block: Block, loops: usize) {
        for repeat in 0..loops {
            for optcode in &block.bytecode {
                self.bytecode.push(optcode.clone());
            }
        }
    }
    pub fn define_variable(
        &mut self,
        data_type: BUILTIN_TYPES,
        visibility: VISIBILITY,
        name: &str,
    ) {
        self.bytecode.push(OPTCODE::DEFINE_VAR {
            data_type,
            visibility,
            name: name.to_string(),
        })
    }
    pub fn load_variable(&mut self, name: &str) {
        self.bytecode.push(OPTCODE::LOAD_VAR {
            name: name.to_string(),
        })
    }
}
