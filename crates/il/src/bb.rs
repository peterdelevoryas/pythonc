use val::Val;
use inst::Inst;
use term::Term;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Label(usize);

pub struct BB {
    defs: Vec<(Val, Inst)>,
    term: Term,
}
