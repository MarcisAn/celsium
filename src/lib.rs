use std::collections::HashMap;
pub mod bytecode;
use bytecode::{BINOP, OPTCODE};
use block::Block;
use module::Function;
use module::FunctionReturnType;
use module::FunctionSignature;
use module::Module;

pub mod block;
pub mod module;
mod vm;
use module::VISIBILITY;
use vm::vm::VM;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn wasm_print(s: &str);
    fn wasm_input() -> String;
}

#[derive(Clone, Debug)]
pub enum BUILTIN_TYPES {
    MAGIC_INT,
    BOOL,
    STRING,
}

pub struct CelsiumConfig {
    is_wasm: bool,
}

pub struct CelsiumProgram {
    modules: Vec<Module>,
    config: CelsiumConfig,
}



impl CelsiumProgram {
    pub fn new(is_wasm: bool) -> CelsiumProgram {
        CelsiumProgram {
            modules: vec![],
            config: CelsiumConfig { is_wasm: is_wasm },
        }
    }

    pub fn add_module(&mut self, module: &Module) {
        self.modules.push(module.clone());
    }

    pub fn run_program(&mut self) -> Result<(), String> {
        let mut bytecode: Vec<OPTCODE> = vec![];
        for module in &self.modules {
            bytecode.append(&mut module.main_block.bytecode.clone());
        }
        let mut vm = VM::new();

        return self.run(&mut vm, &bytecode);
    }

    pub fn run(&mut self, vm: &mut VM, bytecode: &Vec<OPTCODE>) -> Result<(), String> {
        let mut index: isize = 0;
        while index < bytecode.len() as isize {
            let optcode = &bytecode[index as usize];
            //println!("running optcode {:?}", optcode);
            match optcode {
                OPTCODE::LOAD_CONST { data_type, data } => vm.push(&data_type, &data),
                OPTCODE::CALL_FUNCTION { name } => {
                    vm.call_function(name, self);
                }
                OPTCODE::RETURN_FROM_FUNCTION => {
                    break;
                }
                OPTCODE::CALL_FUNCTION_WITH_BYTECODE { bytecode } => {
                    panic!();
                }
                OPTCODE::ADD => vm.aritmethics("+"),
                OPTCODE::SUBTRACT => vm.aritmethics("-"),
                OPTCODE::MULTIPLY => vm.aritmethics("*"),
                OPTCODE::DIVIDE => vm.aritmethics("/"),
                OPTCODE::REMAINDER => vm.aritmethics("%"),
                OPTCODE::CALL_PRINT_FUNCTION { newline } => {
                    let printable = &vm.format_for_print(*newline);
                    #[cfg(target_family = "wasm")]
                    wasm_print(printable);
                    print!("{}", printable);
                }
                OPTCODE::JUMP_IF_FALSE { steps } => {
                    if (vm.must_jump()) {
                        index += *steps as isize
                    }
                }
                OPTCODE::JUMP { steps } => index += *steps as isize,
                OPTCODE::JUMP_BACK { steps } => index -= *steps as isize,
                OPTCODE::LESS_THAN => vm.aritmethics("<"),
                OPTCODE::LARGER_THAN => vm.aritmethics(">"),
                OPTCODE::LESS_OR_EQ => vm.aritmethics("<="),
                OPTCODE::LARGER_OR_EQ => vm.aritmethics(">="),
                OPTCODE::NOT_EQ => vm.aritmethics("!="),
                OPTCODE::EQ => vm.aritmethics("=="),
                OPTCODE::OR => vm.aritmethics("or"),
                OPTCODE::AND => vm.aritmethics("and"),
                OPTCODE::XOR => vm.aritmethics("xor"),
                OPTCODE::DEFINE_VAR {
                    data_type,
                    visibility,
                    name,
                } => vm.define_var(0, name.to_string(), visibility),
                OPTCODE::LOAD_VAR { name } => vm.load_var(name)?,
                OPTCODE::ASSIGN_VAR { name } => vm.assign_var(name),
                OPTCODE::DEFINE_ARRAY {
                    visibility,
                    name,
                    init_values_count,
                } => vm.define_array(0, name.to_string(), visibility, *init_values_count),
                OPTCODE::GET_FROM_ARRAY { name } => vm.get_from_array(name),
                OPTCODE::PUSH_TO_ARRAY { name } => vm.push_to_array(name),
                OPTCODE::GET_ARRAY_LENGTH { name } => vm.get_array_length(name),
                OPTCODE::DEFINE_FUNCTION {
                    body_block,
                    visibility,
                    signature,
                } => self.modules[0].functions.push(Function {
                    signature: signature.clone(),
                    body: body_block.clone(),
                    visibility: visibility.clone(),
                }),
                OPTCODE::CALL_INPUT => { 
                    #[cfg(target_family = "wasm")]
                    vm.push(&BUILTIN_TYPES::STRING, &wasm_input());
                    #[cfg(not(target_family = "wasm"))]
                    vm.input("");
                },
            }
            index += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use self::module::{FunctionSignature, VISIBILITY};

    use super::*;

    #[test]
    fn it_works() {
        let mut celsium = CelsiumProgram::new(false);
        let mut main_module = Module::new("main", &mut celsium);
        let mut main_block = Block::new();

        main_block.load_const(BUILTIN_TYPES::STRING, "a");
        main_block.input_function();
        main_block.call_print_function(true);

        let mut i = 0;
        while i < main_block.bytecode.len() {
            println!("{} {:?}", i, main_block.bytecode[i]);
            i += 1;
        }
        main_module.add_main_block(main_block);
        celsium.add_module(&main_module);
        celsium.run_program();
    }
}
