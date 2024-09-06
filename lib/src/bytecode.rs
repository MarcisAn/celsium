use crate::{ block::Block, BuiltinTypes };

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum OPTCODE {
    LoadInt {
        value: i64
    },
    LoadBool {
        value: bool
    },
    LoadString {
        value: String
    },
    LoadFloat {
        value: f64
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
        field_names: Vec<String>,
    },
    GetObjectField {
        field_name: String,
    },
    DefineArray {
        id: usize,
        init_values_count: usize,
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
    CallSpecialFunction {
        function: super::SpecialFunctions,
    },
    SimpleLoop {
        body_block: Block,
    },
    PushToTestingStack {
        duplicate_stackvalue: bool,
    },
}
