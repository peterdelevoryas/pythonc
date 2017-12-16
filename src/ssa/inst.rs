use ssa::Val;
use ssa::Expr;
use std::fmt;

pub struct Inst {
    pub def: Val,
    pub expr: Expr,
}

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.def, self.expr)
    }
}
