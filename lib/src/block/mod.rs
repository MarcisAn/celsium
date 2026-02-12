use crate::{ Scope };
use crate::{ BINOP, OPTCODE };
mod array;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Block {
    pub bytecode: Vec<OPTCODE>,
    pub scope: Scope,
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TextSpan {
    pub line: usize,
    pub col_start: usize,
    pub length: usize,
}

impl Block {
    pub fn new(scope: Scope) -> Block {
        Block { bytecode: vec![], scope }
    }
    pub fn load_int(&mut self, value: i64) {
        self.bytecode.push(OPTCODE::LoadInt { value });
    }
    pub fn load_bool(&mut self, value: bool) {
        self.bytecode.push(OPTCODE::LoadBool { value });
    }
    pub fn load_string(&mut self, value: &str) {
        self.bytecode.push(OPTCODE::LoadString { value: value.to_string() });
    }
    pub fn load_float(&mut self, value: f64) {
        self.bytecode.push(OPTCODE::LoadFloat { value });
    }
    pub fn create_array(&mut self, number_of_elements: usize) {
        self.bytecode.push(OPTCODE::CreateArray { init_values_count: number_of_elements });
    }
    pub fn binop(&mut self, operator: BINOP, span: TextSpan) {
        self.bytecode.push(match operator {
            BINOP::Add => OPTCODE::Add { span: span },
            BINOP::Subtract => OPTCODE::Subtract,
            BINOP::Multiply => OPTCODE::Multiply,
            BINOP::Divide => OPTCODE::Divide,
            BINOP::Remainder => OPTCODE::Remainder,
            BINOP::LessThan => OPTCODE::LessThan { span: span },
            BINOP::LargerThan => OPTCODE::LargerThan,
            BINOP::LessOrEq => OPTCODE::LessOrEq,
            BINOP::LargerOrEq => OPTCODE::LargerOrEq,
            BINOP::NotEq => OPTCODE::NotEq,
            BINOP::Eq => OPTCODE::Eq,
            BINOP::And => OPTCODE::And,
            BINOP::Or => OPTCODE::Or,
            BINOP::Xor => OPTCODE::Xor,
            BINOP::Not => OPTCODE::Not,
        });
    }
    pub fn define_if_block(
        &mut self,
        block: Block,
        jmp_target_line: usize,
        jmp_target_column: usize
    ) {
        let block_length = block.bytecode.len();
        self.bytecode.push(OPTCODE::JumpIfFalse {
            steps: block_length,
            jump_target_column: jmp_target_column,
            jump_target_line: jmp_target_line,
            is_skipable: false,
        });
        for optcode in block.bytecode {
            self.bytecode.push(optcode);
        }
    }
    pub fn define_if_else_block(
        &mut self,
        if_block: Block,
        else_block: Block,
        jmp_target_line: usize,
        jmp_target_column: usize
    ) {
        //println!("{:?}", else_block);
        let if_block_length = if_block.bytecode.len();
        let else_block_length = else_block.bytecode.len();
        self.bytecode.push(OPTCODE::JumpIfFalse {
            steps: if_block_length + 1,
            jump_target_column: jmp_target_column,
            jump_target_line: jmp_target_line,
            is_skipable: false,
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
    pub fn define_simple_loop(&mut self, loop_block: Block) {
        self.bytecode.push(OPTCODE::SimpleLoop { body_block: loop_block });
    }
    pub fn define_while_loop(
        &mut self,
        loop_block: Block,
        conditional_block: Block,
        jmp_target_line: usize,
        jmp_target_column: usize
    ) {
        let block_length = loop_block.bytecode.len();
        for optcode in &conditional_block.bytecode {
            self.bytecode.push(optcode.clone());
        }
        self.bytecode.push(OPTCODE::JumpIfFalse {
            steps: block_length + 1,
            jump_target_column: jmp_target_column,
            jump_target_line: jmp_target_line,
            is_skipable: true,
        });
        for optcode in loop_block.bytecode {
            self.bytecode.push(optcode);
        }
        self.bytecode.push(OPTCODE::JumpBack {
            steps: block_length + &conditional_block.bytecode.len() + 2,
        });
    }
    pub fn define_variable(&mut self, id: usize) {
        self.bytecode.push(OPTCODE::DefineVar {
            id,
        });
    }
    pub fn define_object(&mut self, id: usize) {
        self.bytecode.push(OPTCODE::DefineObject {
            id,
        });
    }
    pub fn create_object(&mut self, field_names: Vec<String>) {
        self.bytecode.push(OPTCODE::CreateObject { field_names });
    }
    pub fn return_from_function(&mut self) {
        self.bytecode.push(OPTCODE::Return);
    }

    pub fn assign_variable(&mut self, id: usize) {
        self.bytecode.push(OPTCODE::AssignVar { id })
    }
    pub fn load_variable(&mut self, id: usize, span: TextSpan) {
        self.bytecode.push(OPTCODE::LoadVar { id, span })
    }
    pub fn call_special_function(&mut self, function: String) {
        self.bytecode.push(OPTCODE::CallSpecialFunction { function });
    }
    pub fn add_blocks_bytecode(&mut self, block: Block) {
        let mut other = block.bytecode;
        self.bytecode.append(&mut other);
    }
    pub fn get_object_field(&mut self, field_name: String) {
        self.bytecode.push(OPTCODE::GetObjectField { field_name });
    }
    pub fn set_object_field(&mut self, id: usize, field_name: String) {
        self.bytecode.push(OPTCODE::SetObjectField { id, field_name });
    }
    pub fn push_to_testing_stack(&mut self, duplicate_stackvalue: bool) {
        self.bytecode.push(OPTCODE::PushToTestingStack { duplicate_stackvalue });
    }
    pub fn break_loop(&mut self, span: TextSpan) {
        self.bytecode.push(OPTCODE::Break { span: span });
    }
    pub fn continue_loop(&mut self, span: TextSpan) {
        self.bytecode.push(OPTCODE::Continue { span: span });
    }
}
