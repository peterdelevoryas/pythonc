use val::Val;
use inst::Inst;
use term::Term;

impl_index_type!(BasicBlock);

pub struct Data {
    defs: Vec<(Val, Inst)>,
    term: Term,
}
