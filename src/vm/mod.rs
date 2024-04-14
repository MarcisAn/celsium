use crate::{CelsiumConfig, OPTCODE};
pub mod vm;
use num::bigint::{BigInt, ToBigInt};
use vm::VM;
mod math_operators;

#[derive(Debug, PartialEq, Clone)]
pub enum StackValue {
    BOOL { value: bool },
    BIGINT { value: BigInt },
    STRING { value: String },
    ARRAY { value: Vec<StackValue> },
}
