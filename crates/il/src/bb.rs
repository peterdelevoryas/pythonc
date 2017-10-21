use val::Val;
use inst::Inst;
use term::Term;

impl_index_type!(BasicBlock);

pub struct Data {
    defs: Vec<(Val, Inst)>,
    term: Term,
}

// A basic block that has not finished being created yet
// Basically equivalent to Data but without the Term'inator
pub struct Partial {
    defs: Vec<(Val, Inst)>,
}

impl Partial {
    pub fn new() -> Partial {
        Partial { defs: vec![] }
    }
}
