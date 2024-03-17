use block::{Block, FuncArg, FunctionSignature, FUNC_VISIBILITY};
use module::Module;

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
            for block in &module.blocks {
                bytecode.append(&mut block.bytecode.clone());
            }
        }
        vm::run(&bytecode);
    }
}

impl FunctionSignature {
    pub fn new(func_name: String, args: Vec<FuncArg>, return_type: String) -> FunctionSignature {
        FunctionSignature {
            name: func_name,
            args,
            return_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut celsius = CelsiumProgram::new();
        let mut main_module = Module::new("main", &mut celsius);
        let mut main_block = Block::new();
        {
            main_block.load_const(BUILTIN_TYPES::BOOL, "1");
            let mut if_block = Block::new();
            {
                if_block.load_const(BUILTIN_TYPES::STRING, "executed");
                if_block.call_print_function(true);
            }
            let mut else_block = Block::new();
            {
                else_block.load_const(BUILTIN_TYPES::STRING, "executed2");
                else_block.call_print_function(true);
            }
            main_block.define_if_else_block(if_block, else_block);
            main_block.load_const(BUILTIN_TYPES::STRING, "aaa");
            main_block.call_print_function(true);
        }
        let mut i = 0;
        while i < main_block.bytecode.len() {
            println!("{} {:?}", i, main_block.bytecode[i]);
            i += 1;
        }
        main_module.add_block(main_block);
        celsius.add_module(&main_module);
        celsius.run_program();
    }
}
