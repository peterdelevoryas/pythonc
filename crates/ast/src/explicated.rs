// cond ? then : else =>
// 
// switch cond [bb0, bb1]
// bb0:
//     tmp = then
// bb1:
//     tmp = else
// ret tmp
//

use Statement;
use Expression;
use Target;
use Module;
use Program;
use std::ops::RangeFrom;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    If(Val, Block, Block),
    Expr(Expr),
    Def(Val, Expr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Call(Val, Vec<Val>),
    Binop(Binop, Val, Val),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Binop {
    Add,
    Not,
    And,
    Or,
    Is,
    Eq,
    NotEq,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Unop {
    Not,
    Neg,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Val {
    Var(String),
    Tmp(usize),
}


pub struct Builder {
    tmp_generator: RangeFrom<usize>,
}

impl Builder {
    fn new() -> Builder {
        Builder {
            tmp_generator: (0..)
        }
    }
}
