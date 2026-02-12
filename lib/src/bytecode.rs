use crate::block::{Block, TextSpan};

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
    Not
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
        span: TextSpan
    },
    CallFunction {
        name: String,
    },
    Add {span: TextSpan},
    Subtract,
    Multiply,
    Divide,
    Remainder,
    LessThan{span: TextSpan},
    LargerThan,
    LessOrEq,
    LargerOrEq,
    NotEq,
    Eq,
    Or,
    And,
    Xor,
    Not,
    JumpIfFalse {
        steps: usize,
        jump_target_line: usize,
        jump_target_column: usize,
        is_skipable: bool
    },
    Jump {
        steps: usize,
    },
    JumpToFunction {
        target: usize,
        function_name: Option<String>
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
    SetObjectField {
        id: usize,
        field_name: String,
    },
    CreateArray {
        init_values_count: usize,
    },
    GetIndex,
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
        function: String,
    },
    SimpleLoop {
        body_block: Block,
    },
    PushToTestingStack {
        duplicate_stackvalue: bool,
    },
    Break {span: TextSpan},
    Continue {span: TextSpan},
    Return
}
