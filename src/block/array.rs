use crate::bytecode::OPTCODE;

use super::Block;

impl Block {
    pub fn define_array(&mut self, init_values: Vec<usize>, id: usize) {
        self.bytecode.push(OPTCODE::DefineArray { id, init_values })
    }

    pub fn load_from_array(&mut self, id: usize, register: usize) {
        self.bytecode.push(OPTCODE::GetFromArray { id, register })
    }

    pub fn assign_to_array(&mut self, id: usize, value_reg: usize, index_reg: usize) {
        self.bytecode.push(OPTCODE::AssignAtArrayIndex { id, value_reg, index_reg });
    }

    pub fn get_array_length(&mut self, id: usize, register: usize) {
        self.bytecode.push(OPTCODE::GetArrayLength { id, register });
    }
    pub fn push_to_array(&mut self, id: usize, register: usize) {
        self.bytecode.push(OPTCODE::PushToArray { id, register })
    }
}