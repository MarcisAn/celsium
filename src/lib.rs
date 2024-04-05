use block::Block;
use module::FunctionReturnType;
use module::Module;

pub mod block;
pub mod module;
mod vm;
use module::VISIBILITY;
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
    pub fn run_program(&self) {
        let mut bytecode: Vec<OPTCODE> = vec![];
        for module in &self.modules {
            bytecode.append(&mut module.main_block.bytecode.clone());
        }
        vm::run(&bytecode, &self.config);
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
