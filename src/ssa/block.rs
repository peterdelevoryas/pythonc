use ssa::Value;
use ssa::Branch;
use ssa::Function;
use ssa::FunctionBuilder;
use ssa::Expr;
use std::collections::HashSet;
use std::fmt;

impl_ref!(Block, "block");
pub type BlockMap<T> = Slab<T>;

pub struct BlockData {
    pub body: Vec<Value>,
    /// Must be Some after construction
    pub end: Option<Branch>,
    pub predecessors: HashSet<Block>,
}

impl BlockData {
    pub fn new() -> BlockData {
        BlockData {
            body: Vec::new(),
            end: None,
            predecessors: HashSet::new(),
        }
    }

    pub fn successors(&self) -> HashSet<Block> {
        match *self.end.as_ref().unwrap() {
            Branch::Ret(ref ret) => set!(),
            Branch::Jmp(ref jmp) => set!(jmp.destination),
            Branch::Jnz(ref jnz) => set!(jnz.jnz, jnz.jmp),
        }
    }
}
