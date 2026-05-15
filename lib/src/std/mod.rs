use rand::Rng;
use wasm_bindgen::{ JsValue, prelude::wasm_bindgen };
use futures::executor::block_on;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn wasm_print(s: &str);
    fn code_replace(replace_with: &str, line: usize, column: usize, span: usize);
    async fn wasm_input() -> JsValue;
    fn testfn() -> JsValue;
}

use crate::{
    BuiltinTypes,
    module::{ FuncArg, FunctionSignature },
    vm::{ StackValue, vm::VM },
};

fn arg(name: &str, arg_type: BuiltinTypes) -> FuncArg {
    FuncArg {
        name: name.to_string(),
        arg_type,
        mutable: false,
        local_var_id: None,
    }
}

fn stackvalue_to_string(value: StackValue) -> String {
    match value {
        StackValue::String { value } => value,
        _ => unreachable!(),
    }
}
fn stackvalue_to_int(value: StackValue) -> i64 {
    match value {
        StackValue::Int { value } => value,
        _ => unreachable!(),
    }
}
fn stackvalue_to_f64(value: StackValue) -> f64 {
    match value {
        StackValue::Int { value } => value as f64,
        StackValue::Float { value } => value,
        _ => unreachable!(),
    }
}
fn pop_arguments(vm: &mut VM, count: usize) -> Vec<StackValue> {
    let mut arguments = vec![];
    for _ in 0..count {
        arguments.push(vm.pop());
    }
    arguments
}

fn min_numeric(a: StackValue, b: StackValue) -> StackValue {
    match (&a, &b) {
        (StackValue::Int { value: va }, StackValue::Int { value: vb }) =>
            StackValue::Int { value: (*va).min(*vb) },
        _ => StackValue::Float {
            value: stackvalue_to_f64(a).min(stackvalue_to_f64(b)),
        },
    }
}

fn max_numeric(a: StackValue, b: StackValue) -> StackValue {
    match (&a, &b) {
        (StackValue::Int { value: va }, StackValue::Int { value: vb }) =>
            StackValue::Int { value: (*va).max(*vb) },
        _ => StackValue::Float {
            value: stackvalue_to_f64(a).max(stackvalue_to_f64(b)),
        },
    }
}

