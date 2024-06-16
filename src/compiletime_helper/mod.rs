use std::collections::LinkedList;

use crate::{
    bytecode::BINOP,
    module::{ FuncArg, Function },
    vm::vm::Variable,
    Scope,
    BUILTIN_TYPES,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CompileTimeVariable {
    pub id: usize,
    pub name: String,
    pub data_type: BUILTIN_TYPES,
    pub scope: Scope,
    pub is_exported: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompileTimeImport {
    pub name: String,
    pub origin: String,
    pub imported_into: String,
}

#[derive(Clone, Debug)]
pub struct CompileTimeArray {
    pub id: usize,
    pub name: String,
    pub data_type: BUILTIN_TYPES,
    pub length: usize,
    pub scope: Scope,
    pub is_exported: bool,
}

#[derive(Clone, Debug)]
pub struct CompileTimeFunction {
    pub id: usize,
    pub name: String,
    pub arguments: Vec<FuncArg>,
    pub scope: Scope,
    pub return_type: Option<BUILTIN_TYPES>,
    pub is_exported: bool,
}

#[derive(Clone, Debug)]
pub struct CompileTimeHelper {
    stack: LinkedList<BUILTIN_TYPES>,
    pub source_files: Vec<String>,
    pub source_file_paths: Vec<String>,
    pub current_file: usize,
    pub defined_functions: Vec<CompileTimeFunction>,
    pub defined_variables: Vec<CompileTimeVariable>,
    pub defined_arrays: Vec<CompileTimeArray>,
    definition_counter: usize,
    pub imports: Vec<CompileTimeImport>,
}

impl CompileTimeHelper {
    pub fn new(source_file: String, path: String) -> CompileTimeHelper {
        CompileTimeHelper {
            stack: LinkedList::new(),
            source_files: vec![source_file],
            source_file_paths: vec![path.clone()],
            current_file: 0,
            defined_functions: vec![],
            defined_variables: vec![],
            defined_arrays: vec![],
            definition_counter: 0,
            imports: vec![],
        }
    }
    pub fn change_module(&mut self, file_content: String, path: String) {
        self.source_files.push(file_content.clone());
        self.source_file_paths.push(path.clone());
        self.current_file += 1;
    }
    pub fn switch_to_prev_module(&mut self) {
        self.current_file -= 1;
    }
    pub fn import(&mut self, name: String, origin: String, imported_into: String) {
        self.imports.push(CompileTimeImport { name, origin, imported_into });
    }
    pub fn push(&mut self, pushable_type: BUILTIN_TYPES) {
        self.stack.push_back(pushable_type);
    }
    pub fn pop(&mut self) -> Option<BUILTIN_TYPES> {
        self.stack.pop_back()
    }
    pub fn def_function(
        &mut self,
        name: String,
        arguments: Vec<FuncArg>,
        scope: Scope,
        is_exported: bool
    ) -> usize {
        let return_type = self.pop();
        self.defined_functions.push(CompileTimeFunction {
            id: self.definition_counter,
            name: name,
            arguments: arguments,
            scope: scope,
            return_type: return_type,
            is_exported,
        });
        self.definition_counter += 1;
        return self.definition_counter - 1;
    }
    pub fn get_func_return_type(&mut self, id: usize) -> Option<Option<BUILTIN_TYPES>> {
        for func in self.defined_functions.clone() {
            if func.id == id {
                return Some(func.return_type);
            }
        }
        None
    }
    pub fn get_func_args(&mut self, id: usize) -> Option<Vec<FuncArg>> {
        for func in self.defined_functions.clone() {
            if func.id == id {
                return Some(func.arguments);
            }
        }
        None
    }
    pub fn def_var(
        &mut self,
        name: String,
        data_type: BUILTIN_TYPES,
        scope: Scope,
        is_exported: bool
    ) -> Result<usize, &str> {
        let to_be_defined = CompileTimeVariable {
            name,
            data_type,
            scope: scope.clone(),
            id: self.definition_counter,
            is_exported,
        };
        for def in &self.defined_variables {
            if def.name == to_be_defined.name && def.scope == to_be_defined.scope {
                return Err("already_defined");
            }
        }
        for import in &self.imports{
            if import.name == to_be_defined.name && import.imported_into == scope.module_path{
                return Err("already_imported");
            }
        }
        self.defined_variables.push(to_be_defined);
        self.definition_counter += 1;
        return Ok(self.definition_counter - 1);
    }
    pub fn get_var_type(&mut self, var_id: usize) -> Option<BUILTIN_TYPES> {
        for var in self.defined_variables.clone() {
            if var.id == var_id {
                return Some(var.data_type);
            }
        }
        None
    }
    pub fn def_array(
        &mut self,
        name: &str,
        data_type: BUILTIN_TYPES,
        initial_length: usize,
        scope: Scope,
        is_exported: bool
    ) -> usize {
        self.defined_arrays.push(CompileTimeArray {
            name: name.to_string(),
            data_type,
            length: initial_length,
            scope,
            id: self.definition_counter,
            is_exported,
        });

        self.definition_counter += 1;

        return self.definition_counter - 1;
    }

    pub fn get_array_type_and_length(&mut self, id: usize) -> Option<(BUILTIN_TYPES, usize)> {
        for array in &self.defined_arrays {
            if array.id == id {
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
            BUILTIN_TYPES::MAGIC_INT =>
                match b {
                    BUILTIN_TYPES::MAGIC_INT => Some(BUILTIN_TYPES::MAGIC_INT),
                    BUILTIN_TYPES::BOOL => {
                        return None;
                    }
                    BUILTIN_TYPES::STRING => Some(BUILTIN_TYPES::STRING),
                    BUILTIN_TYPES::OBJECT => {
                        return None;
                    }
                    BUILTIN_TYPES::FLOAT => Some(BUILTIN_TYPES::FLOAT),
                }
            BUILTIN_TYPES::BOOL => None,
            BUILTIN_TYPES::STRING =>
                match b {
                    BUILTIN_TYPES::MAGIC_INT => Some(BUILTIN_TYPES::STRING),
                    BUILTIN_TYPES::BOOL => {
                        return None;
                    }
                    BUILTIN_TYPES::STRING => Some(BUILTIN_TYPES::STRING),
                    BUILTIN_TYPES::OBJECT => {
                        return None;
                    }
                    BUILTIN_TYPES::FLOAT => Some(BUILTIN_TYPES::STRING),
                }
            BUILTIN_TYPES::OBJECT => None,
            BUILTIN_TYPES::FLOAT =>
                match b {
                    BUILTIN_TYPES::MAGIC_INT => Some(BUILTIN_TYPES::FLOAT),
                    BUILTIN_TYPES::BOOL => {
                        return None;
                    }
                    BUILTIN_TYPES::STRING => Some(BUILTIN_TYPES::STRING),
                    BUILTIN_TYPES::OBJECT => {
                        return None;
                    }
                    BUILTIN_TYPES::FLOAT => Some(BUILTIN_TYPES::FLOAT),
                }
        };
        return result;
    }
    fn subtract(&mut self) -> Option<BUILTIN_TYPES> {
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();
        let result = match a {
            BUILTIN_TYPES::MAGIC_INT =>
                match b {
                    BUILTIN_TYPES::MAGIC_INT => Some(BUILTIN_TYPES::MAGIC_INT),
                    BUILTIN_TYPES::BOOL => {
                        return None;
                    }
                    BUILTIN_TYPES::STRING => {
                        return None;
                    }
                    BUILTIN_TYPES::OBJECT => {
                        return None;
                    }
                    BUILTIN_TYPES::FLOAT => Some(BUILTIN_TYPES::FLOAT),
                }
            BUILTIN_TYPES::BOOL => None,
            BUILTIN_TYPES::STRING => None,
            BUILTIN_TYPES::OBJECT => None,
            BUILTIN_TYPES::FLOAT =>
                match b {
                    BUILTIN_TYPES::MAGIC_INT => Some(BUILTIN_TYPES::FLOAT),
                    BUILTIN_TYPES::BOOL => {
                        return None;
                    }
                    BUILTIN_TYPES::STRING => {
                        return None;
                    }
                    BUILTIN_TYPES::OBJECT => {
                        return None;
                    }
                    BUILTIN_TYPES::FLOAT => Some(BUILTIN_TYPES::FLOAT),
                }
        };
        return result;
    }
    fn compare(&mut self) -> Option<BUILTIN_TYPES> {
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();
        let result = match a {
            BUILTIN_TYPES::MAGIC_INT =>
                match b {
                    BUILTIN_TYPES::MAGIC_INT => Some(BUILTIN_TYPES::BOOL),
                    BUILTIN_TYPES::BOOL => {
                        return None;
                    }
                    BUILTIN_TYPES::STRING => {
                        return None;
                    }
                    BUILTIN_TYPES::OBJECT => {
                        return None;
                    }
                    BUILTIN_TYPES::FLOAT => Some(BUILTIN_TYPES::BOOL),
                }
            BUILTIN_TYPES::BOOL =>
                match b {
                    BUILTIN_TYPES::MAGIC_INT => {
                        return None;
                    }
                    BUILTIN_TYPES::BOOL => Some(BUILTIN_TYPES::BOOL),
                    BUILTIN_TYPES::STRING => {
                        return None;
                    }
                    BUILTIN_TYPES::OBJECT => {
                        return None;
                    }
                    BUILTIN_TYPES::FLOAT => {
                        return None;
                    }
                }
            BUILTIN_TYPES::STRING => None,
            BUILTIN_TYPES::OBJECT => None,
            BUILTIN_TYPES::FLOAT =>
                match b {
                    BUILTIN_TYPES::MAGIC_INT => Some(BUILTIN_TYPES::BOOL),
                    BUILTIN_TYPES::BOOL => {
                        return None;
                    }
                    BUILTIN_TYPES::STRING => {
                        return None;
                    }
                    BUILTIN_TYPES::OBJECT => {
                        return None;
                    }
                    BUILTIN_TYPES::FLOAT => Some(BUILTIN_TYPES::BOOL),
                }
        };
        return result;
    }
}
