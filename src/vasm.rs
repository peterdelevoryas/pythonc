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
    CallIndirect(Lval),
    Call(String),
    If(Rval, Block, Block),
    /// `Lval - Rval, sets EQ and NE (and other) flags`
    Cmp(Lval, Rval),
    /// `Lval = EQ ? 1 : 0;`
    Sete(Lval),
    /// `Lval = NE ? 1 : 0;`
    Setne(Lval),
    Or(Lval, Rval),
    And(Lval, Rval),
    /// I think `shr` requires arg to be
    /// in CL, which complicates instruction
    /// selection, so for now this only allows
    /// `Imm`'s, which is all we need
    Shr(Lval, Imm),
    /// See doc for `Shr` for why this only allows
    /// `Imm`
    Shl(Lval, Imm),
    /// `mov lval, $func`
    MovLabel(Lval, raise::Func),
    /// Just `ret`, nothing else
    Ret,
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

/// Does not contain the EBP ESP
/// frame pointer storage shim at the top
/// or stack allocation sub instr,
/// must be added when finally formatting to x86
pub struct Func {
    // increases with spillage
    num_stack_slots: u32,
    block: Block,
}

impl Func {
    pub fn from(f: flat::Function) -> Func {
        Func {
            num_stack_slots: 0,
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

    /// ```
    /// push arg[n]
    /// push arg[n - 1]
    /// ...
    /// push arg[1]
    /// push arg[0]
    /// ```
    fn push_args_in_reverse(&mut self, args: Vec<Var>) {
        for arg in args.into_iter().rev() {
            self.push(arg);
        }
    }

    fn call_indirect<L>(&mut self, lval: L)
    where
        L: Into<Lval>,
    {
        self.push_instr(Instr::CallIndirect(lval.into()));
    }

    fn call_direct(&mut self, name: String) {
        self.push_instr(Instr::Call(name));
    }

    fn and<L, R>(&mut self, lval: L, rval: R)
    where
        L: Into<Lval>,
        R: Into<Rval>,
    {
        self.push_instr(Instr::And(lval.into(), rval.into()));
    }

    fn or<L, R>(&mut self, lval: L, rval: R)
    where
        L: Into<Lval>,
        R: Into<Rval>,
    {
        self.push_instr(Instr::Or(lval.into(), rval.into()));
    }

    fn shr<L>(&mut self, lval: L, imm: Imm)
    where
        L: Into<Lval>,
    {
        self.push_instr(Instr::Shr(lval.into(), imm));
    }

    fn shl<L>(&mut self, lval: L, imm: Imm)
    where
        L: Into<Lval>,
    {
        self.push_instr(Instr::Shl(lval.into(), imm));
    }

    fn mov_label<L>(&mut self, lval: L, func: raise::Func)
    where
        L: Into<Lval>,
    {
        self.push_instr(Instr::MovLabel(lval.into(), func));
    }

    fn pop<L>(&mut self, lval: L)
    where
        L: Into<Lval>,
    {
        self.push_instr(Instr::Pop(lval.into()));
    }

    fn ret(&mut self) {
        self.push_instr(Instr::Ret);
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
            flat::Stmt::Def(lhs, flat::Expr::CallFunc(f, args)) => {
                self.push_args_in_reverse(args);
                self.call_indirect(f);
                self.mov(lhs, Reg::EAX);
            }
            flat::Stmt::Def(lhs, flat::Expr::RuntimeFunc(name, args)) => {
                self.push_args_in_reverse(args);
                self.call_direct(name);
                self.mov(lhs, Reg::EAX);
            }
            flat::Stmt::Def(lhs, flat::Expr::GetTag(var)) => {
                self.mov(lhs, var);
                // MASK = 3
                self.and(lhs, ex::MASK);
            }
            flat::Stmt::Def(lhs, flat::Expr::ProjectTo(var, _)) => {
                self.mov(lhs, var);
                self.shr(lhs, ex::SHIFT);
            }
            flat::Stmt::Def(lhs, flat::Expr::InjectFrom(var, ty)) => {
                self.mov(lhs, var);
                match ty {
                    ex::Ty::Int => {
                        self.shl(lhs, ex::SHIFT);
                        self.or(lhs, ex::INT_TAG);
                    }
                    ex::Ty::Bool => {
                        self.shl(lhs, ex::SHIFT);
                        self.or(lhs, ex::BOOL_TAG);
                    }
                    ex::Ty::Big => {
                        self.or(lhs, ex::BIG_TAG);
                    }
                    ex::Ty::Pyobj => {
                        panic!("Encountered InjectFrom(Pyobj) during vasm generation")
                    }
                    ex::Ty::Func => {
                        panic!("Encountered InjectFrom(Func) during vasm generation")
                    }
                }
            }
            flat::Stmt::Def(lhs, flat::Expr::Const(i)) => {
                self.mov(lhs, i);
            }
            flat::Stmt::Def(lhs, flat::Expr::LoadFunctionPointer(f)) => {
                self.mov_label(lhs, f);
            }
            flat::Stmt::Def(lhs, flat::Expr::Copy(var)) => {
                self.mov(lhs, var);
            }
            flat::Stmt::Discard(flat::Expr::CallFunc(f, args)) => {
                self.push_args_in_reverse(args);
                self.call_indirect(f);
            }
            flat::Stmt::Discard(flat::Expr::RuntimeFunc(name, args)) => {
                self.push_args_in_reverse(args);
                self.call_direct(name);
                // no return value handling
            }
            flat::Stmt::Discard(_expr) => {
                // skip over all the other kinds of exprs in a discard
            }
            flat::Stmt::Return(var) => {
                if let Some(var) = var {
                    self.mov(Reg::EAX, var);
                }
                self.mov(Reg::EBP, Reg::ESP);
                self.pop(Reg::EBP);
                self.ret();
            }
            flat::Stmt::If(cond, then, else_) => {
                let then = Block::from(then);
                let else_ = Block::from(else_);
                self.push_instr(Instr::If(cond.into(), then, else_));
            }
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
            Instr::Mov(lval, rval) => {
                write!(f, "mov {rval}, {lval}",
                       lval=lval,
                       rval=rval)?;
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

impl ::std::fmt::Display for Lval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Lval::Reg(reg) => {
                let reg = match reg {
                    Reg::EAX => "eax",
                    Reg::EBX => "ebx",
                    Reg::ECX => "ecx",
                    Reg::EDX => "edx",
                    Reg::ESI => "esi",
                    Reg::EDI => "edi",
                    Reg::ESP => "esp",
                    Reg::EBP => "ebp",
                };
                write!(f, "%{}", reg)
            }
            Lval::StackSlot(slot) => write!(f, "stack {}", slot.ebp_offset),
            Lval::Var(var) => write!(f, "{}", var),
        }
    }
}

impl ::std::fmt::Display for Rval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Rval::Lval(lval) => write!(f, "{}", lval),
            Rval::Imm(imm) => write!(f, "${}", imm),
        }
    }
}
