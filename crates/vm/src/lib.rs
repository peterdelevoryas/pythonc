extern crate python_ir as ir;
extern crate python_trans as trans;

use std::fmt;
use trans::Att;

#[derive(Debug, Copy, Clone)]
pub enum RVal {
    Int(i32),
    LVal(LVal),
}


#[derive(Debug, Copy, Clone)]
pub enum LVal {
    Tmp(ir::Tmp),
    Register(trans::Register),
    Stack(usize),
}

#[derive(Debug, Clone)]
pub enum Instr {
    Mov(RVal, LVal),
    Neg(LVal),
    Add(RVal, LVal),
    Push(RVal),
    Call(String),
}

impl From<ir::Val> for RVal {
    fn from(v: ir::Val) -> Self {
        match v {
            ir::Val::Int(i) => RVal::Int(i),
            ir::Val::Ref(t) => RVal::LVal(LVal::Tmp(t)),
        }
    }
}

impl fmt::Display for RVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RVal::*;
        match *self {
            Int(i) => write!(f, "{}", i),
            LVal(lval) => write!(f, "{}", lval),
        }
    }
}

impl fmt::Display for LVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::LVal::*;
        match *self {
            Tmp(tmp) => write!(f, "{}", tmp),
            Register(r) => write!(f, "{}", Att(&r)),
            Stack(index) => write!(f, "s{}", index),
        }
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instr::*;
        match *self {
            Mov(rval, lval) => write!(f, "mov {}, {}", rval, lval),
            Neg(lval) => write!(f, "neg {}", lval),
            Add(rval, lval) => write!(f, "add {}, {}", rval, lval),
            Push(rval) => write!(f, "push {}", rval),
            Call(ref s) => write!(f, "call {}", s),
        }
    }
}

impl Instr {
    fn replace_with(&mut self, tmp: ir::Tmp, new: LVal) {
        use self::Instr::*;
        use self::LVal::*;
        use self::RVal::*;
        match *self {
            Mov(Int(_), ref mut lval) => lval.replace_with(tmp, new),
            Mov(LVal(ref mut l), ref mut r) => {
                l.replace_with(tmp, new);
                r.replace_with(tmp, new);
            }
            Neg(ref mut lval) => lval.replace_with(tmp, new),
            Add(Int(_), ref mut lval) => lval.replace_with(tmp, new),
            Add(LVal(ref mut l), ref mut r) => {
                l.replace_with(tmp, new);
                r.replace_with(tmp, new);
            }
            Push(Int(_)) => {},
            Push(LVal(ref mut lval)) => lval.replace_with(tmp, new),
            Call(_) => {},
        }
    }

    pub fn replace_with_stack(&mut self, tmp: ir::Tmp, stack_index: usize) {
        unimplemented!()
    }
}

impl LVal {
    /// If this LVal is a Tmp, it is replaced with the new value,
    /// otherwise it is not modified
    fn replace_with(&mut self, tmp: ir::Tmp, new: LVal) {
        use self::LVal::*;
        use std::mem;
        match *self {
            Tmp(t) if t == tmp => {
                mem::replace(self, new);
            }
            _ => {}
        }
    }
}

pub struct Program {
    pub stack: Vec<Instr>,
}

impl Program {
    pub fn build(ir: &ir::Program) -> Program {
        let mut program = Program { stack: vec![] };
        for stmt in &ir.stmts {
            program.trans(stmt);
        }
        program
    }

    pub fn print(&self) {
        for instr in &self.stack {
            println!("{}", instr);
        }
    }

    pub fn spill(&mut self, tmp: ir::Tmp, stack_index: usize) {
        // instruction index
        let mut k = 0;
        loop {
            // If the instruction doesn't reference tmp, then
            // this won't modify the instruction
            self.stack[k].replace_with_stack(tmp, stack_index);
        }
        unimplemented!()
    }

    pub fn to_asm(self) -> trans::Program {
        unimplemented!()
    }

    pub fn replace_stack_to_stack_movs(&mut self, alloc: &mut ir::TmpAllocator) {
        unimplemented!()
    }

    ///
    /// ```
    /// tmp := l + r => {
    ///     mov l, tmp
    ///     add r, tmp
    /// }
    ///
    /// tmp := -v => {
    ///     mov v, tmp
    ///     neg tmp
    /// }
    ///
    /// tmp := input() => {
    ///     call input, eax // eax acts as destination operand implicitly,
    ///                     // this is hardcoded in liveness analysis
    ///     mov eax, tmp
    /// }
    ///
    /// print v => {
    ///     push v
    ///     call print_int_nl
    /// }
    /// ```
    ///
    fn trans(&mut self, stmt: &ir::Stmt) {
        use ir::Stmt::*;
        use ir::Expr::*;
        use ir::Val::*;
        match *stmt {
            Print(v) => {
                self.push(v.into());
                self.call("print_int_nl");
            }
            Def(tmp, Add(l, r)) => {
                self.mov(l.into(), LVal::Tmp(tmp));
                self.add(r.into(), LVal::Tmp(tmp));
            }
            Def(tmp, UnaryNeg(v)) => {
                self.mov(v.into(), LVal::Tmp(tmp));
                self.neg(LVal::Tmp(tmp));
            }
            Def(tmp, Input) => {
                self.call("input");
                let eax = RVal::LVal(LVal::Register(trans::Register::EAX));
                self.mov(eax, LVal::Tmp(tmp));
            }
        }
    }

    fn neg(&mut self, lval: LVal) {
        self.stack.push(Instr::Neg(lval));
    }

    fn add(&mut self, rval: RVal, lval: LVal) {
        self.stack.push(Instr::Add(rval, lval));
    }

    fn mov(&mut self, rval: RVal, lval: LVal) {
        self.stack.push(Instr::Mov(rval, lval));
    }

    fn push(&mut self, rval: RVal) {
        self.stack.push(Instr::Push(rval));
    }

    fn call(&mut self, s: &str) {
        self.stack.push(Instr::Call(s.into()));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
