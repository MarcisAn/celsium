pub mod vm;
use num::bigint::BigInt;
use serde::{Deserialize, Serialize};
mod math_operators;
mod array;
mod format_for_print;

#[derive(Debug, PartialEq, Clone,Serialize, Deserialize)]

pub enum StackValue {
    Bool { value: bool },
    Int { value: i64 },
    Float {value: f64},
    String { value: String },
    ARRAY { value: Vec<StackValue> },
    Object {value: Vec<ObjectField>}
}
#[derive(Debug, PartialEq, Clone,Serialize, Deserialize)]
pub struct  ObjectField {
    pub name: String,
    pub value: StackValue
}
