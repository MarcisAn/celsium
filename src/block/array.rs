use crate::bytecode::OPTCODE;

use super::Block;

impl Block {
    pub fn define_array(&mut self, init_values_count: usize, id: usize) {
        self.bytecode.push(OPTCODE::DefineArray { id, init_values_count })
    }

    pub fn load_from_array(&mut self, id: usize) {
        self.bytecode.push(OPTCODE::GET_FROM_ARRAY { id })
    }

    pub fn assign_to_array(&mut self, id: usize) {
        self.bytecode.push(OPTCODE::ASSIGN_AT_ARRAY_INDEX { id });
    }

    pub fn call_method_on_variable(method_name: String) {}

    pub fn get_array_length(&mut self, id: usize) {
        self.bytecode.push(OPTCODE::GET_ARRAY_LENGTH { id });
    }
    pub fn push_to_array(&mut self, id: usize) {
        self.bytecode.push(OPTCODE::PUSH_TO_ARRAY { id })
    }
}