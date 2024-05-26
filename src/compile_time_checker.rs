use std::collections::LinkedList;

use crate::{bytecode::BINOP, module::Function, vm::vm::Variable, BUILTIN_TYPES};

#[derive(Clone, Debug)]
pub struct CompileTimeVariable {
    name: String,
    data_type: BUILTIN_TYPES
} 

#[derive(Clone, Debug)]
pub struct CompileTimeArray {
    name: String,
    data_type: BUILTIN_TYPES,
    length: usize
} 

pub struct CompileTimeChecker {
    stack: LinkedList<BUILTIN_TYPES>,
    pub source_files: Vec<String>,
    pub source_file_paths: Vec<String>,
    pub current_file: usize,
    pub defined_functions: Vec<Function>,
    pub defined_variables: Vec<CompileTimeVariable>,
    pub defined_arrays: Vec<CompileTimeArray>,

}

impl CompileTimeChecker {
    pub fn new(source_file: String, path: String) -> CompileTimeChecker {
        CompileTimeChecker {
            stack: LinkedList::new(),
            source_files: vec![source_file],
            source_file_paths: vec![path],
            current_file: 0,
            defined_functions: vec![],
            defined_variables: vec![],
            defined_arrays: vec![]
        }
    }
    pub fn push(&mut self, pushable_type: BUILTIN_TYPES) {
        self.stack.push_back(pushable_type);
    }
    pub fn pop(&mut self) -> Option<BUILTIN_TYPES> {
        self.stack.pop_back()
    }
    pub fn def_var(&mut self, name:String, data_type: BUILTIN_TYPES ) {
        self.defined_variables.push(CompileTimeVariable { name, data_type });
    }
    pub fn def_array(&mut self, name: &str, data_type: BUILTIN_TYPES, initial_length: usize ) {
        self.defined_arrays.push(CompileTimeArray { name: name.to_string(), data_type, length: initial_length  });
    }
    pub fn check_var(&mut self, name: &str) -> Option<BUILTIN_TYPES>{
        for var in &self.defined_variables{
            if var.name == name{
                return Some(var.data_type.clone());
            }
        }
        None
    }
    pub fn check_array_type_and_length(&mut self, name: &str) -> Option<(BUILTIN_TYPES, usize)>{
        for array in &self.defined_arrays{
            if array.name == name{
                return Some((array.data_type.clone(), array.length));
            }
        }
        None
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
