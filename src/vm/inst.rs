use std::fmt;
use vm::Reg;
use vm::StackSlot;
use vm::Var;
use vm::Func;
use cfg::Stmt;
use flatten::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Inst {
    pub dst: Lval,
    pub data: Data,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Unary {
    Mov,
    Neg,
    Not,
    Push,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Binary {
    Add,
    Sub,
    Sete,
    Setne,
    Or,
    And,
    Shr,
    Shl,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Data {
    Unary { opcode: Unary, arg: Rval },
    Binary {
        opcode: Binary,
        left: Rval,
        right: Rval,
    },
    CallIndirect { target: Lval, args: Vec<Rval> },
    Call { func: String, args: Vec<Rval> },

    /// XXX Oof! This is unfortunately here for now,
    /// a product of InjectFrom requiring two binary
    /// instructions
    ShiftLeftThenOr {
        arg: Rval,
        shift: Imm,
        or: Imm,
    },

    /// XXX Another oof!
    MovFuncLabel {
        func: Func,
    }
}

pub type Imm = i32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lval {
    Reg(Reg),
    StackSlot(StackSlot),
    Var(Var),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Rval {
    Imm(Imm),
    Lval(Lval),
}

impl Inst {
    pub fn call_indirect(target: Lval, args: Vec<Rval>) -> Data {
        Data::CallIndirect { target, args }
    }

    pub fn call(func: String, args: Vec<Rval>) -> Data {
        Data::Call { func, args }
    }

    pub fn unary(opcode: Unary, arg: Rval) -> Data {
        Data::Unary { opcode, arg }
    }

    pub fn is_stack_to_stack(&self) -> bool {
        use vm::Rval::*;
        use vm::Lval::*;
        match self.dst {
            StackSlot(_) => {}
            _ => return false,
        }
        use self::Data::*;
        match self.data {
            Unary { arg: Lval(StackSlot(_)), .. } |
            Binary { left: Lval(StackSlot(_)), .. } |
            Binary { right: Lval(StackSlot(_)), .. } |
            ShiftLeftThenOr { arg: Lval(StackSlot(_)), .. } => true,
            _ => false,
        }
    }
}

impl Data {
    pub fn dst(self, dst: Lval) -> Inst {
        Inst {
            dst: dst,
            data: self,
        }
    }
}

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.dst, self.data)
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Data::Unary { ref opcode, ref arg } => {
                write!(f, "{} {}", opcode, arg)
            }
            Data::Binary { ref opcode, ref left, ref right } => {
                write!(f, "{} {}, {}", opcode, left, right)
            }
            Data::CallIndirect { ref target, ref args } => {
                write!(f, "call* {}({})", target, ::itertools::join(args, ", "))
            }
            Data::Call { ref func, ref args } => {
                write!(f, "call {}({})", func, ::itertools::join(args, ", "))
            }
            Data::ShiftLeftThenOr { ref arg, ref shift, ref or } => {
                write!(f, "({} << ${}) | ${}", arg, shift, or)
            }
            Data::MovFuncLabel { ref func } => {
                write!(f, "mov ${}", func)
            }
        }
    }
}

impl fmt::Display for Lval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Lval::Var(ref var) => write!(f, "{}", var),
            Lval::StackSlot(ref stack_slot) => write!(f, "{}", stack_slot),
            Lval::Reg(ref reg) => write!(f, "{}", reg),
        }
    }
}

impl fmt::Display for Rval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rval::Lval(ref lval) => write!(f, "{}", lval),
            Rval::Imm(ref imm) => write!(f, "${}", imm),
        }
    }
}

impl Unary {
    pub fn as_str(&self) -> &'static str {
        use self::Unary::*;
        match *self {
            Mov => "movl",
            Neg => "negl",
            Not => "notl",
            Push => "pushl",
        }
    }
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Binary {
    pub fn as_str(&self) -> &'static str {
        use self::Binary::*;
        match *self {
            Add => "addl",
            Sub => "subl",
            Sete => "sete",
            Setne => "setne",
            Or => "or",
            And => "and",
            Shr => "shr",
            Shl => "shl",
        }
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
