use crate::block::Block;
use crate::{CelsiumProgram, BuiltinTypes};
#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub main_block: Option<Block>,
    pub functions: Vec<Function>,
    _id: usize,
}
#[derive(Clone, Debug,serde::Deserialize, serde::Serialize)]
pub enum FunctionReturnType {
    NONE,
    BuiltinTypes,
}
#[derive(Clone, Debug,serde::Deserialize, serde::Serialize)]
pub struct FunctionSignature {
    pub name: String,
    pub return_type: FunctionReturnType,
    pub args: Vec<FuncArg>,
}
impl FunctionSignature {
    pub fn new(
        func_name: String,
        args: Vec<FuncArg>,
        return_type: FunctionReturnType,
    ) -> FunctionSignature {
        FunctionSignature {
            name: func_name,
            args,
            return_type,
        }
    }
}

#[derive(Clone, Debug,serde::Deserialize, serde::Serialize)]
pub struct FuncArg {
    pub name: String,
    pub arg_type: BuiltinTypes,
}
#[derive(Clone, Debug,serde::Deserialize, serde::Serialize)]
pub enum VISIBILITY {
    PRIVATE,
    PUBLIC,
}
#[derive(Debug, Clone)]
pub struct Function {
    pub(crate) signature: FunctionSignature,
    pub(crate) body: Block,
}



impl Module {
    pub fn new(name: &str, celsius_program: &mut CelsiumProgram) -> Module {
        let module = Module {
            name: name.to_string(),
            _id: celsius_program.modules.len(),
            main_block: None,
            functions: vec![],
        };
        module
    }
    pub fn add_main_block(&mut self, mut block: Block) {
        block.bytecode = block.clone().bytecode;
        self.main_block = Some(block);
    }
}
