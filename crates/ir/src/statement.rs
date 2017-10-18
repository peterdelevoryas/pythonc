use std::fmt;
use value::Val;
use expression::Expr;
use tmp::Tmp;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    Print(Val),
    Def(Tmp, Expr),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Stmt::*;
        match *self {
            Print(ref val) => write!(f, "print {}", val),
            Def(tmp, ref expr) => write!(f, "{} := {}", tmp, expr),
        }
    }
}
