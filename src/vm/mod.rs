pub mod vm;
use num::bigint::BigInt;
use serde::{Deserialize, Serialize};
mod math_operators;
mod array;
mod format_for_print;

#[derive(Debug, PartialEq, Clone,Serialize, Deserialize)]

pub enum StackValue {
    BOOL { value: bool },
    BIGINT { value: BigInt },
    FLOAT {value: f64},
    STRING { value: String },
    ARRAY { value: Vec<StackValue> },
}
#[derive(Debug, PartialEq, Clone,Serialize, Deserialize)]
pub struct  ObjectField {
    pub name: String,
    pub value: StackValue
}
