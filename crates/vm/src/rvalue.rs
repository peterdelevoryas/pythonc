use LValue;
use ir;

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum RValue {
    Int(i32),
    LValue(LValue),
}

impl From<ir::Val> for RValue {
    fn from(v: ir::Val) -> Self {
        match v {
            ir::Val::Const(i, _) => RValue::Int(i),
            ir::Val::PyObj(t) => RValue::LValue(LValue::Tmp(t)),
        }
    }
}

impl fmt::Display for RValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RValue::*;
        match *self {
            Int(i) => write!(f, "${}", i),
            LValue(lval) => write!(f, "{}", lval),
        }
    }
}
