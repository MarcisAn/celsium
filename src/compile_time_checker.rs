use std::collections::LinkedList;


use crate::{bytecode::BINOP, BUILTIN_TYPES};

pub struct CompileTimeChecker {
    stack: LinkedList<BUILTIN_TYPES>,
    pub source_files: Vec<String>,
    pub source_file_paths: Vec<String>,
    pub current_file: usize
}

impl CompileTimeChecker {
    pub fn new(source_file: String, path: String) -> CompileTimeChecker {
        CompileTimeChecker { stack: LinkedList::new(), source_files: vec![source_file], source_file_paths: vec![path], current_file: 0 }
    }
    pub fn push(&mut self, pushable_type: BUILTIN_TYPES){
        self.stack.push_back(pushable_type);
    }
    pub fn pop(&mut self) -> Option<BUILTIN_TYPES> {
        self.stack.pop_back()
    }
    pub fn binop(&mut self, binop: BINOP) -> Option<BUILTIN_TYPES> {
        /*
        Subtraction, multiplication, division and getting remainder
        are identical in the way types ar handled
        */
        match binop {
            BINOP::ADD => self.add(),
            BINOP::SUBTRACT => self.subtract(),
            BINOP::MULTIPLY => self.subtract(),
            BINOP::DIVIDE => self.subtract(),
            BINOP::REMAINDER => self.subtract(),
            BINOP::LessThan => self.compare(),
            BINOP::LargerThan => self.compare(),
            BINOP::LessOrEq => self.compare(),
            BINOP::LargerOrEq => self.compare(),
            BINOP::NotEq => self.compare(),
            BINOP::EQ => self.compare(),
            BINOP::AND => self.compare(),
            BINOP::OR => self.compare(),
            BINOP::XOR => self.compare(),
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
    fn subtract(&mut self) -> Option<BUILTIN_TYPES> {
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();
        let result = match a {
            BUILTIN_TYPES::MAGIC_INT => match b {
                BUILTIN_TYPES::MAGIC_INT => Some(BUILTIN_TYPES::MAGIC_INT),
                BUILTIN_TYPES::BOOL => return None,
                BUILTIN_TYPES::STRING => return None,
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
                BUILTIN_TYPES::MAGIC_INT => return None,
                BUILTIN_TYPES::BOOL => return None,
                BUILTIN_TYPES::STRING => return None,
                BUILTIN_TYPES::OBJECT => return None,
                BUILTIN_TYPES::FLOAT => return None,
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
                BUILTIN_TYPES::STRING => return None,
                BUILTIN_TYPES::OBJECT => return None,
                BUILTIN_TYPES::FLOAT => Some(BUILTIN_TYPES::FLOAT),
            },
        };
        self.stack.push_back(result.clone().unwrap());
        return result;
    }
    fn compare(&mut self) -> Option<BUILTIN_TYPES> {
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();
        let result = match a {
            BUILTIN_TYPES::MAGIC_INT => match b {
                BUILTIN_TYPES::MAGIC_INT => Some(BUILTIN_TYPES::BOOL),
                BUILTIN_TYPES::BOOL => return None,
                BUILTIN_TYPES::STRING => return None,
                BUILTIN_TYPES::OBJECT => return None,
                BUILTIN_TYPES::FLOAT => Some(BUILTIN_TYPES::BOOL),
            },
            BUILTIN_TYPES::BOOL => match b {
                BUILTIN_TYPES::MAGIC_INT => return None,
                BUILTIN_TYPES::BOOL => Some(BUILTIN_TYPES::BOOL),
                BUILTIN_TYPES::STRING => return None,
                BUILTIN_TYPES::OBJECT => return None,
                BUILTIN_TYPES::FLOAT => return None,
            },
            BUILTIN_TYPES::STRING => match b {
                BUILTIN_TYPES::MAGIC_INT => return None,
                BUILTIN_TYPES::BOOL => return None,
                BUILTIN_TYPES::STRING => return None,
                BUILTIN_TYPES::OBJECT => return None,
                BUILTIN_TYPES::FLOAT => return None,
            },
            BUILTIN_TYPES::OBJECT => match b {
                BUILTIN_TYPES::MAGIC_INT => return None,
                BUILTIN_TYPES::BOOL => return None,
                BUILTIN_TYPES::STRING => return None,
                BUILTIN_TYPES::OBJECT => return None,
                BUILTIN_TYPES::FLOAT => return None,
            },
            BUILTIN_TYPES::FLOAT => match b {
                BUILTIN_TYPES::MAGIC_INT => Some(BUILTIN_TYPES::BOOL),
                BUILTIN_TYPES::BOOL => return None,
                BUILTIN_TYPES::STRING => return None,
                BUILTIN_TYPES::OBJECT => return None,
                BUILTIN_TYPES::FLOAT => Some(BUILTIN_TYPES::BOOL),
            },
        };
        self.stack.push_back(result.clone().unwrap());
        return result;
    }
    
}