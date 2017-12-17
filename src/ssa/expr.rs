use ssa::Function;
use ssa::Value;
use std::fmt;
use ssa::Block;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CallTarget {
    Runtime(&'static str),
    Direct(Function),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Unary { opcode: Unary, arg: Value },
    Binary {
        opcode: Binary,
        left: Value,
        right: Value,
    },
    Call { target: CallTarget, args: Vec<Value> },

    /// XXX Oof! This is unfortunately here for now,
    /// a product of InjectFrom requiring two binary
    /// instructions
    ShiftLeftThenOr {
        arg: Value,
        shift: i32,
        or: i32,
    },

    Phi {
        block: Block,
        args: Vec<Value>,
    },

    LoadParam {
        /// func(x, y, z, ...)
        /// root:
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
            Call { target, ref args } => {
                match target {
                    CallTarget::Runtime(func_name) => {
                        write!(f, "call @{}({})", func_name, join(args, ", "))
                    }
                    CallTarget::Direct(func) => {
                        write!(f, "call {}({})", func, join(args, ", "))
                    }
                }
            }
            ShiftLeftThenOr { arg, shift, or } => {
                write!(f, "({} << {}) | {}", arg, shift, or)
            }
            Phi { block, ref args } => {
                write!(f, "{}.phi({})", block, join(args, ", "))
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
