extern crate python_ir as ir;
extern crate python_trans as trans;

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

pub enum Instr {
    Mov(Val, ir::Tmp),
    Neg(ir::Tmp),
    Add(Val, ir::Tmp),
    Push(Val),
    Call(String),
}

pub struct Program {
    stack: Vec<Instr>,
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
