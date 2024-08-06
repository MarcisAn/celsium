use crate::bytecode::OPTCODE;

use super::Block;

impl Block {
    pub fn define_array(&mut self, init_values_count: usize, id: usize) {
        self.bytecode.push(OPTCODE::DefineArray { id, init_values_count })
    }

    pub fn load_from_array(&mut self, id: usize) {
        self.bytecode.push(OPTCODE::GetFromArray { id })
    }

    pub fn assign_to_array(&mut self, id: usize) {
        self.bytecode.push(OPTCODE::AssignAtArrayIndex { id });
    }


    pub fn get_array_length(&mut self, id: usize) {
        self.bytecode.push(OPTCODE::GettArrayLength { id });
    }
    pub fn push_to_array(&mut self, id: usize) {
        self.bytecode.push(OPTCODE::PushToArray { id })
    }
}