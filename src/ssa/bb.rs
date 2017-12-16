use ssa::Inst;
use ssa::Term;
use ssa::Val;
use std::collections::HashSet;
use std::iter;

impl_ref!(Block, "bb");

#[derive(Clone)]
pub struct BlockData {
    pub body: Vec<Val>,
    pub term: Option<Term>,
    pub preds: HashSet<Block>,
}

pub type BlockGen = Gen;

impl BlockData {
    pub fn new() -> Self {
        Self {
            body: vec![],
            term: None,
            preds: set!(),
        }
    }

    pub fn preds_iter(&self) -> impl Iterator<Item=Block> {
        self.preds.clone().into_iter()
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
