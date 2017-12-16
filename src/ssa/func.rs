use ssa::Block;
use ssa::BlockData;
use std::collections::HashMap;

impl_ref!(Func, "fn");

pub struct FuncData {
    pub name: Func,
    pub main: bool,
    pub blocks: HashMap<Block, BlockData>,
}

pub type FuncGen = Gen;
