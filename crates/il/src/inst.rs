use val::Val;
use val::Const;
use val::Slab as ValSlab;
use ty::Ty;
use std::fmt;
use slab::Slab;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Inst {

    /// mov l, dst
    /// add r, dst
    Add(Val, Val),

    /// mov val, dst
    /// neg dst
    Neg(Val),

    /// mov val, dst
    Copy(Val),

    /// mov imm, dst
    CopyI32(i32),

    /// $(push val)*
    /// call val
    /// add len(args) * 4
    Call(Val, Vec<Val>),
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
                _ => PyObj,
            },
            Eq | NotEq | Is => Bool,
        }
    }
}

impl Unop {
    pub fn ret_ty(&self, arg: Ty) -> Ty {
        use self::Unop::*;
        match *self {
            Neg => Ty::Int,
            Not => Ty::Bool,
            Copy => arg,
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
            Copy => write!(f, "copy"),
        }
    }
}
