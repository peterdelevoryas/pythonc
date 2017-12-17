use ssa::Val;
use std::fmt;
use ssa::Func;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Rval {
    Val(Val),
    Imm(i32),
    Func(Func),
}

impl fmt::Display for Rval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rval::Val(val) => write!(f, "{}", val),
            Rval::Imm(int) => write!(f, "${}", int),
            Rval::Func(func) => write!(f, "{}", func),
        }
    }
}
