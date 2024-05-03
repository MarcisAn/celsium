pub mod vm;
use num::bigint::BigInt;
use serde::{Deserialize, Serialize};
mod math_operators;

#[derive(Debug, PartialEq, Clone,Serialize, Deserialize)]

pub enum StackValue {
    BOOL { value: bool },
    BIGINT { value: BigInt },
    STRING { value: String },
    ARRAY { value: Vec<StackValue> },
    OBJECT {name: String, value: Vec<ObjectField>},
}
#[derive(Debug, PartialEq, Clone,Serialize, Deserialize)]
pub struct ObjectField {
    pub(crate) name: String,
    pub(crate) value: StackValue
}