pub fn get_std_functions() -> Vec<FunctionSignature> {
    let s = BuiltinTypes::String;
    let i = BuiltinTypes::Int;
    let f = BuiltinTypes::Float;
    let b = BuiltinTypes::Bool;
    vec![
        FunctionSignature { name: "izvade".to_string(), args: vec![], return_type: None },
        FunctionSignature { name: "izvadetp".to_string(), args: vec![], return_type: None },
        FunctionSignature {
            name: "ievade".to_string(),
            args: vec![],
            return_type: Some(s.clone()),
        },
        FunctionSignature {
            name: "garums".to_string(),
            args: vec![],
            return_type: Some(i.clone()),
        },
        FunctionSignature {
            name: "nejaušs".to_string(),
            args: vec![],
            return_type: Some(f.clone()),
        },
        FunctionSignature {
            name: "nejaušs_robežās".to_string(),
            args: vec![arg("min", i.clone()), arg("maks", i.clone())],
            return_type: Some(i.clone()),
        },
        FunctionSignature {
            name: "apgriezt".to_string(),
            args: vec![arg("teksts", s.clone())],
            return_type: Some(s.clone()),
        },
        FunctionSignature {
            name: "mazie_burti".to_string(),
            args: vec![arg("teksts", s.clone())],
            return_type: Some(s.clone()),
        },
        FunctionSignature {
            name: "lielie_burti".to_string(),
            args: vec![arg("teksts", s.clone())],
            return_type: Some(s.clone()),
        },
        FunctionSignature {
            name: "apakšvirkne".to_string(),
            args: vec![arg("teksts", s.clone()), arg("sākums", i.clone()), arg("garums", i.clone())],
            return_type: Some(s.clone()),
        },
        FunctionSignature {
            name: "aizvietot".to_string(),
            args: vec![arg("teksts", s.clone()), arg("no", s.clone()), arg("uz", s.clone())],
            return_type: Some(s.clone()),
        },
        FunctionSignature {
            name: "satur".to_string(),
            args: vec![arg("teksts", s.clone()), arg("meklēt", s.clone())],
            return_type: Some(b.clone()),
        },
        FunctionSignature {
            name: "sākas_ar".to_string(),
            args: vec![arg("teksts", s.clone()), arg("prefikss", s.clone())],
            return_type: Some(b.clone()),
        },
        FunctionSignature {
            name: "beidzas_ar".to_string(),
            args: vec![arg("teksts", s.clone()), arg("sufikss", s.clone())],
            return_type: Some(b.clone()),
        },
        FunctionSignature {
            name: "absolūtā_vērtība".to_string(),
            args: vec![],
            return_type: None,
        },
        FunctionSignature {
            name: "minimums".to_string(),
            args: vec![arg("a", i.clone()), arg("b", i.clone())],
            return_type: None,
        },
        FunctionSignature {
            name: "maksimums".to_string(),
            args: vec![arg("a", i.clone()), arg("b", i.clone())],
            return_type: None,
        },
        FunctionSignature {
            name: "apaļot".to_string(),
            args: vec![arg("x", f.clone())],
            return_type: Some(i.clone()),
        },
        FunctionSignature {
            name: "grīda".to_string(),
            args: vec![arg("x", f.clone())],
            return_type: Some(f.clone()),
        },
        FunctionSignature {
            name: "griesti".to_string(),
            args: vec![arg("x", f.clone())],
            return_type: Some(f.clone()),
        },
        FunctionSignature {
            name: "pakāpināt".to_string(),
            args: vec![arg("bāze", f.clone()), arg("eksponents", f.clone())],
            return_type: Some(f.clone()),
        },
        FunctionSignature {
            name: "kvadrātsakne".to_string(),
            args: vec![arg("x", f.clone())],
            return_type: Some(f.clone()),
        },
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
    block_on(async {
        let value = wasm_input().await.as_string().unwrap();
        vm.push(&BuiltinTypes::String, &value);
    });
    #[cfg(not(target_family = "wasm"))]
    vm.input("");
}
pub fn nejauss_robezas(vm: &mut VM) {
    let args = pop_arguments(vm, 2);
    let value = rand::thread_rng().gen_range(
        stackvalue_to_int(args[1].clone())..stackvalue_to_int(args[0].clone()),
    );
    vm.push_stackvalue(StackValue::Int { value });
}
pub fn nejauss(vm: &mut VM) {
    let value = rand::thread_rng().gen::<f64>();
    vm.push_stackvalue(StackValue::Float { value });
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

pub fn apgriezt(vm: &mut VM) {
    let teksts = stackvalue_to_string(pop_arguments(vm, 1)[0].clone());
    vm.push_stackvalue(StackValue::String { value: teksts.trim().to_string() });
}

pub fn mazie_burti(vm: &mut VM) {
    let teksts = stackvalue_to_string(pop_arguments(vm, 1)[0].clone());
    vm.push_stackvalue(StackValue::String { value: teksts.to_lowercase() });
}

pub fn lielie_burti(vm: &mut VM) {
    let teksts = stackvalue_to_string(pop_arguments(vm, 1)[0].clone());
    vm.push_stackvalue(StackValue::String { value: teksts.to_uppercase() });
}

pub fn apaksvirkne(vm: &mut VM) {
    let args = pop_arguments(vm, 3);
    let garums = stackvalue_to_int(args[0].clone()) as usize;
    let sakums = stackvalue_to_int(args[1].clone()) as usize;
    let teksts = stackvalue_to_string(args[2].clone());
    let chars: Vec<char> = teksts.chars().collect();
    let sub: String = chars.into_iter().skip(sakums).take(garums).collect();
    vm.push_stackvalue(StackValue::String { value: sub });
}

pub fn aizvietot(vm: &mut VM) {
    let args = pop_arguments(vm, 3);
    let uz = stackvalue_to_string(args[0].clone());
    let no = stackvalue_to_string(args[1].clone());
    let teksts = stackvalue_to_string(args[2].clone());
    vm.push_stackvalue(StackValue::String { value: teksts.replace(&no, &uz) });
}

pub fn satur(vm: &mut VM) {
    let args = pop_arguments(vm, 2);
    let meklet = stackvalue_to_string(args[0].clone());
    let teksts = stackvalue_to_string(args[1].clone());
    vm.push_stackvalue(StackValue::Bool { value: teksts.contains(&meklet) });
}

pub fn sakas_ar(vm: &mut VM) {
    let args = pop_arguments(vm, 2);
    let prefikss = stackvalue_to_string(args[0].clone());
    let teksts = stackvalue_to_string(args[1].clone());
    vm.push_stackvalue(StackValue::Bool { value: teksts.starts_with(&prefikss) });
}

pub fn beidzas_ar(vm: &mut VM) {
    let args = pop_arguments(vm, 2);
    let sufikss = stackvalue_to_string(args[0].clone());
    let teksts = stackvalue_to_string(args[1].clone());
    vm.push_stackvalue(StackValue::Bool { value: teksts.ends_with(&sufikss) });
}

pub fn absoluta_vertiba(vm: &mut VM) {
    let value = vm.pop();
    let result = match value {
        StackValue::Int { value } => StackValue::Int { value: value.abs() },
        StackValue::Float { value } => StackValue::Float { value: value.abs() },
        _ => unreachable!(),
    };
    vm.push_stackvalue(result);
}

pub fn minimums(vm: &mut VM) {
    let args = pop_arguments(vm, 2);
    vm.push_stackvalue(min_numeric(args[1].clone(), args[0].clone()));
}

pub fn maksimums(vm: &mut VM) {
    let args = pop_arguments(vm, 2);
    vm.push_stackvalue(max_numeric(args[1].clone(), args[0].clone()));
}

pub fn apalot(vm: &mut VM) {
    let x = stackvalue_to_f64(pop_arguments(vm, 1)[0].clone());
    vm.push_stackvalue(StackValue::Int { value: x.round() as i64 });
}

pub fn grida(vm: &mut VM) {
    let x = stackvalue_to_f64(pop_arguments(vm, 1)[0].clone());
    vm.push_stackvalue(StackValue::Float { value: x.floor() });
}

pub fn griesti(vm: &mut VM) {
    let x = stackvalue_to_f64(pop_arguments(vm, 1)[0].clone());
    vm.push_stackvalue(StackValue::Float { value: x.ceil() });
}

pub fn pakapinat(vm: &mut VM) {
    let args = pop_arguments(vm, 2);
    let eksponents = stackvalue_to_f64(args[0].clone());
    let baze = stackvalue_to_f64(args[1].clone());
    vm.push_stackvalue(StackValue::Float { value: baze.powf(eksponents) });
}

pub fn kvadratsakne(vm: &mut VM) {
    let x = stackvalue_to_f64(pop_arguments(vm, 1)[0].clone());
    vm.push_stackvalue(StackValue::Float { value: x.sqrt() });
}
