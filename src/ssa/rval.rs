use ssa::Val;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Rval {
    Val(Val),
    Imm(i32),
}

impl fmt::Display for Rval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rval::Val(val) => write!(f, "{}", val),
            Rval::Imm(int) => write!(f, "${}", int),
        }
    }
}
