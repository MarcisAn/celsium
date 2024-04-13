use std::collections::HashMap;

use block::Block;
use module::Function;
use module::FunctionReturnType;
use module::FunctionSignature;
use module::Module;

pub mod block;
pub mod module;
mod vm;
use module::VISIBILITY;
use rand::Rng;
use vm::vm::VM;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Debug)]
pub enum BINOP {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    REMAINDER,
    LESS_THAN,
    LARGER_THAN,
    LESS_OR_EQ,
    LARGER_OR_EQ,
    NOT_EQ,
    EQ,
    AND,
    OR,
    XOR,
}

#[derive(Clone, Debug)]
pub enum OPTCODE {
    LOAD_CONST {
        data_type: BUILTIN_TYPES,
        data: String,
    },
    LOAD_VAR {
        name: String,
    },
    CALL_FUNCTION {
        name: String,
    },
    CALL_FUNCTION_WITH_BYTECODE {
        bytecode: Vec<OPTCODE>,
    },
    CALL_PRINT_FUNCTION {
        newline: bool,
    },
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    REMAINDER,
    LESS_THAN,
    LARGER_THAN,
    LESS_OR_EQ,
    LARGER_OR_EQ,
    NOT_EQ,
    EQ,
    OR,
    AND,
    XOR,
    JUMP_IF_FALSE {
        steps: usize,
    },
    JUMP {
        steps: usize,
    },
    JUMP_BACK {
        steps: usize,
    },
    DEFINE_VAR {
        data_type: BUILTIN_TYPES,
        visibility: VISIBILITY,
        name: String,
    },
    DEFINE_ARRAY {
        visibility: VISIBILITY,
        name: String,
        init_values_count: usize,
    },
    GET_FROM_ARRAY {
        name: String,
    },
    PUSH_TO_ARRAY {
        name: String,
    },
    GET_ARRAY_LENGTH {
        name: String,
    },
    ASSIGN_VAR {
        name: String,
    },
    DEFINE_FUNCTION {
        body_block: Block,
        visibility: VISIBILITY,
        signature: FunctionSignature,
    },
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

fn generate_rand_varname(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            ~!@#$%^&*()-_+=";

    let mut rng = rand::thread_rng();
    let randstring: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    randstring
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

    pub fn run_program(&mut self) {
        let mut bytecode: Vec<OPTCODE> = vec![];
        for module in &self.modules {
            bytecode.append(&mut module.main_block.bytecode.clone());
        }
        let mut vm = VM::new();

        self.run(&mut vm, &bytecode);
    }

    pub fn run(&mut self, vm: &mut VM, bytecode: &Vec<OPTCODE>) {
        let mut index: isize = 0;
        while index < bytecode.len() as isize {
            let optcode = &bytecode[index as usize];
            //println!("running optcode {:?}", optcode);
            match optcode {
                OPTCODE::LOAD_CONST { data_type, data } => vm.push(&data_type, &data),
                OPTCODE::CALL_FUNCTION { name } => {
                    for function in &self.modules.clone()[0].functions {
                        if function.signature.name == name.to_string() {
                            let mut argument_names_to_replace = HashMap::new();

                            for arg in &function.signature.args {
                                let var_name = "__".to_string()
                                    + &arg.name.to_string()
                                    + &generate_rand_varname(5);
                                vm.define_var(0, var_name.clone(), &VISIBILITY::PRIVATE);
                                argument_names_to_replace.insert(arg.clone().name, var_name);
                            }
                            let mut replaced_bytecode: Vec<OPTCODE> = vec![];
                            for optcode in &function.body.bytecode.clone() {
                                match optcode {
                                    OPTCODE::LOAD_VAR { name } => {
                                        match argument_names_to_replace.get(name) {
                                            Some(ref new_name) => {
                                                replaced_bytecode.push(OPTCODE::LOAD_VAR {
                                                    name: new_name.to_string(),
                                                })
                                            }
                                            None => todo!(),
                                        }
                                    }
                                    _ => replaced_bytecode.push(optcode.clone()),
                                }
                            }
                            self.run(vm, &replaced_bytecode);
                        }
                    }
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
                OPTCODE::LOAD_VAR { name } => vm.load_var(name),
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
            }
            index += 1;
        }
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

        main_block.load_const(BUILTIN_TYPES::MAGIC_INT, "2");
        main_block.load_const(BUILTIN_TYPES::STRING, "a");

        main_block.define_array(VISIBILITY::PRIVATE, "aa".to_string(), 2);
        main_block.load_variable("aa");
        main_block.call_print_function(true);
        main_block.load_const(BUILTIN_TYPES::MAGIC_INT, "2");
        main_block.push_to_array("aa");
        main_block.load_variable("aa");
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
