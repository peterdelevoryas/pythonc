use explicate::*;
use error::*;

pub mod func {
    use explicate::Var;
    use super::Block;

    impl_ref!(Func, "f");

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Data {
        pub args: Vec<Var>,
        pub body: Block,
    }
}
pub use self::func::Func;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    stmts: Vec<Stmt>,
}

pub struct TransUnit {
    main: Func,
    funcs: func::Slab<func::Data>,
}

pub struct Builder {
    // curr is a stack of blocks that are being created.
    // Each time a nested block is entered, the nested
    // block is pushed on top, and thus the top of the stack
    // is the actual current block. Each time a block
    // is exited, the current block is popped off the
    // stack and added to the slab of funcs.
    curr: Vec<Block>,
    funcs: func::Slab<func::Data>,
}

impl Builder {
    pub fn build(heapified: Module) -> TransUnit {
        let mut builder = Self {
            curr: vec![],
            funcs: func::Slab::new(),
        };
        builder.new_func();
        builder.add_to_curr_func(heapified.stmts);
        // no params for main function
        let main = builder.end_func(vec![]);
        TransUnit {
            main,
            funcs: builder.funcs,
        }
    }

    pub fn new_func(&mut self) {
        self.curr.push(Block { stmts: vec![] });
    }

    pub fn add_to_curr_func(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            let stmt = self.stmt(stmt);
        }
    }

    pub fn stmt(&mut self, s: Stmt) -> Stmt {
        unimplemented!()
    }

    // moves curr.last to funcs
    pub fn end_func(&mut self, params: Vec<Var>) -> Func {
        let curr = self.curr.pop().expect("end_block with empty curr");
        let data = func::Data {
            args: params,
            body: curr,
        };
        self.funcs.insert(data)
    }
}
