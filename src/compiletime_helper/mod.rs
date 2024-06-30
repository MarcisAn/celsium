use std::collections::LinkedList;
use crate::{ bytecode::BINOP, module::FuncArg, ObjectFieldType, Scope, BuiltinTypes };

#[derive(Debug, Clone, PartialEq)]
pub struct CompileTimeVariable {
    pub id: usize,
    pub name: String,
    pub data_type: BuiltinTypes,
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
    pub data_type: BuiltinTypes,
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
    pub return_type: Option<BuiltinTypes>,
    pub is_exported: bool,
}

#[derive(Debug, Clone)]
pub struct ObjectDefinitionDefinition {
    pub module_defined_in: String,
    pub name: String,
    pub fields: Vec<ObjectFieldType>,
}

#[derive(Debug, Clone)]
pub struct CompileTimeObject {
    pub id: usize,
    pub name: String,
    pub data_type: BuiltinTypes,
    pub scope: Scope,
    pub is_exported: bool,
}

#[derive(Clone, Debug)]
pub struct CompileTimeHelper {
    stack: LinkedList<BuiltinTypes>,
    pub source_files: Vec<String>,
    pub source_file_paths: Vec<String>,
    pub current_file: usize,
    pub defined_functions: Vec<CompileTimeFunction>,
    pub defined_variables: Vec<CompileTimeVariable>,
    pub defined_arrays: Vec<CompileTimeArray>,
    pub defined_object_definitions: Vec<ObjectDefinitionDefinition>,
    pub defined_objects: Vec<CompileTimeObject>,
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
            defined_object_definitions: vec![],
            defined_objects: vec![],
        }
    }
    pub fn define_struct(&mut self, name: String, fields: Vec<ObjectFieldType>) {
        self.defined_object_definitions.push(ObjectDefinitionDefinition {
            module_defined_in: self.source_file_paths[self.current_file].clone(),
            name,
            fields,
        });
    }

    pub fn struct_exists(&mut self, name: &str) -> Option<ObjectDefinitionDefinition> {
        for object in &self.defined_object_definitions {
            if
                object.name == name &&
                object.module_defined_in == self.source_file_paths[self.current_file]
            {
                return Some(object.clone());
            }
        }
        return None;
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
    pub fn push(&mut self, pushable_type: BuiltinTypes) {
        self.stack.push_back(pushable_type);
    }
    pub fn pop(&mut self) -> Option<BuiltinTypes> {
        self.stack.pop_back()
    }
    pub fn def_function(
        &mut self,
        name: String,
        arguments: Vec<FuncArg>,
        scope: Scope,
        is_exported: bool,
        return_type: Option<BuiltinTypes>
    ) -> usize {
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
    pub fn get_func_return_type(&mut self, id: usize) -> Option<Option<BuiltinTypes>> {
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
        data_type: BuiltinTypes,
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
        for def in &self.defined_objects {
            if def.name == to_be_defined.name && def.scope == to_be_defined.scope {
                return Err("already_defined");
            }
        }
        for def in &self.defined_arrays {
            if def.name == to_be_defined.name && def.scope == to_be_defined.scope {
                return Err("already_defined");
            }
        }
        for import in &self.imports {
            if import.name == to_be_defined.name && import.imported_into == scope.module_path {
                return Err("already_imported");
            }
        }
        self.defined_variables.push(to_be_defined);
        self.definition_counter += 1;
        return Ok(self.definition_counter - 1);
    }
    pub fn def_object(
        &mut self,
        name: String,
        scope: Scope,
        is_exported: bool,
        fields: Vec<ObjectFieldType>
    ) -> Result<usize, &str> {
        let object: CompileTimeObject = CompileTimeObject {
            data_type: BuiltinTypes::Object { fields: fields },
            name,
            id: self.definition_counter,
            scope: scope.clone(),
            is_exported,
        };
        for def in &self.defined_variables {
            if def.name == object.name && def.scope == object.scope {
                return Err("already_defined");
            }
        }
        for def in &self.defined_objects {
            if def.name == object.name && def.scope == object.scope {
                return Err("already_defined");
            }
        }
        for def in &self.defined_arrays {
            if def.name == object.name && def.scope == object.scope {
                return Err("already_defined");
            }
        }
        for import in &self.imports {
            if import.name == object.name && import.imported_into == scope.module_path {
                return Err("already_imported");
            }
        }
        self.defined_objects.push(object);
        self.definition_counter += 1;
        return Ok(self.definition_counter - 1);
    }
    pub fn get_object_if_exists(&mut self, name: &str) -> Option<CompileTimeObject> {
        for object in &self.defined_objects {
            if object.name == name {
                return Some(object.clone());
            }
        }
        None
    }
    pub fn get_var_type(&mut self, var_id: usize) -> Option<BuiltinTypes> {
        for var in self.defined_variables.clone() {
            if var.id == var_id {
                return Some(var.data_type);
            }
        }
        for var in self.defined_objects.clone() {
            if var.id == var_id {
                return Some(var.data_type);
            }
        }
        for var in self.defined_arrays.clone() {
            if var.id == var_id {
                return Some(BuiltinTypes::Array { element_type: Box::new(var.data_type) });
            }
        }
        None
    }
    pub fn def_array(
        &mut self,
        name: &str,
        data_type: BuiltinTypes,
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

    pub fn get_array_type_and_length(&mut self, id: usize) -> Option<(BuiltinTypes, usize)> {
        for array in &self.defined_arrays {
            if array.id == id {
                return Some((array.data_type.clone(), array.length));
            }
        }
        None
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
    fn add(&mut self) -> Option<BuiltinTypes> {
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();
        let result = match a {
            BuiltinTypes::MagicInt =>
                match b {
                    BuiltinTypes::MagicInt => Some(BuiltinTypes::MagicInt),
                    BuiltinTypes::Bool => {
                        return None;
                    }
                    BuiltinTypes::String => Some(BuiltinTypes::String),
                    BuiltinTypes::Object { fields: _ } => {
                        return None;
                    }
                    BuiltinTypes::Float => Some(BuiltinTypes::Float),
                    BuiltinTypes::Array { element_type } => None,
                }
            BuiltinTypes::Bool => None,
            BuiltinTypes::String =>
                match b {
                    BuiltinTypes::MagicInt => Some(BuiltinTypes::String),
                    BuiltinTypes::Bool => {
                        return None;
                    }
                    BuiltinTypes::String => Some(BuiltinTypes::String),
                    BuiltinTypes::Object { fields: _ } => {
                        return None;
                    }
                    BuiltinTypes::Float => Some(BuiltinTypes::String),
                    BuiltinTypes::Array { element_type } => None,
                }
            BuiltinTypes::Object { fields: _ } => None,
            BuiltinTypes::Float =>
                match b {
                    BuiltinTypes::MagicInt => Some(BuiltinTypes::Float),
                    BuiltinTypes::Bool => {
                        return None;
                    }
                    BuiltinTypes::String => Some(BuiltinTypes::String),
                    BuiltinTypes::Object { fields: _ } => {
                        return None;
                    }
                    BuiltinTypes::Float => Some(BuiltinTypes::Float),
                    BuiltinTypes::Array { element_type } => None,
                }
            BuiltinTypes::Array { element_type } => None,
        };
        return result;
    }
    fn subtract(&mut self) -> Option<BuiltinTypes> {
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();
        let result = match a {
            BuiltinTypes::MagicInt =>
                match b {
                    BuiltinTypes::MagicInt => Some(BuiltinTypes::MagicInt),
                    BuiltinTypes::Bool => {
                        return None;
                    }
                    BuiltinTypes::String => {
                        return None;
                    }
                    BuiltinTypes::Object { fields: _ } => {
                        return None;
                    }
                    BuiltinTypes::Float => Some(BuiltinTypes::Float),
                    BuiltinTypes::Array { element_type } => None,
                }
            BuiltinTypes::Bool => None,
            BuiltinTypes::String => None,
            BuiltinTypes::Object { fields: _ } => None,
            BuiltinTypes::Array { element_type } => None,

            BuiltinTypes::Float =>
                match b {
                    BuiltinTypes::MagicInt => Some(BuiltinTypes::Float),
                    BuiltinTypes::Bool => {
                        return None;
                    }
                    BuiltinTypes::String => {
                        return None;
                    }
                    BuiltinTypes::Object { fields: _ } => {
                        return None;
                    }
                    BuiltinTypes::Float => Some(BuiltinTypes::Float),
                    BuiltinTypes::Array { element_type } => None,
                }
        };
        return result;
    }
    fn compare(&mut self) -> Option<BuiltinTypes> {
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();
        let result = match a {
            BuiltinTypes::MagicInt =>
                match b {
                    BuiltinTypes::MagicInt => Some(BuiltinTypes::Bool),
                    BuiltinTypes::Bool => {
                        return None;
                    }
                    BuiltinTypes::String => {
                        return None;
                    }
                    BuiltinTypes::Object { fields: _ } => {
                        return None;
                    }
                    BuiltinTypes::Float => Some(BuiltinTypes::Bool),
                    BuiltinTypes::Array { element_type } => None,
                }
            BuiltinTypes::Bool =>
                match b {
                    BuiltinTypes::MagicInt => {
                        return None;
                    }
                    BuiltinTypes::Bool => Some(BuiltinTypes::Bool),
                    BuiltinTypes::String => {
                        return None;
                    }
                    BuiltinTypes::Object { fields: _ } => {
                        return None;
                    }
                    BuiltinTypes::Float => {
                        return None;
                    }
                    BuiltinTypes::Array { element_type } => None,
                }
            BuiltinTypes::String => None,
            BuiltinTypes::Object { fields: _ } => None,
            BuiltinTypes::Float =>
                match b {
                    BuiltinTypes::MagicInt => Some(BuiltinTypes::Bool),
                    BuiltinTypes::Bool => {
                        return None;
                    }
                    BuiltinTypes::String => {
                        return None;
                    }
                    BuiltinTypes::Object { fields: _ } => {
                        return None;
                    }
                    BuiltinTypes::Float => Some(BuiltinTypes::Bool),
                    BuiltinTypes::Array { element_type } => None,
                }
            BuiltinTypes::Array { element_type } => None,
        };
        return result;
    }
}
