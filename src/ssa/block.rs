use ssa::Value;
use ssa::Branch;
use std::collections::HashSet;

impl_ref!(Block, "block");
pub type BlockMap<T> = Slab<T>;

pub struct BlockData {
    pub body: Vec<Value>,
    /// Must be Some after construction
    pub end: Option<Branch>,
    pub predecessors: HashSet<Block>,
}
