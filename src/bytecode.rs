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
        register: usize
    },
    LoadVar {
        id: usize,
        register: usize
    },
    CallFunction {
        name: String,
    },
    CallFunctionWithBytecode {
        bytecode: Vec<OPTCODE>,
    },
    ReturnFromFunction,
    Binop {a_reg: usize, b_reg: usize, result_reg: usize, binop: BINOP},
    JumpIfFalse {
        steps: usize,
        register: usize
    },
    Jump {
        steps: usize,
    },
    JumpBack {
        steps: usize,
    },
    DefineVar {
        id: usize,
        register: usize
    },
    DefineObject {
        id: usize,
        register: usize
    },
    CreateObject {
        field_names: Vec<String>,
        field_regs: Vec<usize>,
        target_reg: usize
    },
    GetObjectField {
        field_name: String,
        object_register: usize
    },
    DefineArray {
        id: usize,
        init_values: Vec<usize>
    },
    GetFromArray {
        id: usize,
        register: usize
    },
    AssignAtArrayIndex {
        id: usize,
        value_reg: usize,
        index_reg: usize
    },
    PushToArray {
        id: usize,
        register: usize
    },
    GetArrayLength {
        id: usize,
        register: usize
    },
    AssignVar {
        id: usize,
        register: usize
    },
    DefineFunction {
        body_block: Block,
        visibility: VISIBILITY,
        signature: FunctionSignature,
    },
    CallSpecialFunction {
        function: super::SpecialFunctions,
        register: usize
    },
    SimpleLoop {
        body_block: Block,
        count_reg: usize
    },
    PushToTestingStack {duplicate_stackvalue: bool, register: usize}    
}
