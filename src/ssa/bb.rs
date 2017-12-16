use ssa::Inst;
use ssa::Term;

impl_ref!(Block, "bb");

pub struct BlockData {
    pub body: Vec<Inst>,
    pub term: Option<Term>,
}

pub type BlockGen = Gen;
