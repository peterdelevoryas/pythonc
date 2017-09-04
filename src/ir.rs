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
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    statements: Vec<Statement>,
}

pub type Index = usize;

/// Def(index) -> index of Def in stack
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operand {
    Constant(i32),
    Def(Index),
}

///
///     tmp8 = tmp7     ; DefIndex
///     tmp0 = 1        ; Constant
///     tmp1 = -1       ; UnaryNeg(Constant)
///     tmp2 = -tmp1    ; UnaryNeg(DefIndex)
///     tmp3 = 1 + 2    ; Add(Constant, Constant)
///     tmp4 = 1 + tmp3 ; Add(Constant, Def)
///     tmp5 = tmp4 + 1     ; Add(Def, Constant)
///     tmp6 = tmp4 + tmp5  ; Add(Def, Def)
///     tmp7 = input()  ; Input
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Def {
    Assign(Operand),
    UnaryNeg(Operand),
    Add(Operand, Operand),
    Input,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    Print(Operand),
    Def(Def),
}

#[derive(Debug)]
pub struct Builder {
    stack: Vec<Statement>,
    names: HashMap<ast::Name, Def>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            stack: vec![],
            names: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
}
