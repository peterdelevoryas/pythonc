use std::collections::HashMap;
use std::collections::HashSet;
use vm::Func;
use vm::FuncData;
use vm::Block;
use vm::BlockData;
use vm::Var;
use vm::Lval;

pub type Lvals = HashSet<Lval>;

pub struct Liveness<'func_data> {
    func_data: &'func_data FuncData,

    gens: HashMap<Block, Lvals>,
    kills: HashMap<Block, Lvals>,
    in_: HashMap<Block, Lvals>,
    out: HashMap<Block, Lvals>,
}

impl<'func_data> Liveness<'func_data> {
    pub fn new(func_data: &'func_data FuncData) -> Self {
        unimplemented!()
    }
}
