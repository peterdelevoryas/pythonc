use ssa::Inst;
use ssa::Term;
use std::collections::HashSet;
use std::iter;

impl_ref!(Block, "bb");

pub struct BlockData {
    pub body: Vec<Inst>,
    pub term: Option<Term>,
    pub pred: HashSet<Block>,
}

pub type BlockGen = Gen;

impl BlockData {
    pub fn new() -> Self {
        Self {
            body: vec![],
            term: None,
            pred: set!(),
        }
    }

    pub fn predecessors(&self) -> impl Iterator<Item=Block> {
        self.pred.clone().into_iter()
    }

    pub fn successors(&self) -> Box<Iterator<Item=Block>> {
        match *self.term.as_ref().expect("successors called on block without term") {
            Term::Ret { .. } => Box::new(iter::empty()),
            Term::Goto { block } => Box::new(iter::once(block)),
            Term::Switch { then, else_, .. } => {
                let iter = iter::once(then).chain(iter::once(else_));
                Box::new(iter)
            }
        }
    }
}
