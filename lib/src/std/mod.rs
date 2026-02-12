use js_sys::Math::random;
use rand::Rng;
use wasm_bindgen::{ JsValue, prelude::wasm_bindgen };
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn wasm_print(s: &str);
    fn code_replace(replace_with: &str, line: usize, column: usize, span: usize);
    async fn wasm_input() -> JsValue;
}

use crate::{ BuiltinTypes, module::{ FuncArg, FunctionSignature }, vm::{ self, StackValue, vm::VM } };

fn f64_to_stackvalue(value: f64) -> StackValue {
    return StackValue::Float { value };
}
fn stackvalue_to_bool(value: StackValue) -> bool {
    match value {
        StackValue::Bool { value } => {
            return value;
        }
        _ => unreachable!(),
    }
}
fn stackvalue_to_string(value: StackValue) -> String {
    match value {
        StackValue::String { value } => {
            return value;
        }
        _ => unreachable!(),
    }
}
fn stackvalue_to_int(value: StackValue) -> i64 {
    match value {
        StackValue::Int { value } => {
            return value;
        }
        _ => unreachable!(),
    }
}
fn pop_arguments(vm: &mut VM, count: usize) -> Vec<StackValue> {
    let mut arguments = vec![];
    for _ in 0..count {
        arguments.push(vm.pop());
    }
    return arguments;
}

pub fn get_std_functions() -> Vec<FunctionSignature> {
    vec![
        FunctionSignature { name: "izvade".to_string(), args: vec![], return_type: None },
        FunctionSignature { name: "izvadetp".to_string(), args: vec![], return_type: None },
        FunctionSignature {
            name: "ievade".to_string(),
            args: vec![],
            return_type: Some(crate::BuiltinTypes::String),
        },
        FunctionSignature { name: "garums".to_string(), args: vec![], return_type: Some(crate::BuiltinTypes::Int) },
        FunctionSignature {
            name: "nejaušs".to_string(),
            args: vec![],
            return_type: Some(crate::BuiltinTypes::Float),
        },
        FunctionSignature {
            name: "nejaušs_robežās".to_string(),
            args: vec![
                FuncArg { name: "min".to_string(), arg_type: crate::BuiltinTypes::Int },
                FuncArg { name: "maks".to_string(), arg_type: crate::BuiltinTypes::Int }
            ],
            return_type: Some(crate::BuiltinTypes::Float),
        }
    ]
}

pub fn izvade(vm: &mut VM) {
    let printable = &vm.format_for_print(true);
    #[cfg(target_family = "wasm")]
    wasm_print(printable);
    print!("{}", printable);
}
pub fn izvadetp(vm: &mut VM) {
    let printable = &vm.format_for_print(false);
    #[cfg(target_family = "wasm")]
    wasm_print(printable);
    print!("{}", printable);
}
pub fn ievade(vm: &mut VM) {
    #[cfg(target_family = "wasm")]
    async {
        let value = &wasm_input().await.as_string().unwrap();
        vm.push(&BuiltinTypes::String, value);
    };
    #[cfg(not(target_family = "wasm"))]
    vm.input("");
}
pub fn nejauss_robezas(vm: &mut VM) {
    let args = pop_arguments(vm, 2);
    let value = rand
        ::thread_rng()
        .gen_range(stackvalue_to_int(args[1].clone())..stackvalue_to_int(args[0].clone()));
    vm.push_stackvalue(StackValue::Int {
        value,
    });
}
pub fn nejauss(vm: &mut VM) {
    let value = rand::thread_rng().gen::<f64>();
    vm.push_stackvalue(StackValue::Float {
        value,
    });
}
pub fn garums(vm: &mut VM) {
    let value = vm.pop();
    let length_value = match value {
        StackValue::Bool { value: _ } => 1,
        StackValue::Int { value } => value.to_string().len(),
        StackValue::Float { value } => value.to_string().len(),
        StackValue::String { value } => value.len(),
        StackValue::Array { value } => value.len(),
        StackValue::Object { value } => value.len(),
    };
    vm.push_stackvalue(StackValue::Int { value: length_value as i64 });
}
