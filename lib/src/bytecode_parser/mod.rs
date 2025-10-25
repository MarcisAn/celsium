use crate::bytecode::OPTCODE;

pub fn parse_bytecode(bytecode_json: String) -> Vec<OPTCODE>{
    serde_json::from_str(&bytecode_json).unwrap()
}
