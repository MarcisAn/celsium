use crate::{block::Block, module::{FunctionSignature, VISIBILITY}, BUILTIN_TYPES};

#[derive(Clone, Debug)]
pub enum BINOP {
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
        name: String,
    },
    CALL_FUNCTION {
        name: String,
    },
    CALL_FUNCTION_WITH_BYTECODE {
        bytecode: Vec<OPTCODE>,
    },
    CALL_PRINT_FUNCTION {
        newline: bool,
    },
    CALL_INPUT,
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
        visibility: VISIBILITY,
        name: String,
    },
    DEFINE_ARRAY {
        visibility: VISIBILITY,
        name: String,
        init_values_count: usize,
    },
    GET_FROM_ARRAY {
        name: String,
    },
    PUSH_TO_ARRAY {
        name: String,
    },
    GET_ARRAY_LENGTH {
        name: String,
    },
    ASSIGN_VAR {
        name: String,
    },
    DEFINE_FUNCTION {
        body_block: Block,
        visibility: VISIBILITY,
        signature: FunctionSignature,
    },
}