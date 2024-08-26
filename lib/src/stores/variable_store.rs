use std::collections::HashMap;

use crate::{ BuiltinTypes, ObjectFieldType, Scope };

pub struct VariableStore {
    variables: Vec<CompileTimeVariable>,
    objects: Vec<CompileTimeObject>,
    id_counter: usize,
    object_counter: usize
}

impl VariableStore {
    pub fn new() -> VariableStore {
        VariableStore { variables: vec![], id_counter: 0, objects: vec![], object_counter: 0 }
    }
    pub fn define_variable(&mut self, name: String, scope: Scope, data_type: BuiltinTypes) -> usize{
        self.variables.push(CompileTimeVariable {
            id: self.id_counter,
            name: name,
            data_type: data_type,
            scope: scope,
        });
        self.id_counter += 1;
        return self.id_counter - 1;
    }
    pub fn define_object(&mut self, name: String, scope: Scope, object_fields: Vec<ObjectFieldType>) -> Vec<usize> {
        let object_type = BuiltinTypes::Object { fields: object_fields.clone()  };
        self.objects.push(CompileTimeObject { field_id_start: self.id_counter, name: name, data_type: object_type, scope: scope.clone() });
        let mut ids: Vec<usize> = vec![];
        for field in object_fields{
            let id = self.define_variable("".to_string(), scope.clone(), field.data_type);
            ids.push(id);
        }
        return ids;
    }
    pub fn get_object_field(&mut self, name: String, scope: Scope, field_name:String) -> Option<usize>{
        for object in &self.objects{
            if object.name == name && object.scope == scope{
                let fields = match &object.data_type {
                    BuiltinTypes::Object { fields } => fields,
                    _ => panic!()   
                };
                for (index, field) in fields.into_iter().enumerate(){
                    if field.name == field_name{
                        return Some(object.field_id_start + index);
                    }
                }
            }
        }
        None
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

#[derive(Debug, Clone, PartialEq)]
pub struct CompileTimeObject {
    pub name: String,
    pub data_type: BuiltinTypes,
    pub scope: crate::Scope,
    pub field_id_start: usize
}