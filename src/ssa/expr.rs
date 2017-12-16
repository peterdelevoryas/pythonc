use ssa::Rval;
use ssa::Func;
use ssa::Val;

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
    }
}
