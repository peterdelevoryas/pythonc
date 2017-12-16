use ssa::Val;
use ssa::Expr;

pub struct Inst {
    pub def: Val,
    pub expr: Expr,
}
