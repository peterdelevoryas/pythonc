use ssa::ProgramBuilder;
use ssa::Value;
use ssa::ValueMap;
use ssa::Expr;
use ssa::BlockMap;
use ssa::BlockData;
use ssa::Block;
use explicate::Var;
use explicate::VarMap;

impl_ref!(Function, "f");
pub type FunctionGen = Gen;
pub type FunctionMap<T> = Slab<T>;

pub struct FunctionData {
    pub is_main: bool,
    pub params: Vec<Value>,
    pub values: ValueMap<Expr>,
    pub defs: BlockMap<VarMap<Value>>,
    pub blocks: BlockMap<BlockData>,
}

pub struct Builder<'a> {
    program: &'a mut ProgramBuilder,
    values: ValueMap<Expr>,
    params: Vec<Value>,
    is_main: bool,
    defs: BlockMap<VarMap<Value>>,
    blocks: BlockMap<BlockData>,
}

impl<'a> Builder<'a> {
    pub fn new(program: &'a mut ProgramBuilder) -> Self {
        Builder {
            program: program,
            is_main: false,
            params: vec![],
            values: ValueMap::new(),
            defs: BlockMap::new(),
            blocks: BlockMap::new(),
        }
    }

    pub fn is_main(&mut self, is_main: bool) {
        self.is_main = is_main;
    }

    pub fn block(&self, block: Block) -> &BlockData {
        &self.blocks[block]
    }

    pub fn block_mut(&mut self, block: Block) -> &mut BlockData {
        &mut self.blocks[block]
    }

    pub fn create_block(&mut self) -> Block {
        let block = BlockData::new();
        self.blocks.insert(block)
    }

    pub fn build(self) -> FunctionData {
        for (block, block_data) in &self.blocks {
            if block_data.end.is_none() {
                panic!("{} does not have a terminating branch intruction", block)
            }
        }
        FunctionData {
            is_main: self.is_main,
            params: self.params,
            values: self.values,
            defs: self.defs,
            blocks: self.blocks,
        }
    }
}
