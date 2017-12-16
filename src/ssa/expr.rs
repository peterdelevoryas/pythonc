use ssa::Rval;
use ssa::Func;
use ssa::Val;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Unary {
    Mov,
    Neg,
    Not,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Binary {
    Add,
    And,
    Or,
    Sete,
    Setne,
    Shr,
    Shl,
}

pub enum Expr {
    Unary { opcode: Unary, arg: Rval },
    Binary {
        opcode: Binary,
        left: Rval,
        right: Rval,
    },
    CallIndirect { target: Val, args: Vec<Rval> },
    Call { func: String, args: Vec<Rval> },

    /// XXX Oof! This is unfortunately here for now,
    /// a product of InjectFrom requiring two binary
    /// instructions
    ShiftLeftThenOr {
        arg: Rval,
        shift: i32,
        or: i32,
    },

    /// XXX Another oof!
    MovFuncLabel {
        func: Func,
    },

    Phi {
        vals: Vec<Val>,
    },

    LoadParam {
        /// func(x, y, z, ...)
        ///     x = LoadParam(0)
        ///     y = LoadParam(1)
        ///     z = LoadParam(2)
        ///     ...
        position: usize,
    }
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Unary::Mov => write!(f, "mov"),
            Unary::Neg => write!(f, "neg"),
            Unary::Not => write!(f, "not"),
        }
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Binary::Add => write!(f, "add"),
            Binary::And => write!(f, "and"),
            Binary::Or => write!(f, "or"),
            Binary::Sete => write!(f, "sete"),
            Binary::Setne => write!(f, "setne"),
            Binary::Shr => write!(f, "shr"),
            Binary::Shl => write!(f, "shl"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;
        use itertools::join;
        match *self {
            Unary { opcode, arg } => write!(f, "{} {}", opcode, arg),
            Binary { opcode, left, right } => write!(f, "{} {}, {}", opcode, left, right),
            CallIndirect { target, ref args } => {
                write!(f, "({})({})", target, join(args, ", "))
            }
            Call { ref func, ref args } => {
                write!(f, "{}({})", func, join(args, ", "))
            }
            ShiftLeftThenOr { arg, shift, or } => {
                write!(f, "({} << {}) | {}", arg, shift, or)
            }
            MovFuncLabel { func } => {
                write!(f, "${}", func)
            }
            Phi { ref vals } => {
                write!(f, "phi({})", join(vals, ", "))
            }
            LoadParam { position } => {
                write!(f, "load param {}", position)
            }
        }
    }
}
