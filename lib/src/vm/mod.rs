pub mod vm;
use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::vm::format_for_print::format_for_print;
mod math_operators;
mod array;
pub mod format_for_print;

#[derive(Debug, PartialEq, Clone,Serialize, Deserialize)]

pub enum StackValue {
    Bool { value: bool },
    Int { value: i64 },
    Float {value: f64},
    String { value: String },
    Array { value: Vec<StackValue> },
    Object {value: Vec<ObjectField>}
}
#[derive(Debug, PartialEq, Clone,Serialize, Deserialize)]
pub struct  ObjectField {
    pub name: String,
    pub value: StackValue
}
impl fmt::Display for StackValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format_for_print(self, false))
    }
}