use std::{collections::LinkedList, io::Error};

use num::Float;

use crate::{bytecode::BINOP, BUILTIN_TYPES};

pub struct TypeStack {
    stack: LinkedList<BUILTIN_TYPES>
}

impl TypeStack {
    pub fn new() -> TypeStack {
        TypeStack { stack: LinkedList::new() }
    }
    pub fn push(&mut self, pushable_type: BUILTIN_TYPES){
        self.stack.push_back(pushable_type);
    }
    pub fn binop(&mut self, binop: BINOP) -> Option<BUILTIN_TYPES> {
        match binop {
            BINOP::ADD => self.add(),
            BINOP::SUBTRACT => todo!(),
            BINOP::MULTIPLY => todo!(),
            BINOP::DIVIDE => todo!(),
            BINOP::REMAINDER => todo!(),
            BINOP::LessThan => todo!(),
            BINOP::LargerThan => todo!(),
            BINOP::LessOrEq => todo!(),
            BINOP::LargerOrEq => todo!(),
            BINOP::NotEq => todo!(),
            BINOP::EQ => todo!(),
            BINOP::AND => todo!(),
            BINOP::OR => todo!(),
            BINOP::XOR => todo!(),
        }
    }
    fn add(&mut self) -> Option<BUILTIN_TYPES> {
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();
        let result = match a {
            BUILTIN_TYPES::MAGIC_INT => match b {
                BUILTIN_TYPES::MAGIC_INT => Some(BUILTIN_TYPES::MAGIC_INT),
                BUILTIN_TYPES::BOOL => return None,
                BUILTIN_TYPES::STRING => Some(BUILTIN_TYPES::STRING),
                BUILTIN_TYPES::OBJECT => return None,
                BUILTIN_TYPES::FLOAT => Some(BUILTIN_TYPES::FLOAT),
            },
            BUILTIN_TYPES::BOOL => match b {
                BUILTIN_TYPES::MAGIC_INT => return None,
                BUILTIN_TYPES::BOOL => return None,
                BUILTIN_TYPES::STRING => return None,
                BUILTIN_TYPES::OBJECT => return None,
                BUILTIN_TYPES::FLOAT => return None,
            },
            BUILTIN_TYPES::STRING => match b {
                BUILTIN_TYPES::MAGIC_INT => Some(BUILTIN_TYPES::STRING),
                BUILTIN_TYPES::BOOL => return None,
                BUILTIN_TYPES::STRING => Some(BUILTIN_TYPES::STRING),
                BUILTIN_TYPES::OBJECT => return None,
                BUILTIN_TYPES::FLOAT => Some(BUILTIN_TYPES::STRING),
            },
            BUILTIN_TYPES::OBJECT => match b {
                BUILTIN_TYPES::MAGIC_INT => return None,
                BUILTIN_TYPES::BOOL => return None,
                BUILTIN_TYPES::STRING => return None,
                BUILTIN_TYPES::OBJECT => return None,
                BUILTIN_TYPES::FLOAT => return None,
            },
            BUILTIN_TYPES::FLOAT => match b {
                BUILTIN_TYPES::MAGIC_INT => Some(BUILTIN_TYPES::FLOAT),
                BUILTIN_TYPES::BOOL => return None,
                BUILTIN_TYPES::STRING => Some(BUILTIN_TYPES::STRING),
                BUILTIN_TYPES::OBJECT => return None,
                BUILTIN_TYPES::FLOAT => Some(BUILTIN_TYPES::FLOAT),
            },
        };
        self.stack.push_back(result.clone().unwrap());
        return result;
    }
}