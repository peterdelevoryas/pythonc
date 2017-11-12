use explicate::*;
use error::*;

pub mod func {
    use explicate::Var;
    use super::Block;

    impl_ref!(Func, "f");

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Data {
        args: Vec<Var>,
        body: Block,
    }
}
pub use self::func::Func;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    stmts: Vec<Stmt>,
}

pub struct TransUnit {
    funcs: func::Slab<func::Data>,
}

impl TransUnit {
    
}
