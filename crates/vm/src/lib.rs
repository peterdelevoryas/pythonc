extern crate python_ir as ir;
extern crate python_trans as trans;

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Val {
    Virtual(ir::Tmp),
    Const(i32),
    Register(trans::Register),
}

impl From<ir::Val> for Val {
    fn from(v: ir::Val) -> Self {
        match v {
            ir::Val::Int(i) => Val::Const(i), ir::Val::Ref(t) => Val::Virtual(t),
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Val::*;
        match *self {
            Virtual(tmp) => write!(f, "{}", tmp),
            Const(i) => write!(f, "{}", i),
            Register(r) => write!(f, "{}", trans::Att(&r)),
        }
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instr::*;
        match *self {
            Mov(val, tmp) => write!(f, "mov {}, {}", val, tmp),
            Neg(tmp) => write!(f, "neg {}", tmp),
            Add(val, tmp) => write!(f, "add {}, {}", val, tmp),
            Push(val) => write!(f, "push {}", val),
            Call(ref s) => write!(f, "call {}", s),
        }
    }
}

pub enum Instr {
    Mov(Val, ir::Tmp),
    Neg(ir::Tmp),
    Add(Val, ir::Tmp),
    Push(Val),
    Call(String),
}

pub struct Program {
    pub stack: Vec<Instr>,
}

impl Program {
    pub fn build(ir: &ir::Program) -> Program {
        let mut program = Program {
            stack: vec![],
        };
        for stmt in &ir.stmts {
            program.trans(stmt);
        }
        program
    }

    pub fn print(&self) {
        for instr in &self.stack {
            use self::Instr::*;
            match *instr {
                Mov(val, tmp) => println!("mov {}, {}", val, tmp),
                Neg(tmp) => println!("neg {}", tmp),
                Add(val, tmp) => println!("add {}, {}", val, tmp),
                Push(val) => println!("push {}", val),
                Call(ref label) => println!("call {}", label),
            }
        }
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
    ///     call input
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
                self.mov(l.into(), tmp);
                self.add(r.into(), tmp);
            }
            Def(tmp, UnaryNeg(v)) => {
                self.mov(v.into(), tmp);
                self.neg(tmp);
            }
            Def(tmp, Input) => {
                self.call("input");
                let eax = Val::Register(trans::Register::EAX);
                self.mov(eax, tmp);
            }
        }
    }

    fn neg(&mut self, tmp: ir::Tmp) {
        self.stack.push(Instr::Neg(tmp));
    }

    fn add(&mut self, val: Val, tmp: ir::Tmp) {
        self.stack.push(Instr::Add(val, tmp));
    }

    fn mov(&mut self, val: Val, tmp: ir::Tmp) {
        self.stack.push(Instr::Mov(val, tmp));
    }

    fn push(&mut self, val: Val) {
        self.stack.push(Instr::Push(val));
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
