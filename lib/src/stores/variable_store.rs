use crate::{ BuiltinTypes, Scope };

pub struct VariableStore {
    variables: Vec<CompileTimeVariable>,
    id_counter: usize,
}

impl VariableStore {
    pub fn define_variable(&mut self, name: String, scope: Scope, data_type: BuiltinTypes) {
        self.variables.push(CompileTimeVariable {
            id: self.id_counter,
            name: name,
            data_type: data_type,
            scope: scope,
        })
    }
    pub fn find_variable(&mut self, name: String, scope: Scope) -> Option<&CompileTimeVariable> {
        for var in &self.variables{
            if var.name == name && var.scope == scope{
                return Some(var);
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompileTimeVariable {
    pub id: usize,
    pub name: String,
    pub data_type: BuiltinTypes,
    pub scope: crate::Scope,
}
