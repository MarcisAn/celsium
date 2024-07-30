
use crate::module::FunctionSignature;
use crate::{Scope, SpecialFunctions};
use crate::{ module::VISIBILITY, BINOP, BuiltinTypes, OPTCODE };
mod array;

#[derive(Clone, Debug)]
pub struct Block {
    pub bytecode: Vec<OPTCODE>,
    pub scope: Scope
}


impl Block {
    pub fn new(scope: Scope) -> Block {
        Block { bytecode: vec![], scope }
    }
    pub fn load_const(&mut self, data_type: BuiltinTypes, value: &str, target_reg: usize) {
        self.bytecode.push(OPTCODE::LoadConst {
            data: value.to_owned(),
            data_type,
            register: target_reg,
        });
    }
    pub fn binop(&mut self, operator: BINOP, a_reg: usize, b_reg: usize, result_reg: usize) {
        self.bytecode.push(OPTCODE::Binop { a_reg, b_reg, result_reg, binop: operator });
    }
    pub fn define_if_block(&mut self, block: Block, condition_reg: usize) {
        let block_length = block.bytecode.len();
        self.bytecode.push(OPTCODE::JumpIfFalse {
            steps: block_length,
            register: condition_reg,
        });
        for optcode in block.bytecode {
            self.bytecode.push(optcode);
        }
    }
    pub fn define_if_else_block(&mut self, if_block: Block, else_block: Block, condition_reg: usize) {
        //println!("{:?}", else_block);
        let if_block_length = if_block.bytecode.len();
        let else_block_length = else_block.bytecode.len();
        self.bytecode.push(OPTCODE::JumpIfFalse {
            steps: if_block_length + 1,
            register: condition_reg,
        });
        for optcode in if_block.bytecode {
            self.bytecode.push(optcode);
        }
        self.bytecode.push(OPTCODE::Jump {
            steps: else_block_length,
        });
        for optcode in else_block.bytecode {
            self.bytecode.push(optcode);
        }
    }
    pub fn call_function(&mut self, name: &str) {
        self.bytecode.push(OPTCODE::CallFunction {
            name: name.to_string(),
        });
    }
    pub fn define_simple_loop(&mut self, loop_block: Block, count_reg: usize) {
        self.bytecode.push(OPTCODE::SimpleLoop { body_block: loop_block, count_reg });
        
    }
    pub fn define_while_loop(&mut self, loop_block: Block, conditional_block: Block, condition_reg: usize) {
        let block_length = loop_block.bytecode.len();
        for optcode in &conditional_block.bytecode {
            self.bytecode.push(optcode.clone());
        }
        self.bytecode.push(OPTCODE::JumpIfFalse {
            steps: block_length + 1,
            register: condition_reg,
        });
        for optcode in loop_block.bytecode {
            self.bytecode.push(optcode);
        }
        self.bytecode.push(OPTCODE::JumpBack {
            steps: block_length + &conditional_block.bytecode.len() + 2,
        });
    }
    pub fn define_variable(
        &mut self,
        id: usize,
        register: usize
    ) {
        self.bytecode.push(OPTCODE::DefineVar {
            id,
            register,
        });
    }
    pub fn define_object(
        &mut self,
        id: usize,
        register: usize
    ) {
        self.bytecode.push(OPTCODE::DefineObject {
            id,
            register,
        });
    }
    pub fn create_object(&mut self, field_names: Vec<String>, field_regs: Vec<usize>, target_reg: usize){

        self.bytecode.push(OPTCODE::CreateObject { field_names, field_regs, target_reg });
    }
    pub fn define_function(
        &mut self,
        body_block: Block,
        visibility: VISIBILITY,
        signature: FunctionSignature
    ) {
        self.bytecode.push(OPTCODE::DefineFunction {
            signature: signature,
            visibility: visibility,
            body_block,
        })
    }
    pub fn return_from_function(&mut self) {
        self.bytecode.push(OPTCODE::ReturnFromFunction);
    }
    
    pub fn assign_variable(&mut self, id: usize, register: usize) {
        self.bytecode.push(OPTCODE::AssignVar { id, register })
    }
    pub fn load_variable(&mut self, id: usize, register: usize) {
        self.bytecode.push(OPTCODE::LoadVar { id, register })
    }
    pub fn call_special_function(&mut self, function: SpecialFunctions, register: usize) {
        self.bytecode.push(OPTCODE::CallSpecialFunction { function, register });
    }
    pub fn add_blocks_bytecode(&mut self, block: Block){
        let mut other = block.bytecode;
        self.bytecode.append(&mut other);
    }
    pub fn get_object_field(&mut self, field_name: String, object_register: usize) {
        self.bytecode.push(OPTCODE::GetObjectField { field_name, object_register });
    }
    pub fn push_to_testing_stack(&mut self, duplicate_stackvalue: bool, register: usize) {
        self.bytecode.push(OPTCODE::PushToTestingStack{duplicate_stackvalue, register });
    }
}
