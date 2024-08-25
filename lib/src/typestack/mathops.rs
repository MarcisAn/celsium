use crate::BuiltinTypes;

use super::TypeStack;

impl TypeStack{
    pub(super) fn add(&mut self) -> Option<BuiltinTypes> {
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();
        let result = match a {
            BuiltinTypes::Int =>
                match b {
                    BuiltinTypes::Int => Some(BuiltinTypes::Int),
                    BuiltinTypes::Bool => {
                        return None;
                    }
                    BuiltinTypes::String => Some(BuiltinTypes::String),
                    BuiltinTypes::Object { fields: _ } => {
                        return None;
                    }
                    BuiltinTypes::Float => Some(BuiltinTypes::Float),
                    BuiltinTypes::Array { element_type: _ } => None,
                }
            BuiltinTypes::Bool => None,
            BuiltinTypes::String =>
                match b {
                    BuiltinTypes::Int => Some(BuiltinTypes::String),
                    BuiltinTypes::Bool => {
                        return None;
                    }
                    BuiltinTypes::String => Some(BuiltinTypes::String),
                    BuiltinTypes::Object { fields: _ } => {
                        return None;
                    }
                    BuiltinTypes::Float => Some(BuiltinTypes::String),
                    BuiltinTypes::Array { element_type: _ } => None,
                }
            BuiltinTypes::Object { fields: _ } => None,
            BuiltinTypes::Float =>
                match b {
                    BuiltinTypes::Int => Some(BuiltinTypes::Float),
                    BuiltinTypes::Bool => {
                        return None;
                    }
                    BuiltinTypes::String => Some(BuiltinTypes::String),
                    BuiltinTypes::Object { fields: _ } => {
                        return None;
                    }
                    BuiltinTypes::Float => Some(BuiltinTypes::Float),
                    BuiltinTypes::Array { element_type: _ } => None,
                }
            BuiltinTypes::Array { element_type: _ } => None,
        };
        return result;
    }
    pub(super)fn subtract(&mut self) -> Option<BuiltinTypes> {
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();
        let result = match a {
            BuiltinTypes::Int =>
                match b {
                    BuiltinTypes::Int => Some(BuiltinTypes::Int),
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
                    BuiltinTypes::Array { element_type: _ } => None,
                }
            BuiltinTypes::Bool => None,
            BuiltinTypes::String => None,
            BuiltinTypes::Object { fields: _ } => None,
            BuiltinTypes::Array { element_type: _ } => None,

            BuiltinTypes::Float =>
                match b {
                    BuiltinTypes::Int => Some(BuiltinTypes::Float),
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
                    BuiltinTypes::Array { element_type: _ } => None,
                }
        };
        return result;
    }
    pub(super)fn compare(&mut self) -> Option<BuiltinTypes> {
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();
        let result = match a {
            BuiltinTypes::Int =>
                match b {
                    BuiltinTypes::Int => Some(BuiltinTypes::Bool),
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
                    BuiltinTypes::Array { element_type: _ } => None,
                }
            BuiltinTypes::Bool =>
                match b {
                    BuiltinTypes::Int => {
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
                    BuiltinTypes::Array { element_type: _ } => None,
                }
            BuiltinTypes::String => None,
            BuiltinTypes::Object { fields: _ } => None,
            BuiltinTypes::Float =>
                match b {
                    BuiltinTypes::Int => Some(BuiltinTypes::Bool),
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
                    BuiltinTypes::Array { element_type: _ } => None,
                }
            BuiltinTypes::Array { element_type: _ } => None,
        };
        return result;
    }
}