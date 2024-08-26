use std::collections::LinkedList;
mod mathops;
use crate::{bytecode::BINOP, BuiltinTypes};

pub struct TypeStack {
    stack: LinkedList<BuiltinTypes>,
}
impl TypeStack {
    pub fn new() -> TypeStack {
        TypeStack {
            stack: LinkedList::new(),
        }
    }
    pub fn push(&mut self, pushable_type: BuiltinTypes) {
        self.stack.push_back(pushable_type);
    }
    pub fn pop(&mut self) -> Option<BuiltinTypes> {
        self.stack.pop_back()
    }
    pub fn pop_multiple(&mut self, count: usize) -> Option<Vec<BuiltinTypes>>{
        let mut result: Vec<BuiltinTypes> = vec![];
        let mut counter = 0;
        while counter < count{
            let popped = self.stack.pop_back();
            if popped.is_some(){
                result.push(popped.unwrap());
            }
            else{
                return None;
            }
            counter += 1;
        }
        return Some(result);

    }
    pub fn peek(self) -> Option<BuiltinTypes>{
        self.stack.back().cloned()
    }
    pub fn binop(&mut self, binop: BINOP) -> Option<BuiltinTypes> {
        /*
        Subtraction, multiplication, division and getting remainder
        are identical in the way types ar handled
        */
        let result_type = match binop {
            BINOP::Add => self.add(),
            BINOP::Subtract => self.subtract(),
            BINOP::Multiply => self.subtract(),
            BINOP::Divide => self.subtract(),
            BINOP::Remainder => self.subtract(),
            BINOP::LessThan => self.compare(),
            BINOP::LargerThan => self.compare(),
            BINOP::LessOrEq => self.compare(),
            BINOP::LargerOrEq => self.compare(),
            BINOP::NotEq => self.compare(),
            BINOP::Eq => self.compare(),
            BINOP::And => self.compare(),
            BINOP::Or => self.compare(),
            BINOP::Xor => self.compare(),
        };
        if result_type.is_some() {
            self.stack.push_back(result_type.clone().unwrap());
            return result_type;
        } else {
            return None;
        }
    }
}
