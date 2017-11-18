use vasm::Reg;
use vasm::Lval;
use vasm::Inst;
use vasm::Function;
use explicate::Var;
use std::collections::HashSet;

/// Computes live sets for each instruction
/// in a function
pub fn liveness(f: &Function) -> Vec<LiveSet> {
    let mut live_sets: Vec<LiveSet> = Vec::new();
    let mut live_after: HashSet<Lval> = HashSet::new();
    let mut live_before: HashSet<Lval>;

    for inst in f.block.insts.iter().rev() {
        //live_before = 
    }

    live_sets
}

#[derive(Debug, Clone)]
pub struct LiveSet<'inst> {
    pub inst: &'inst Inst,
    pub live_after: HashSet<Lval>,
}
