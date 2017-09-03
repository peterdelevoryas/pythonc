//!
//! Below is a running example of how I think p0 should
//! translate to the IR in this module:
//!
//!     x = 1 + 2
//!     y = 3 + 4
//!     print x + y
//!     z = -x + 1 + y
//!     print z
//!
//!
//!     
//!
//!
//! In general, here are the mappings between `ast::Statement`s and `ir::Statement`s
//!
//!     Expression::DecimalI32(value) -> {
//!         tmpN = value
//!     }
//!
//!     Assign(Name, Expr) -> {
//!         tmpN = ir(expr)
//!     }
//!

use ast;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    statements: Vec<Statement>,
}

/// Refers to the result of a specific statement,
/// index is into Program.statements Vec
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Temporary {
    index: usize
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operand {
    Constant(i32),
    Temporary(Temporary),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    Add(Operand, Operand),
    UnaryNeg(Operand),
    Input,
    Print(Operand),
}

#[derive(Debug)]
pub struct Builder {
    stack: Vec<Statement>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder { stack: vec![] }
    }
}
