use bb::{self, BasicBlock};
use val::{self, Val};
use slab::Slab;

pub struct Func {
    bbs: Slab<BasicBlock, bb::Data>,
    vals: Slab<Val, val::Data>,
}
