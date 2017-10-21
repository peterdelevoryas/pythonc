use bb::{self, BasicBlock};
use val::{self, Val};
use slab::Slab;
use std::collections::HashMap;

pub struct Func {
    bbs: Slab<BasicBlock, bb::Data>,
    vals: Slab<Val, val::Data>,
}

impl Func {
    pub fn builder() -> Builder {
        Builder::new()
    }
}

pub struct Builder {
    curr: bb::Partial,
    bbs: Slab<BasicBlock, bb::Data>,
    vals: Slab<Val, val::Data>,
    names: HashMap<String, Val>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            curr: bb::Partial::new(),
            bbs: Slab::new(),
            vals: Slab::new(),
            names: HashMap::new(),
        }
    }
}
