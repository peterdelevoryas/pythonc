use std::fmt;
use value::Val;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    UnaryNeg(Val),
    Add(Val, Val),
    PolyEqv(Val, Val),
    Not(Val),
    Eq(Val, Val),
    PolyUnEqv(Val, Val),
    And(Val, Val),
    Or(Val, Val),
    If(Val, Val, Val),
    FunCall(String, Vec<Val>),
    Subscript(Val, Val),
    Inject(Val),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self {
            UnaryNeg(ref val) => write!(f, "-{}", val),
            Add(ref l, ref r) => write!(f, "{} + {}", l, r),
            PolyEqv(ref l, ref r) => write!(f, "{} == {}", l, r),
            Not(ref val) => write!(f, "not {}", val),
            Eq(ref l, ref r) => write!(f, "{} is {}", l, r),
            PolyUnEqv(ref l, ref r) => write!(f, "{} != {}", l, r),
            And(ref l, ref r) => write!(f, "{} and {}", l, r),
            Or(ref l, ref r) => write!(f, "{} or {}", l, r),
            If(ref test, ref then, ref els) => write!(f, "{} if {} else {}", then, test, els),
            FunCall(ref name, ref args) => {
                let args: Vec<String> = args.iter().map(|arg| format!("{}", arg)).collect();
                let args: String = args.join(", ");
                write!(f, "{}({})", name, args)
            }
            Subscript(ref target, ref index) => write!(f, "{}[{}]", target, index),
            Inject(ref val) => write!(f, "__inject({})", val),
        }
    }
}


