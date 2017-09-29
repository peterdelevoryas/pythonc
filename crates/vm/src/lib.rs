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
            }
            Def(t, ref e) => {

            }
        }
    }

    fn push(&mut self, val: Val) {
        self.stack.push(Instr::Push(val));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
