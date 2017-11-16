use explicate as ex;
use flatten as flat;
use raise;
use util::fmt;

use explicate::Var;

use std::collections::HashMap;

#[derive(Debug, Clone, Hash)]
pub enum Instr {
    Mov(Lval, Rval),
    Add(Lval, Rval),
    Neg(Lval),
    Push(Rval),
    Pop(Lval),
    CallIndirect(Rval),
    Call(String),
    If(Rval, Block, Block),
    /// `Lval - Rval, sets EQ and NE (and other) flags`
    Cmp(Lval, Rval),
    /// `Lval = EQ ? 1 : 0;`
    Sete(Lval),
    /// `Lval = NE ? 1 : 0;`
    Setne(Lval),
}

#[derive(Debug, Clone, Hash)]
pub struct Block {
    instrs: Vec<Instr>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Rval {
    Lval(Lval),
    Imm(Imm),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Lval {
    Reg(Reg),
    StackSlot(StackSlot),
    Var(Var),
}

pub type Imm = i32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Reg {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StackSlot {
    ebp_offset: i32,
}

pub struct Module {
    funcs: HashMap<raise::Func, Func>,
}

impl Module {
    pub fn from(f: flat::Flattener) -> Module {
        let mut funcs = HashMap::new();

        for (func, function) in f.units {
            funcs.insert(func, Func::from(function));
        }

        Module { funcs }
    }
}

pub struct Func {
    block: Block,
}

impl Func {
    pub fn from(f: flat::Function) -> Func {
        Func {
            block: Block::from(f.body),
        }
    }
}

impl Block {
    fn empty() -> Block {
        Block { instrs: vec![] }
    }

    pub fn from(stmts: Vec<flat::Stmt>) -> Block {
        let mut block = Block::empty();

        for stmt in stmts {
            block.stmt(stmt);
        }

        block
    }

    fn push_instr(&mut self, instr: Instr) {
        self.instrs.push(instr);
    }

    fn mov<L, R>(&mut self, lval: L, rval: R)
    where
        L: Into<Lval>,
        R: Into<Rval>,
    {
        let lval = lval.into();
        let rval = rval.into();
        self.push_instr(Instr::Mov(lval, rval));
    }

    fn neg<L>(&mut self, lval: L)
    where
        L: Into<Lval>,
    {
        let lval = lval.into();
        self.push_instr(Instr::Neg(lval));
    }

    fn push<R>(&mut self, rval: R)
    where
        R: Into<Rval>,
    {
        let rval = rval.into();
        self.push_instr(Instr::Push(rval));
    }

    fn call(&mut self, name: String) {
        self.push_instr(Instr::Call(name));
    }

    fn call_indirect<R>(&mut self, rval: R)
    where
        R: Into<Rval>,
    {
        self.push_instr(Instr::CallIndirect(rval.into()));
    }

    /// ```
    /// if lval == 0 {
    ///     lval = 1;
    /// } else {
    ///     lval = 0;
    /// }
    /// ```
    fn not<L>(&mut self, lval: L)
    where
        L: Into<Lval>,
    {
        let lval = lval.into();
        self.push_instr(Instr::Cmp(lval, 0.into()));
        self.push_instr(Instr::Sete(lval));
    }

    /// ```
    /// lval += rval;
    /// ```
    fn add<L, R>(&mut self, lval: L, rval: R)
    where
        L: Into<Lval>,
        R: Into<Rval>,
    {
        let lval = lval.into();
        let rval = rval.into();
        self.push_instr(Instr::Add(lval, rval));
    }

    /// ```
    /// if lval == rval {
    ///     lval = 1;
    /// } else {
    ///     lval = 0;
    /// }
    /// ```
    fn eq<L, R>(&mut self, lval: L, rval: R)
    where
        L: Into<Lval>,
        R: Into<Rval>,
    {
        let lval = lval.into();
        let rval = rval.into();
        self.push_instr(Instr::Cmp(lval, rval));
        self.push_instr(Instr::Sete(lval));
    }

    /// ```
    /// if lval != rval {
    ///     lval = 1;
    /// } else {
    ///     lval = 0;
    /// }
    /// ```
    fn not_eq<L, R>(&mut self, lval: L, rval: R)
    where
        L: Into<Lval>,
        R: Into<Rval>,
    {
        let lval = lval.into();
        let rval = rval.into();
        self.push_instr(Instr::Cmp(lval, rval));
        self.push_instr(Instr::Setne(lval));
    }

    /// Sets compare flags like EQ and NE
    /// that can be read by `sete` and `setne`
    fn cmp<L, R>(&mut self, lval: L, rval: R)
    where
        L: Into<Lval>,
        R: Into<Rval>,
    {
        let lval = lval.into();
        let rval = rval.into();
        self.push_instr(Instr::Cmp(lval, rval));
    }


    fn stmt(&mut self, stmt: flat::Stmt) {
        match stmt {
            flat::Stmt::Def(lhs, flat::Expr::UnaryOp(op, rhs)) => {
                self.mov(lhs, rhs);
                match op {
                    flat::UnaryOp::NEGATE => self.neg(lhs),
                    flat::UnaryOp::NOT => self.not(lhs),
                }
            }
            flat::Stmt::Def(lhs, flat::Expr::BinOp(op, l, r)) => {
                self.mov(lhs, l);
                match op {
                    flat::BinOp::ADD => self.add(lhs, r),
                    flat::BinOp::EQ => self.eq(lhs, r),
                    flat::BinOp::NOTEQ => self.not_eq(lhs, r),
                }
            }
            flat::Stmt::Discard(expr) => {
            }
            flat::Stmt::Return(var) => {
            }
            flat::Stmt::If(cond, then, else_) => {

            }
            _ => {}
        }
    }
}

impl fmt::Fmt for Module {
    fn fmt<W: ::std::io::Write>(&self, out: &mut fmt::Formatter<W>) -> ::std::io::Result<()> {
        use std::io::Write;

        writeln!(out, "vasm {{")?;
        for (f, func) in &self.funcs {
            if f.is_main_func() {
                writeln!(out, "main:")?;
            } else {
                writeln!(out, "{}:", f)?;
            }
            out.indent();
            out.fmt(func)?;
            out.dedent();
        }
        writeln!(out, "}}")?;
        Ok(())
    }
}

impl fmt::Fmt for Func {
    fn fmt<W: ::std::io::Write>(&self, f: &mut fmt::Formatter<W>) -> ::std::io::Result<()> {
        f.fmt(&self.block)?;
        Ok(())
    }
}

impl fmt::Fmt for Block {
    fn fmt<W: ::std::io::Write>(&self, f: &mut fmt::Formatter<W>) -> ::std::io::Result<()> {
        for instr in &self.instrs {
            f.fmt(instr)?;
        }
        Ok(())
    }
}

impl fmt::Fmt for Instr {
    fn fmt<W: ::std::io::Write>(&self, f: &mut fmt::Formatter<W>) -> ::std::io::Result<()> {
        use std::io::Write;

        match *self {
            Instr::Mov(_, _) => {
                write!(f, "mov _, _")?;
            }
            _ => unimplemented!(),
        }
        Ok(())
    }
}

impl From<Reg> for Lval {
    fn from(r: Reg) -> Self {
        Lval::Reg(r)
    }
}

impl From<StackSlot> for Lval {
    fn from(s: StackSlot) -> Self {
        Lval::StackSlot(s)
    }
}

impl From<Var> for Lval {
    fn from(v: Var) -> Self {
        Lval::Var(v)
    }
}

impl<L> From<L> for Rval
where
    L: Into<Lval>
{
    fn from(l: L) -> Self {
        let lval = l.into();
        Rval::Lval(lval)
    }
}

impl From<Imm> for Rval {
    fn from(i: Imm) -> Self {
        Rval::Imm(i)
    }
}
