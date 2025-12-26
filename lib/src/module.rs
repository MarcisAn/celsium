use crate::block::Block;
use crate::BuiltinTypes;


#[derive(Clone, Debug,serde::Deserialize, serde::Serialize)]
pub struct FunctionSignature {
    pub name: String,
    pub return_type: Option<BuiltinTypes>,
    pub args: Vec<FuncArg>,
}
impl FunctionSignature {
    pub fn new(
        func_name: String,
        args: Vec<FuncArg>,
        return_type: Option<BuiltinTypes>,
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
    pub signature: FunctionSignature,
    pub body: Block,
}
