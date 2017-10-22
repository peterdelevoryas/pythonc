use val::Val;
use val::Const;
use val::Slab as ValSlab;
use ty::Ty;
use std::fmt;
use slab::Slab;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Inst {
    Binop(Binop, Arg, Arg),
    Unop(Unop, Arg),
    Call {
        func: Arg,
        args: Vec<Arg>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Binop {
    Add,
    And,
    Or,
    Eq,
    NotEq,
    Is,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Unop {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Arg {
    Loc(Val),
    Const(Const),
}

impl From<Val> for Arg {
    fn from(v: Val) -> Arg {
        Arg::Loc(v)
    }
}

impl From<Const> for Arg {
    fn from(c: Const) -> Arg {
        Arg::Const(c)
    }
}

impl Binop {
    pub fn ret_ty(&self, l: Ty, r: Ty) -> Ty {
        use self::Binop::*;
        use self::Ty::*;
        match *self {
            Add => match (l, r) {
                (List, List) => List,
                (Dict, _) | (_, Dict) =>
                    panic!("Invalid arg types in binop: {} {} {}", Add, l, r),
                _ => Ty::Int,
            },
            And | Or => match (l, r) {
                (List, List) => List,
                (Dict, Dict) => Dict,
                (Int, Int) => Int,
                (Bool, Bool) => Bool,
                _ => Any,
            },
            Eq | NotEq | Is => Bool,
        }
    }
}

impl Unop {
    pub fn ret_ty(&self, _arg: Ty) -> Ty {
        use self::Unop::*;
        match *self {
            Neg => Ty::Int,
            Not => Ty::Bool,
        }
    }
}

impl Arg {
    pub fn ty(&self, vals: &ValSlab) -> Ty {
        match *self {
            Arg::Const(ref c) => c.ty(),
            Arg::Loc(v) => vals[v].ty(),
        }
    }
} 
impl Inst {
    pub fn ret_ty(&self, vals: &ValSlab) -> Ty {
        use self::Inst::*;
        match *self {
            Binop(op, ref l, ref r) => op.ret_ty(l.ty(vals), r.ty(vals)),
            Unop(op, ref val) => op.ret_ty(val.ty(vals)),
            Call {
                ref func,
                ref args,
            } => {
                func.ty(vals)
            }
        }
    }
}

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Inst::Binop(op, ref l, ref r) => {
                write!(f, "{} {} {}", op, l, r)
            }
            Inst::Unop(op, ref arg) => {
                write!(f, "{} {}", op, arg)
            }
            Inst::Call {
                ref func,
                ref args,
            } => {
                write!(f, "call ")?;
                match *func {
                    Arg::Const(Const::Func(func)) => {
                        write!(f, "{}", func.name)?;
                    }
                    _ => {
                        write!(f, "({})", func)?;
                    }
                    
                }
                write!(f, "(")?;
                if !args.is_empty() {
                    write!(f, "{}", args[0])?;
                    for arg in &args[1..] {
                        write!(f, ", {}", arg)?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

impl fmt::Display for Binop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Binop::*;
        let binop = match *self {
            Add => "add",
            And => "and",
            Or => "or",
            Eq => "eq",
            NotEq => "noteq",
            Is => "is",
        };
        write!(f, "{}", binop)
    }
}

impl fmt::Display for Unop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Unop::*;
        match *self {
            Not => write!(f, "not"),
            Neg => write!(f, "neg"),
        }
    }
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Arg::Const(ref c) => write!(f, "{}", c),
            Arg::Loc(v) => write!(f, "{}", v),
        }
    }
}
