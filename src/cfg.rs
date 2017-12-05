use explicate::Var;
use explicate::var;
use flatten::Expr;

/// Contains all data for a function
pub struct Function {
    pub args: Vec<Var>,
    pub cfg: Cfg,
}

#[derive(Debug)]
pub enum Stmt {
    Def {
        lhs: Var,
        rhs: Expr,
    },
    Discard(Expr),
}

/// A statement which terminates a basic block
pub enum Term {
    /// Return from function
    Return,
    /// Unconditional jump
    Goto(bb::BasicBlock),
    /// Conditional jump
    ///     if cond {
    ///         goto then;
    ///     } else {
    ///         goto else_;
    ///     }
    Switch {
        cond: Var,
        then: bb::BasicBlock,
        else_: bb::BasicBlock,
    }
}

/// Control flow graph
pub struct Cfg {
    bbs: bb::Slab<bb::Data>,
}

pub mod bb {
    use std::collections::HashSet;
    use super::Stmt;
    use super::Term;

    /// Basic block identifier
    impl_ref!(BasicBlock, "bb");

    /// Basic block data
    pub struct Data {
        /// Body of the basic block
        pub body: Vec<Stmt>,

        /// Last stmt in basic block,
        /// which results in leaving
        /// the basic block
        pub term: Term,

        /// Predecessors
        pub pred: HashSet<BasicBlock>,
    }
}

/// Builds a `Function` from a `flatten::Function`.
pub struct FuncBuilder<'var_data> {
    curr: bb::BasicBlock,
    bbs: bb::Slab<bb::Data>,
    var_data: &'var_data mut var::Slab<var::Data>,
}

impl<'var_data> FuncBuilder<'var_data> {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder() {
    }
}
