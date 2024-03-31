use crate::block::Block;
use crate::{CelsiumProgram, BUILTIN_TYPES, OPTCODE};
#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub main_block: Block,
    functions: Vec<Function>,
    _id: usize,
}
#[derive(Clone, Debug)]
pub enum FunctionReturnType {
    NONE,
    BuiltinTypes,
}
#[derive(Clone, Debug)]
pub struct FunctionSignature {
    pub(crate) name: String,
    pub(crate) return_type: FunctionReturnType,
    pub(crate) args: Vec<FuncArg>,
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

#[derive(Clone, Debug)]
pub struct FuncArg {
    name: String,
    arg_type: BUILTIN_TYPES,
}
#[derive(Clone, Debug)]
pub enum VISIBILITY {
    PRIVATE,
    PUBLIC,
}
#[derive(Debug, Clone)]
pub struct Function {
    signature: FunctionSignature,
    body: Block,
    visibility: VISIBILITY,
}

fn load_function_bytecode(name: String, module: &Module) -> Result<Vec<OPTCODE>, String> {
    for func in &module.functions {
        if func.signature.name == name {
            return Ok(func.clone().body.bytecode);
        }
    }
    Err(format!("Could not find function {} in module", { name }))
}

impl Module {
    pub fn new(name: &str, celsius_program: &mut CelsiumProgram) -> Module {
        let module = Module {
            name: name.to_string(),
            _id: celsius_program.modules.len(),
            main_block: Block::new(),
            functions: vec![],
        };
        module
    }
    pub fn add_main_block(&mut self, mut block: Block) {
        let mut bytecode_inserted: Vec<OPTCODE> = vec![];
        for optcode in block.clone().bytecode {
            match optcode {
                OPTCODE::CALL_FUNCTION { name } => {
                    bytecode_inserted.append(&mut load_function_bytecode(name, self).unwrap())
                }
                _ => bytecode_inserted.push(optcode),
            }
        }
        block.bytecode = bytecode_inserted.clone();
        //println!("{:?}", bytecode_inserted);
        self.main_block = block;
    }
    pub fn define_function(
        &mut self,
        body_block: Block,
        visibility: VISIBILITY,
        signature: FunctionSignature,
    ) {
        self.functions.push(Function {
            signature: signature,
            body: body_block,
            visibility: visibility,
        })
    }
}
