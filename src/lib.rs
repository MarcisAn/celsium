use block::Block;
use module::Module;
use module::FUNCTION_RETURN_TYPE;

pub mod block;
pub mod module;
mod vm;

#[derive(Clone, Debug)]
pub enum BINOP {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    REMAINDER,
}

#[derive(Clone, Debug)]
pub enum OPTCODE {
    LOAD_CONST {
        data_type: BUILTIN_TYPES,
        data: String,
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
    JUMP_IF_FALSE {
        steps: usize,
    },
    JUMP {
        steps: usize,
    },
}
#[derive(Clone, Debug)]
pub enum BUILTIN_TYPES {
    MAGIC_INT,
    BOOL,
    STRING,
}

pub struct CelsiumProgram {
    modules: Vec<Module>,
}

impl CelsiumProgram {
    pub fn new() -> CelsiumProgram {
        CelsiumProgram { modules: vec![] }
    }
    pub fn add_module(&mut self, module: &Module) {
        self.modules.push(module.clone());
    }
    pub fn run_program(&self) {
        let mut bytecode: Vec<OPTCODE> = vec![];
        for module in &self.modules {
            bytecode.append(&mut module.main_block.bytecode.clone());
        }
        vm::run(&bytecode);
    }
}

#[cfg(test)]
mod tests {
    use self::module::{FunctionSignature, FUNC_VISIBILITY};

    use super::*;

    #[test]
    fn it_works() {
        let mut celsius = CelsiumProgram::new();
        let mut main_module = Module::new("main", &mut celsius);
        let mut main_block = Block::new();

        let mut fn_block = Block::new();
        fn_block.load_const(BUILTIN_TYPES::STRING, "aaa");
        fn_block.call_print_function(true);

        main_module.define_function(
            fn_block,
            FUNC_VISIBILITY::PRIVATE,
            FunctionSignature {
                name: "test".to_owned(),
                return_type: FUNCTION_RETURN_TYPE::NONE,
                args: vec![],
            },
        );

        main_block.call_function("test");

        let mut i = 0;
        while i < main_block.bytecode.len() {
            println!("{} {:?}", i, main_block.bytecode[i]);
            i += 1;
        }
        main_module.add_main_block(main_block);
        celsius.add_module(&main_module);
        celsius.run_program();
    }
}
