use crate::{ block::Block, module::{ FunctionSignature, VISIBILITY }, BUILTIN_TYPES };

#[derive(Clone, Debug)]
pub enum BINOP {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    REMAINDER,
    LessThan,
    LargerThan,
    LessOrEq,
    LargerOrEq,
    NotEq,
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
        id: usize,
    },
    CALL_FUNCTION {
        name: String,
    },
    CALL_FUNCTION_WITH_BYTECODE {
        bytecode: Vec<OPTCODE>,
    },
    RETURN_FROM_FUNCTION,
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
        id: usize,
    },
    DefineArray {
        id: usize,
        init_values_count: usize
    },
    GET_FROM_ARRAY {
        id: usize,
    },
    ASSIGN_AT_ARRAY_INDEX {
        id: usize,
    },
    PUSH_TO_ARRAY {
        id: usize,
    },
    GET_ARRAY_LENGTH {
        id: usize,
    },
    ASSIGN_VAR {
        id: usize,
    },
    DEFINE_FUNCTION {
        body_block: Block,
        visibility: VISIBILITY,
        signature: FunctionSignature,
    },
    CREATE_OBJECT {
        name: String,
        field_names: Vec<String>,
    },
    CALL_SPECIAL_FUNCTION {
        function: super::SpecialFunctions,
    },
}
