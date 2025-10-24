use crate::bytecode::OPTCODE;

use super::Block;

impl Block {
    pub fn load_from_array(&mut self) {
        self.bytecode.push(OPTCODE::GetFromArray {})
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