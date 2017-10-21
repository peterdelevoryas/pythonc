use bb::{self, BasicBlock};
use val::{self, Val};
use slab::Slab;
use std::collections::HashMap;
use ast;

pub struct Func {
    bbs: Slab<BasicBlock, bb::Data>,
    vals: Slab<Val, val::Data>,
}

impl Func {
    pub fn build(m: &ast::Module) -> Func {
        let mut b = Builder::new();
        b.build(m)
    }
}

pub struct Builder {
    curr: bb::Partial,
    bbs: Slab<BasicBlock, bb::Data>,
    vals: Slab<Val, val::Data>,
    names: HashMap<String, Val>,
}

impl Builder {
    fn new() -> Builder {
        Builder {
            curr: bb::Partial::new(),
            bbs: Slab::new(),
            vals: Slab::new(),
            names: HashMap::new(),
        }
    }

    fn build(self, m: &ast::Module) -> Func {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use func;
    use ast;

    #[test]
    fn flatten_target() {
        unimplemented!()
    }

    #[test]
    fn flatten_statement() {
        unimplemented!()
    }

    #[test]
    fn flatten_expression() {
        unimplemented!()
    }
}
