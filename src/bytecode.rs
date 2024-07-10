use crate::{ block::Block, module::{ FunctionSignature, VISIBILITY }, BuiltinTypes };

#[derive(Clone, Debug)]
pub enum BINOP {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    LessThan,
    LargerThan,
    LessOrEq,
    LargerOrEq,
    NotEq,
    Eq,
    And,
    Or,
    Xor,
}

#[derive(Clone, Debug)]
pub enum OPTCODE {
    LoadConst {
        data_type: BuiltinTypes,
        data: String,
    },
    LoadVar {
        id: usize,
    },
    CallFunction {
        name: String,
    },
    CallFunctionWithBytecode {
        bytecode: Vec<OPTCODE>,
    },
    ReturnFromFunction,
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    LessThan,
    LargerThan,
    LessOrEq,
    LargerOrEq,
    NotEq,
    Eq,
    Or,
    And,
    Xor,
    JumpIfFalse {
        steps: usize,
    },
    Jump {
        steps: usize,
    },
    JumpBack {
        steps: usize,
    },
    DefineVar {
        id: usize,
    },
    DefineObject {
        id: usize,
    },
    CreateObject {
        field_names: Vec<String>
    },
    GetObjectField {
        field_name: String
    },
    DefineArray {
        id: usize,
        init_values_count: usize
    },
    GetFromArray {
        id: usize,
    },
    AssignAtArrayIndex {
        id: usize,
    },
    PushToArray {
        id: usize,
    },
    GettArrayLength {
        id: usize,
    },
    AssignVar {
        id: usize,
    },
    DefineFunction {
        body_block: Block,
        visibility: VISIBILITY,
        signature: FunctionSignature,
    },
    CallSpecialFunction {
        function: super::SpecialFunctions,
    },
    SimpleLoop {
        body_block: Block
    },
    PushToTestingStack {duplicate_stackvalue: bool}    
}
