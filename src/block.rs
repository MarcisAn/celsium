use rand::Rng;
use crate::module::FunctionSignature;
use crate::SpecialFunctions;
use crate::{module::VISIBILITY, BINOP, BUILTIN_TYPES, OPTCODE};

#[derive(Clone, Debug)]
pub struct Block {
    pub bytecode: Vec<OPTCODE>
}
fn generate_rand_varname(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            ~!@#$%^&*()-_+=";

    let mut rng = rand::thread_rng();
    let randstring: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    randstring
}

impl Block {
    pub fn new() -> Block {
        Block { bytecode: vec![] }
    }
    pub fn load_const(&mut self, data_type: BUILTIN_TYPES, value: &str) {
        self.bytecode.push(OPTCODE::LOAD_CONST {
            data: value.to_owned(),
            data_type,
        });
    }
    pub fn binop(&mut self, operator: BINOP) {
        self.bytecode.push(match operator {
            BINOP::ADD => OPTCODE::ADD,
            BINOP::SUBTRACT => OPTCODE::SUBTRACT,
            BINOP::MULTIPLY => OPTCODE::MULTIPLY,
            BINOP::DIVIDE => OPTCODE::DIVIDE,
            BINOP::REMAINDER => OPTCODE::REMAINDER,
            BINOP::LessThan => OPTCODE::LESS_THAN,
            BINOP::LargerThan => OPTCODE::LARGER_THAN,
            BINOP::LessOrEq => OPTCODE::LESS_OR_EQ,
            BINOP::LargerOrEq => OPTCODE::LARGER_OR_EQ,
            BINOP::NotEq => OPTCODE::NOT_EQ,
            BINOP::EQ => OPTCODE::EQ,
            BINOP::AND => OPTCODE::AND,
            BINOP::OR => OPTCODE::OR,
            BINOP::XOR => OPTCODE::XOR,
        });
    }
    pub fn define_if_block(&mut self, block: Block) {
        let block_length = block.bytecode.len();
        self.bytecode.push(OPTCODE::JUMP_IF_FALSE {
            steps: block_length,
        });
        for optcode in block.bytecode {
            self.bytecode.push(optcode);
        }
    }
    pub fn define_if_else_block(&mut self, if_block: Block, else_block: Block) {
        //println!("{:?}", else_block);
        let if_block_length = if_block.bytecode.len();
        let else_block_length = else_block.bytecode.len();
        self.bytecode.push(OPTCODE::JUMP_IF_FALSE {
            steps: if_block_length + 1,
        });
        for optcode in if_block.bytecode {
            self.bytecode.push(optcode);
        }
        self.bytecode.push(OPTCODE::JUMP {
            steps: else_block_length,
        });
        for optcode in else_block.bytecode {
            self.bytecode.push(optcode);
        }
    }
    pub fn call_function(&mut self, name: &str) {
        self.bytecode.push(OPTCODE::CALL_FUNCTION {
            name: name.to_string(),
        });
    }
    pub fn define_simple_loop(&mut self, loop_block: Block, loops_count_block: Block) {
        let block_length = loop_block.bytecode.len();
        let tmp_var_name = "_".to_string() + &generate_rand_varname(17);

        //Define a variable with a crazy name to hold the value for repeats with the value of 0

        self.bytecode.push(OPTCODE::LOAD_CONST {
            data_type: BUILTIN_TYPES::MAGIC_INT,
            data: "0".to_string(),
        });
        self.bytecode.push(OPTCODE::DEFINE_VAR {
            data_type: BUILTIN_TYPES::MAGIC_INT,
            visibility: VISIBILITY::PRIVATE,
            name: tmp_var_name.clone(),
        });

        //make a conditional with the tmp variable

        self.bytecode.push(OPTCODE::LOAD_VAR {
            name: tmp_var_name.clone(),
        });
        for optcode in &loops_count_block.bytecode {
            self.bytecode.push(optcode.clone());
        }
        self.bytecode.push(OPTCODE::LESS_THAN);

        //make a while loop

        self.bytecode.push(OPTCODE::JUMP_IF_FALSE {
            steps: block_length + 5,
        });
        for optcode in loop_block.bytecode {
            self.bytecode.push(optcode);
        }

        //add 1 to the tmp variable at the end of the loop

        self.bytecode.push(OPTCODE::LOAD_VAR {
            name: tmp_var_name.clone(),
        });
        self.bytecode.push(OPTCODE::LOAD_CONST {
            data_type: BUILTIN_TYPES::MAGIC_INT,
            data: "1".to_string(),
        });
        self.bytecode.push(OPTCODE::ADD);
        self.bytecode.push(OPTCODE::ASSIGN_VAR {
            name: tmp_var_name.clone(),
        });

        //jump to the start of the loop

        self.bytecode.push(OPTCODE::JUMP_BACK {
            steps: block_length + &loops_count_block.bytecode.len() + 8,
        });
    }
    pub fn define_while_loop(&mut self, loop_block: Block, conditional_block: Block) {
        let block_length = loop_block.bytecode.len();
        for optcode in &conditional_block.bytecode {
            self.bytecode.push(optcode.clone());
        }
        self.bytecode.push(OPTCODE::JUMP_IF_FALSE {
            steps: block_length + 1,
        });
        for optcode in loop_block.bytecode {
            self.bytecode.push(optcode);
        }
        self.bytecode.push(OPTCODE::JUMP_BACK {
            steps: block_length + &conditional_block.bytecode.len() + 2,
        });
    }
    pub fn define_variable(
        &mut self,
        data_type: BUILTIN_TYPES,
        visibility: VISIBILITY,
        name: &str,
    ) {
        self.bytecode.push(OPTCODE::DEFINE_VAR {
            data_type,
            visibility,
            name: name.to_string(),
        })
    }
    pub fn define_function(
        &mut self,
        body_block: Block,
        visibility: VISIBILITY,
        signature: FunctionSignature,
    ) {
        self.bytecode.push(OPTCODE::DEFINE_FUNCTION {
            signature: signature,
            visibility: visibility,
            body_block,
        })
    }
    pub fn return_from_function(&mut self) {
        self.bytecode.push(OPTCODE::RETURN_FROM_FUNCTION);
    }
    pub fn define_array(&mut self, visibility: VISIBILITY, name: String, init_values_count: usize) {
        self.bytecode.push(OPTCODE::DefineArray {
            visibility,
            name,
            init_values_count,
        })
    }

