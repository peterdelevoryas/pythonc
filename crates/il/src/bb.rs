use val::Val;
use inst::Inst;
use term::Term;
use std::fmt;

impl_index_type!(BasicBlock);

#[derive(Debug)]
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

    pub fn push(&mut self, val: Val, inst: Inst) {
        self.defs.push((val, inst));
    }

    pub fn ret(self) -> Data {
        Data {
            defs: self.defs,
            term: Term::Return,
        }
    }
}

impl Data {
    pub fn defs(&self) -> &[(Val, Inst)] {
        &self.defs
    }

    pub fn term(&self) -> &Term {
        &self.term
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bb{}", self.0)
    }
}
