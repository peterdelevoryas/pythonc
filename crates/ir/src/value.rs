use tmp::Tmp;
use std::fmt;

// Tmp(index) -> index of Tmp in stack
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Val {
    Const(i32, bool),
    PyObj(Tmp),
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Val::*;
        match *self {
            Const(i, b) => write!(f, "Const({}, {})", i, b),
            PyObj(tmp) => write!(f, "{}", tmp),
        }
    }
}
