use crate::{CelsiumConfig, OPTCODE};
pub mod vm;
use num::bigint::{BigInt, ToBigInt};
use vm::VM;
mod math_operators;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn wasm_print(s: &str);
}

#[derive(Debug, PartialEq, Clone)]
pub enum StackValue {
    BOOL { value: bool },
    BIGINT { value: BigInt },
    STRING { value: String },
    ARRAY { value: Vec<StackValue> },
}
