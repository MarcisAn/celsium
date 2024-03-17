use crate::block::{Block, FUNC_VISIBILITY};
use crate::vm::StackValue;
use crate::{CelsiumProgram, BINOP};
#[derive(Clone)]
pub struct Module {
    id: usize,
    name: String,
    pub(super) blocks: Vec<Block>,
}

impl Module {
    pub fn new(name: &str, celsius_program: &mut CelsiumProgram) -> Module {
        let module = Module {
            name: name.to_string(),
            id: celsius_program.modules.len(),
            blocks: vec![],
        };
        module
    }
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
    pub fn define_function(&mut self, body: Block, visibility: FUNC_VISIBILITY) {}
}
