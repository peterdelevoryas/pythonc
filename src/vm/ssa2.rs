use vm::FuncData;
use vm::Block;
use vm::BlockData;
use vm::VarEnv;
use std::collections::HashMap;
use vm::Var;
use vm::Inst;
use vm::InstData;

// SSA val
pub type Val = Var;

pub struct Builder {
    curr_def: HashMap<Var, HashMap<Block, Val>>,
    sealed: HashMap<Block, BlockData>,
    incomplete_phis: HashMap<Block, HashMap<Var, Block>>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            curr_def: HashMap::new(),
            sealed: HashMap::new(),
            incomplete_phis: HashMap::new(),
        }
    }

    pub fn write(&mut self, var: Var, block: Block, value: Var) {
        if !self.curr_def.contains_key(&var) {
            self.curr_def.insert(var, HashMap::new());
        }
        let inner = self.curr_def.get_mut(&var).unwrap();
        inner.insert(block, value);
    }

    pub fn read(&self, var: &Var, block: &Block) -> Var {
        if self.curr_def[var].contains_key(block) {
            return self.curr_def[var][block]
        }
        return self.read_recursive(var, block)
    }

    pub fn read_recursive(&self, var: &Var, block: &Block) -> Var {
        if !self.sealed.contains_key(block) {
            
        }
        unimplemented!()
    }
}

pub fn convert_to_ssa2(func: &mut FuncData, vars: &mut VarEnv) -> FuncData {
    let blocks = ::std::mem::replace(&mut func.blocks, HashMap::new());
    unimplemented!()
}