    pub fn load_from_array(&mut self, name: &str) {
        self.bytecode.push(OPTCODE::GET_FROM_ARRAY {
            name: name.to_string(),
        })
    }

    pub fn call_method_on_variable(method_name: String) {}

    pub fn get_array_length(&mut self, name: &str) {
        self.bytecode.push(OPTCODE::GET_ARRAY_LENGTH {
            name: name.to_string(),
        });
    }
    pub fn push_to_array(&mut self, name: &str) {
        self.bytecode.push(OPTCODE::PUSH_TO_ARRAY {
            name: name.to_string(),
        })
    }
    pub fn assign_variable(&mut self, name: &str) {
        self.bytecode.push(OPTCODE::ASSIGN_VAR {
            name: name.to_string(),
        })
    }
    pub fn load_variable(&mut self, name: &str) {
        self.bytecode.push(OPTCODE::LOAD_VAR {
            name: name.to_string(),
        })
    }
    pub fn call_special_function(&mut self, function: SpecialFunctions){
        self.bytecode.push(OPTCODE::CALL_SPECIAL_FUNCTION { function });
    }
    pub fn create_object(&mut self, name: &str, field_names: Vec<&str>){
        let mut owned_names: Vec<String> = vec![];
        for name in field_names {
            owned_names.push(name.to_string());
        }
        self.bytecode.push(OPTCODE::CREATE_OBJECT { name: name.to_string(), field_names: owned_names });
    }
}
