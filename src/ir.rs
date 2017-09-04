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

/// Def(index) -> index of Def in stack
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Operand {
    Constant(i32),
    Def(usize),
}

///
///     tmp8 = tmp7     ; Copy(Def)
///     tmp0 = 1        ; Copy(Constant)
///     tmp1 = -1       ; UnaryNeg(Constant)
///     tmp2 = -tmp1    ; UnaryNeg(Def)
///     tmp3 = 1 + 2    ; Add(Constant, Constant)
///     tmp4 = 1 + tmp3 ; Add(Constant, Def)
///     tmp5 = tmp4 + 1     ; Add(Def, Constant)
///     tmp6 = tmp4 + tmp5  ; Add(Def, Def)
///     tmp7 = input()  ; Input
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Def {
    Copy(Operand),
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
    names: HashMap<ast::Name, Operand>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            stack: vec![],
            names: HashMap::new(),
        }
    }

    pub fn expr_to_operand(&mut self, expr: &ast::Expression) -> Operand {
        match *expr {
            ast::Expression::DecimalI32(ast::DecimalI32(i)) => {
                Operand::Constant(i)
            }
            ast::Expression::Name(ref name) => {
                *self.names.get(name).expect("reference to undefined name")
            }
            _ => unimplemented!()
        }
    }

    pub fn handle_statement(&mut self, stmt: &ast::Statement) {
        match *stmt {
            ast::Statement::Print(ref expr) => {
                
            }
            ast::Statement::Assign(ref name, ref expr) => {

            }
            ast::Statement::Expression(ref expr) => {

            }
            ast::Statement::Newline => {},
        }
    }
}

#[cfg(test)]
mod test {
}
