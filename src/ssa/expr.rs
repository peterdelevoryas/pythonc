use ssa::Rval;
use ssa::Func;
use ssa::Val;
use ssa::Block;
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

#[derive(Debug, Clone)]
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
        block: Block,
        vals: Vec<Val>,
    },

    LoadParam {
        /// func(x, y, z, ...)
        ///     x = LoadParam(0)
        ///     y = LoadParam(1)
        ///     z = LoadParam(2)
        ///     ...
        position: usize,
    },

    Undef,
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
            Phi { block, ref vals } => {
                write!(f, "{}.phi({})", block, join(vals, ", "))
            }
            LoadParam { position } => {
                write!(f, "load param {}", position)
            }
            Undef => {
                write!(f, "undefined")
            }
        }
    }
}

use flatten::UnaryOp;
impl From<UnaryOp> for Unary {
    fn from(op: UnaryOp) -> Unary {
        match op {
            UnaryOp::NEGATE => Unary::Neg,
            UnaryOp::NOT => Unary::Not,
        }
    }
}

use flatten::BinOp;
impl From<BinOp> for Binary {
    fn from(op: BinOp) -> Binary {
        match op {
            BinOp::ADD => Binary::Add,
            BinOp::EQ => Binary::Sete,
            BinOp::NOTEQ => Binary::Setne,
        }
    }
}
