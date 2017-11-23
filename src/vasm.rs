use explicate as ex;
use flatten as flat;
use raise;
use util::fmt;

use explicate::Var;

use std::collections::HashMap;
use std::collections::HashSet;

pub const WORD_SIZE: Imm = 4;

pub struct Module {
    pub main: raise::Func,
    pub funcs: HashMap<raise::Func, Function>,
    pub vars: ex::var::Slab<ex::var::Data>,
}

#[derive(Clone)]
pub struct Function {
    pub args: Vec<Var>,
    pub stack_slots: u32,
    pub block: Block,
}

pub struct FunctionBuilder {
    args: Vec<Var>,
    stack_slots: u32, }

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Block {
    pub insts: Vec<Inst>,
}

pub struct BlockBuilder<'a> {
    func: &'a mut FunctionBuilder,
    insts: Vec<Inst>,
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Inst {
    Mov(Lval, Rval),
    Add(Lval, Rval),
    Neg(Lval),
    Push(Rval),
    Pop(Lval),
    CallIndirect(Lval),
    Call(String),
    If(Lval, Block, Block),
    While(Lval, Block, Block),
    /// `Lval - Rval, sets EQ and NE (and other) flags`
    Cmp(Lval, Rval),
    /// `Lval = EQ ? 1 : 0;`
    Sete(Lval),
    /// `Lval = NE ? 1 : 0;`
    Setne(Lval),
    Or(Lval, Rval),
    And(Lval, Rval),
    /// I think `shr` requires arg to be
    /// in CL, which complicates instuction
    /// selection, so for now this only allows
    /// `Imm`'s, which is all we need
    Shr(Lval, Imm),
    /// See doc for `Shr` for why this only allows
    /// `Imm`
    Shl(Lval, Imm),
    /// `mov lval, $func`
    MovLabel(Lval, raise::Func),
    /// String s -> `jmp s`
    JmpLabel(String),
    /// If EFLAGS has eq-bit, jump to label
    JeqLabel(String),
    /// `sub l,r`
    Sub(Lval, Rval),
    /// Just `ret`, nothing else
    Ret,
    /// String s -> `s:`
    Label(String)
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
    Param(Param),
}

pub type Imm = i32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Param(u32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
pub struct StackSlot(u32);

impl Module {
    pub fn from(flattener: flat::Flattener) -> Module {
        let main = flattener.main;
        let vars = flattener.var_data;

        let mut funcs = HashMap::new();
        for (func, function) in flattener.units {
            let function = Function::from(function);
            funcs.insert(func, function);
        }

        Module {
            main,
            funcs,
            vars,
        }
    }
}

impl Function {
    pub fn from(function: flat::Function) -> Function {
        let mut func = FunctionBuilder {
            args: function.args,
            stack_slots: 0,
        };

        let block = {
            let mut block = func.block();
            block.stmts(function.body);
            block.complete()
        };

        let mut block = ReplaceParamVars::new(func.args.clone()).block(block);

        match block.insts.last() {
            Some(&Inst::Ret) => {}
            Some(_) | None => {
                block.insts.push(Inst::Ret.into());
            }
        }

        Function {
            args: func.args,
            stack_slots: func.stack_slots,
            block: block,
        }
    }
}

impl FunctionBuilder {
    pub fn block<'a>(&'a mut self) -> BlockBuilder<'a> {
        BlockBuilder {
            func: self,
            insts: vec![],
        }
    }
}

pub struct ReplaceParamVars {
    params: Vec<Var>,
}

impl ReplaceParamVars {
    pub fn new(params: Vec<Var>) -> ReplaceParamVars {
        ReplaceParamVars { params }
    }
}

impl TransformBlock for ReplaceParamVars {
    fn lval(&mut self, lval: Lval) -> Lval {
        match lval {
            Lval::Reg(reg) => Lval::Reg(reg),
            Lval::StackSlot(slot) => Lval::StackSlot(slot),
            Lval::Var(var) if self.params.contains(&var) => {
                let pos = self.params.iter().position(|&v| v == var).unwrap();
                Lval::Param(Param(pos as u32))
            }
            Lval::Var(var) => Lval::Var(var),
            Lval::Param(p) => Lval::Param(p),
        }
    }
}

impl<'a> BlockBuilder<'a> {
    fn nested<'b>(&'b mut self) -> BlockBuilder<'b> {
        BlockBuilder {
            func: self.func,
            insts: vec![],
        }
    }

    fn stmts(&mut self, stmts: Vec<flat::Stmt>) {
        for stmt in stmts {
            self.stmt(stmt);
        }
    }

    fn complete(self) -> Block {
        Block {
            insts: self.insts,
        }
    }

    fn push_inst(&mut self, inst: Inst) {
        self.insts.push(inst);
    }

    fn mov<L, R>(&mut self, lval: L, rval: R)
    where
        L: Into<Lval>,
        R: Into<Rval>,
    {
        let lval = lval.into();
        let rval = rval.into();
        self.push_inst(Inst::Mov(lval, rval));
    }

    fn neg<L>(&mut self, lval: L)
    where
        L: Into<Lval>,
    {
        let lval = lval.into();
        self.push_inst(Inst::Neg(lval));
    }

    fn push<R>(&mut self, rval: R)
    where
        R: Into<Rval>,
    {
        let rval = rval.into();
        self.push_inst(Inst::Push(rval));
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
        self.push_inst(Inst::Cmp(lval, 0.into()));
        self.push_inst(Inst::Sete(lval));
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
        self.push_inst(Inst::Add(lval, rval));
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
        self.push_inst(Inst::Cmp(lval, rval));
        self.push_inst(Inst::Sete(lval));
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
        self.push_inst(Inst::Cmp(lval, rval));
        self.push_inst(Inst::Setne(lval));
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
        self.push_inst(Inst::Cmp(lval, rval));
    }

    /// ```
    /// push arg[n]
    /// push arg[n - 1]
    /// ...
    /// push arg[1]
    /// push arg[0]
    /// ```
    fn push_args_in_reverse(&mut self, args: Vec<Var>) -> Imm {
        let alloca_len = args.len() as Imm * WORD_SIZE;
        for arg in args.into_iter().rev() {
            self.push(arg);
        }
        alloca_len
    }

    fn call_indirect<L>(&mut self, lval: L)
    where
        L: Into<Lval>,
    {
        self.push_inst(Inst::CallIndirect(lval.into()));
    }

    fn call_direct(&mut self, name: String) {
        self.push_inst(Inst::Call(name));
    }

    fn and<L, R>(&mut self, lval: L, rval: R)
    where
        L: Into<Lval>,
        R: Into<Rval>,
    {
        self.push_inst(Inst::And(lval.into(), rval.into()));
    }

    fn or<L, R>(&mut self, lval: L, rval: R)
    where
        L: Into<Lval>,
        R: Into<Rval>,
    {
        self.push_inst(Inst::Or(lval.into(), rval.into()));
    }

    fn shr<L>(&mut self, lval: L, imm: Imm)
    where
        L: Into<Lval>,
    {
        self.push_inst(Inst::Shr(lval.into(), imm));
    }

    fn shl<L>(&mut self, lval: L, imm: Imm)
    where
        L: Into<Lval>,
    {
        self.push_inst(Inst::Shl(lval.into(), imm));
    }

    fn mov_label<L>(&mut self, lval: L, func: raise::Func)
    where
        L: Into<Lval>,
    {
        self.push_inst(Inst::MovLabel(lval.into(), func));
    }

    fn pop<L>(&mut self, lval: L)
    where
        L: Into<Lval>,
    {
        self.push_inst(Inst::Pop(lval.into()));
    }

    fn ret(&mut self) {
        self.push_inst(Inst::Ret);
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
                let alloca_len = self.push_args_in_reverse(args);
                self.call_indirect(f);
                self.mov(lhs, Reg::EAX);
                self.add(Reg::ESP, alloca_len);
            }
            flat::Stmt::Def(lhs, flat::Expr::RuntimeFunc(name, args)) => {
                let alloca_len = self.push_args_in_reverse(args);
                self.call_direct(name);
                self.mov(lhs, Reg::EAX);
                self.add(Reg::ESP, alloca_len);
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
                let alloca_len = self.push_args_in_reverse(args);
                self.call_indirect(f);
                self.add(Reg::ESP, alloca_len);
            }
            flat::Stmt::Discard(flat::Expr::RuntimeFunc(name, args)) => {
                let alloca_len = self.push_args_in_reverse(args);
                self.call_direct(name);
                // no return value handling
                self.add(Reg::ESP, alloca_len);
            }
            flat::Stmt::Discard(_expr) => {
                // skip over all the other kinds of exprs in a discard
            }
            flat::Stmt::Return(var) => {
                if let Some(var) = var {
                    self.mov(Reg::EAX, var);
                }
                self.ret();
            }
            flat::Stmt::If(cond, then, else_) => {
                let then = {
                    let mut block = self.nested();
                    block.stmts(then);
                    block.complete()
                };
                let else_ = {
                    let mut block = self.nested();
                    block.stmts(else_);
                    block.complete()
                };
                self.push_inst(Inst::If(cond.into(), then, else_));
            }
            flat::Stmt::While(cond, comp, body) => {
                let compute_condition = {
                    let mut block = self.nested();
                    block.stmts(comp);
                    block.complete()
                };
                let i_body = {
                    let mut block = self.nested();
                    block.stmts(body);
                    block.complete()
                };
                self.push_inst(Inst::While(cond.into(), compute_condition, i_body))
            }
        }
    }
}

impl fmt::Fmt for Module {
    fn fmt<W: ::std::io::Write>(&self, out: &mut fmt::Formatter<W>) -> ::std::io::Result<()> {
        use std::io::Write;

        for (&f, func) in &self.funcs {
            if f == self.main {
                writeln!(out, ".globl main")?;
                writeln!(out, "main:")?;
            } else {
                writeln!(out, "{}:", f)?;
            }
            out.indent();
            out.fmt(func)?;
            out.dedent();
        }
        Ok(())
    }
}

impl fmt::Fmt for Function {
    fn fmt<W: ::std::io::Write>(&self, f: &mut fmt::Formatter<W>) -> ::std::io::Result<()> {
        use std::io::Write;
        f.fmt(&self.block)?;
        Ok(())
    }
}

impl fmt::Fmt for Block {
    fn fmt<W: ::std::io::Write>(&self, f: &mut fmt::Formatter<W>) -> ::std::io::Result<()> {
        use std::io::Write;

        for inst in &self.insts {
            f.fmt(inst)?;
        }
        Ok(())
    }
}

impl fmt::Fmt for Inst {
    fn fmt<W>(&self, f: &mut fmt::Formatter<W>) -> ::std::io::Result<()>
    where
        W: ::std::io::Write
    {
        use std::io::Write;
        match *self {
            Inst::Mov(lval, rval) => writeln!(f, "mov {}, {}", rval, lval),
            Inst::Add(lval, rval) => writeln!(f, "add {}, {}", rval, lval),
            Inst::Neg(lval) => writeln!(f, "neg {}", lval),
            Inst::Push(rval) => writeln!(f, "push {}", rval),
            Inst::Pop(lval) => writeln!(f, "pop {}", lval),
            Inst::CallIndirect(lval) => writeln!(f, "call *{}", lval),
            Inst::Call(ref name) => writeln!(f, "call {}", name),
            Inst::If(cond, ref then, ref else_) => {
                writeln!(f, "if {} {{", cond)?;
                f.indent();
                f.fmt(then)?;
                f.dedent();
                writeln!(f, "}} else {{")?;
                f.indent();
                f.fmt(else_)?;
                f.dedent();
                writeln!(f, "}}")?;
                Ok(())
            }
            Inst::While(c, ref cond, ref body) => {
                writeln!(f, "while {} via [", c)?;
                f.indent();
                f.fmt(cond)?;
                f.dedent();
                writeln!(f, "] {{")?;
                f.indent();
                f.fmt(body)?;
                f.dedent();
                writeln!(f, "}}")
            }
            Inst::Cmp(lval, rval) => writeln!(f, "cmp {}, {}", rval, lval),
            Inst::Sete(lval) => {
                let arg = match lval {
                    Lval::Reg(r) => match r {
                        Reg::EAX => "%al",
                        Reg::EBX => "%bl",
                        Reg::ECX => "%cl",
                        Reg::EDX => "%dl",
                        _ => panic!("registers edi and esi do not have 8 bit versions!"),
                    }.into(),
                    lval => format!("{}", lval),
                };
                writeln!(f, "sete {}", arg)
            }
            Inst::Setne(lval) => {
                let arg = match lval {
                    Lval::Reg(r) => match r {
                        Reg::EAX => "%al",
                        Reg::EBX => "%bl",
                        Reg::ECX => "%cl",
                        Reg::EDX => "%dl",
                        _ => panic!("registers edi and esi do not have 8 bit versions!"),
                    }.into(),
                    lval => format!("{}", lval),
                };
                writeln!(f, "setne {}", arg)
            }
            Inst::Or(lval, rval) => writeln!(f, "or {}, {}", rval, lval),
            Inst::And(lval, rval) => writeln!(f, "and {}, {}", rval, lval),
            Inst::Shr(lval, imm) => writeln!(f, "shr ${}, {}", imm, lval),
            Inst::Shl(lval, imm) => writeln!(f, "shl ${}, {}", imm, lval),
            Inst::MovLabel(lval, func) => writeln!(f, "mov ${}, {}", func, lval),
            Inst::JmpLabel(ref label) => writeln!(f, "jmp {}", label),
            Inst::JeqLabel(ref label) => writeln!(f, "je {}", label),
            Inst::Label(ref label) => {
                f.dedent();
                writeln!(f, "{}:", label)?;
                f.indent();
                Ok(())
            },
            Inst::Sub(lval, rval) => writeln!(f, "sub {}, {}", rval, lval),
            Inst::Ret => writeln!(f, "ret"),
        }
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

impl From<Param> for Lval {
    fn from(p: Param) -> Self {
        Lval::Param(p)
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
            Lval::StackSlot(slot) => write!(f, "stack {}", slot.0),
            Lval::Var(var) => write!(f, "{}", var),
            Lval::Param(p) => write!(f, "param {}", p.0),
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

pub trait TransformBlock {
    fn block(&mut self, block: Block) -> Block {
        Block {
            insts: block.insts.into_iter().flat_map(|inst| {
                self.inst(inst)
            }).collect()
        }
    }

    fn inst(&mut self, inst: Inst) -> Vec<Inst> {
        use self::Inst::*;
        let inst = match inst {
            Mov(l, r) => Mov(self.lval(l), self.rval(r)),
            Add(l, r) => Add(self.lval(l), self.rval(r)),
            Neg(l) => Neg(self.lval(l)),
            Push(r) => Push(self.rval(r)),
            Pop(l) => Pop(self.lval(l)),
            CallIndirect(l) => CallIndirect(self.lval(l)),
            Call(name) => Call(name),
            If(c, then, else_) => If(self.lval(c), self.block(then), self.block(else_)),
            Cmp(l, r) => Cmp(self.lval(l), self.rval(r)),
            Sete(l) => Sete(self.lval(l)),
            Setne(l) => Setne(self.lval(l)),
            Or(l, r) => Or(self.lval(l), self.rval(r)),
            And(l, r) => And(self.lval(l), self.rval(r)),
            Shr(l, imm) => Shr(self.lval(l), imm),
            Shl(l, imm) => Shl(self.lval(l), imm),
            MovLabel(l, func) => MovLabel(self.lval(l), func),
            While(c, cond, body) => While(self.lval(c), self.block(cond), self.block(body)),
                        Ret => Ret,
            Label(_) => panic!("Encountered label in TransformBlock"),
            JmpLabel(_) => panic!("Encountered `jmp label` in TransformBlock"),
            JeqLabel(_) => panic!("Encountered `jeq label` in TransformBlock"),
            Sub(_, _) => panic!("Encountered `sub` in TransformBlock"),
        };
        vec![inst]
    }

    fn lval(&mut self, lval: Lval) -> Lval {
        lval
    }

    fn rval(&mut self, rval: Rval) -> Rval {
        match rval {
            Rval::Imm(imm) => Rval::Imm(imm),
            Rval::Lval(lval) => Rval::Lval(self.lval(lval))
        }
    }
}

pub fn render_func(label: raise::Func, f : Function, is_main: bool) -> Vec<Inst> {
    let label: String = if is_main { "main".into() } else { format!("{}", label) };
    let mut result = vec![
        //Inst::Label(format!("{}", label)),
        Inst::Push(Rval::Lval(Lval::Reg(Reg::EBX))),
        Inst::Push(Rval::Lval(Lval::Reg(Reg::EDI))),
        Inst::Push(Rval::Lval(Lval::Reg(Reg::ESI))),
    ];

    // write shim first
    result.push(Inst::Push(Reg::EBP.into()));
    result.push(Inst::Mov(Reg::EBP.into(), Reg::ESP.into()));
    result.push(Inst::Sub(Reg::ESP.into(), Rval::Imm(f.stack_slots as Imm * WORD_SIZE)));

    result.extend(linearize(label.clone(), f.block.insts));

    result.push(Inst::Label(format!("{}.out", label)));

    result.push(Inst::Mov(Reg::ESP.into(), Reg::EBP.into()));
    result.push(Inst::Pop(Reg::EBP.into()));

    result.extend(vec![
        Inst::Pop(Lval::Reg(Reg::ESI)),
        Inst::Pop(Lval::Reg(Reg::EDI)),
        Inst::Pop(Lval::Reg(Reg::EBX)),
    ]);

    result.push(Inst::Ret);

    result
}

fn linearize(label_prefix: String, instrs: Vec<Inst>) -> Vec<Inst> {
    let mut label_index : usize = 0;

    let mut next_label = | | {
        let r = label_index;
        label_index += 1;
        format!("{}.{}", label_prefix, r)
    };

    let mut v = vec![];
    for instr in instrs {
        match instr {
            Inst::If(c, then, else_) => {
                let l_else = next_label();
                let l_end = next_label();

                //   CMP c,c
                //   JZ _else
                v.push(Inst::Cmp(c,Rval::Imm(0)));
                v.push(Inst::JeqLabel(l_else.clone()));

                v.extend(then.insts);

                //   JMP _end
                // _else:
                v.push(Inst::JmpLabel(l_end.clone()));
                v.push(Inst::Label(l_else));

                v.extend(else_.insts);

                // _end:
                v.push(Inst::Label(l_end));
            },
            Inst::While(c, comp, body) => {
                let l_top = next_label();
                let l_bot = next_label();

                // _top:
                v.push(Inst::Label(l_top.clone()));

                v.extend(comp.insts);

                //   CMP c,c
                //   JZ _bot
                v.push(Inst::Cmp(c, Rval::Imm(0)));
                v.push(Inst::JeqLabel(l_bot.clone()));

                v.extend(body.insts);

                //   JMP _top
                v.push(Inst::JmpLabel(l_top));

                // _bot:
                v.push(Inst::Label(l_bot));
            },
            Inst::Ret => v.push(Inst::JmpLabel(format!("{}.out", label_prefix))),
            // Default case, don't modify the instruction
            i => v.push(i),
        };
    }
    v
}

pub trait VisitBlock {
    fn block(&mut self, block: &Block) {
        for inst in &block.insts {
            self.inst(inst);
        }
    }

    fn inst(&mut self, inst: &Inst) {
        use self::Inst::*;
        match *inst {
            Mov(l, r) => self.mov(l, r),
            Add(l, r) => self.add(l, r),
            Neg(l) => self.neg(l),
            Push(r) => self.push(r),
            Pop(l) => self.pop(l),
            CallIndirect(l) => self.call_indirect(l),
            Call(ref name) => self.call(name),
            If(lval, ref then, ref else_) => self.if_(lval, then, else_),
            Cmp(l, r) => self.cmp(l, r),
            Sete(l) => self.sete(l),
            Setne(l) => self.setne(l),
            Or(l, r) => self.or(l, r),
            And(l, r) => self.and(l, r),
            Shr(l, imm) => self.shr(l, imm),
            Shl(l, imm) => self.shl(l, imm),
            MovLabel(l, func) => self.mov_label(l, func),
            While(_, _, _) => unimplemented!(),
            JmpLabel(_) => unimplemented!(),
            JeqLabel(_) => unimplemented!(),
            Sub(_, _) => unimplemented!(),
            Label(_) => unimplemented!(),
            Ret => self.ret(),
        }
    }

    fn mov(&mut self, l: Lval, r: Rval) {
        self.lval(l);
        self.rval(r);
    }

    fn add(&mut self, l: Lval, r: Rval) {
        self.lval(l);
        self.rval(r);
    }

    fn neg(&mut self, l: Lval) {
        self.lval(l);
    }

    fn push(&mut self, r: Rval) {
        self.rval(r);
    }

    fn pop(&mut self, l: Lval) {
        self.lval(l);
    }

    fn call_indirect(&mut self, l: Lval) {
        self.lval(l);
    }

    fn call(&mut self, name: &str) {}

    fn if_(&mut self, cond: Lval, then: &Block, else_: &Block) {
        self.lval(cond);
        self.block(then);
        self.block(else_);
    }

    fn cmp(&mut self, l: Lval, r: Rval) {
        self.lval(l);
        self.rval(r);
    }

    fn sete(&mut self, l: Lval) {
        self.lval(l);
    }

    fn setne(&mut self, l: Lval) {
        self.lval(l);
    }

    fn or(&mut self, l: Lval, r: Rval) {
        self.lval(l);
        self.rval(r);
    }

    fn and(&mut self, l: Lval, r: Rval) {
        self.lval(l);
        self.rval(r);
    }

    fn shr(&mut self, l: Lval, i: Imm) {
        self.lval(l);
    }

    fn shl(&mut self, l: Lval, i: Imm) {
        self.lval(l);
    }

    fn mov_label(&mut self, l: Lval, label: raise::Func) {
        self.lval(l);
    }

    fn ret(&mut self) {}

    fn lval(&mut self, lval: Lval) {}

    fn rval(&mut self, rval: Rval) {
        match rval {
            Rval::Imm(imm) => {}
            Rval::Lval(lval) => self.lval(lval),
        }
    }
}

impl Inst {

    /// Gives the write set for the instruction.
    /// NOTE: if the instruction is an If, it will
    /// panic! Ifs should be handled manually
    pub fn write_set(&self) -> HashSet<Lval> {
        use self::Inst::*;
        match *self {
            Mov(lval, _) | Add(lval, _) |
            Neg(lval) | Pop(lval) |
            Sete(lval) | Setne(lval) |
            Or(lval, _) | And(lval, _) |
            Shr(lval, _) | Shl(lval, _) |
            MovLabel(lval, _) => hash_set!(lval),

            Push(_) | Cmp(_, _) | Ret => hash_set!(),

            Call(_) | CallIndirect(_)
                => hash_set!(Reg::EAX, Reg::ECX, Reg::EDX).into_iter().map(|reg| reg.into()).collect(),

            If(_, _, _) => panic!("write_set called on Inst::If"),
            While(_, _, _) => unimplemented!(),
            JmpLabel(_) => unimplemented!(),
            JeqLabel(_) => unimplemented!(),
            Sub(_, _) => unimplemented!(),
            Label(_) => unimplemented!(),
        }
    }

    /// Gives the read set for the instruction.
    /// NOTE: if the instruction is an If, it will
    /// panic! Ifs should be handled manually
    pub fn read_set(&self) -> HashSet<Lval> {
        use self::Inst::*;
        match *self {
            Add(l, Rval::Lval(r))
                | Or(l, Rval::Lval(r))
                | And(l, Rval::Lval(r))
                | Cmp(l, Rval::Lval(r)) => hash_set!(l, r),

            Add(v, Rval::Imm(_))
                | Or(v, Rval::Imm(_))
                | And(v, Rval::Imm(_))
                | Cmp(v, Rval::Imm(_)) => hash_set!(v),

            Mov(_, Rval::Lval(v))
                | Neg(v)
                | Push(Rval::Lval(v))
                | CallIndirect(v)
                | Shr(v, _)
                | Shl(v, _) => hash_set!(v),

            Call(_)
                | Sete(_)
                | Setne(_)
                | MovLabel(_, _)
                | Ret
                | Pop(_)
                | Mov(_, Rval::Imm(_))
                | Push(Rval::Imm(_))
                => hash_set!(),
            If(_, _, _) => panic!("read_set called on Inst::If"),
            While(_, _, _) => unimplemented!(),
            JmpLabel(_) => unimplemented!(),
            JeqLabel(_) => unimplemented!(),
            Sub(_, _) => unimplemented!(),
            Label(_) => unimplemented!(),
        }
    }
}

pub struct ReplaceStackOps<'vars> {
    vars: &'vars mut ::explicate::var::Slab<::explicate::var::Data>,
}

impl<'vars> TransformBlock for ReplaceStackOps<'vars> {
    fn inst(&mut self, inst: Inst) -> Vec<Inst> {
        use self::Inst::*;
        use self::Lval::*;
        use self::Rval::*;
        let inst = match inst {
            Mov(StackSlot(dst), Lval(StackSlot(src))) => {
                let var = self.vars.insert(::explicate::var::Data::Temp);
                return vec![
                    Mov(var.into(), StackSlot(src).into()),
                    Mov(StackSlot(dst).into(), var.into()),
                ]
            }
            Add(StackSlot(dst), Lval(StackSlot(src))) => {
                let var = self.vars.insert(::explicate::var::Data::Temp);
                return vec![
                    Mov(var.into(), StackSlot(dst).into()),
                    Add(var.into(), StackSlot(src).into()),
                    Mov(StackSlot(dst).into(), var.into()),
                ]
            }
            Or(StackSlot(dst), Lval(StackSlot(src))) => {
                let var = self.vars.insert(::explicate::var::Data::Temp);
                return vec![
                    Mov(var.into(), StackSlot(dst).into()),
                    Or(var.into(), StackSlot(src).into()),
                    Mov(StackSlot(dst).into(), var.into()),
                ]
            }
            And(StackSlot(dst), Lval(StackSlot(src))) => {
                let var = self.vars.insert(::explicate::var::Data::Temp);
                return vec![
                    Mov(var.into(), StackSlot(dst).into()),
                    And(var.into(), StackSlot(src).into()),
                    Mov(StackSlot(dst).into(), var.into()),
                ]
            }

            Mov(l, r) => Mov(self.lval(l), self.rval(r)),
            Add(l, r) => Add(self.lval(l), self.rval(r)),
            Neg(l) => Neg(self.lval(l)),
            Push(r) => Push(self.rval(r)),
            Pop(l) => Pop(self.lval(l)),
            CallIndirect(l) => CallIndirect(self.lval(l)),
            Call(name) => Call(name),
            If(lval, then, else_) => If(self.lval(lval), self.block(then), self.block(else_)),
            Cmp(l, r) => Cmp(self.lval(l), self.rval(r)),
            Sete(l) => Sete(self.lval(l)),
            Setne(l) => Setne(self.lval(l)),
            Or(l, r) => Or(self.lval(l), self.rval(r)),
            And(l, r) => And(self.lval(l), self.rval(r)),
            Shr(l, imm) => Shr(self.lval(l), imm),
            Shl(l, imm) => Shl(self.lval(l), imm),
            MovLabel(l, func) => MovLabel(self.lval(l), func),
            While(l, cond, body) => While(self.lval(l), self.block(cond), self.block(body)),
            JmpLabel(_) => unimplemented!(),
            JeqLabel(_) => unimplemented!(),
            Sub(_, _) => unimplemented!(),
            Label(_) => unimplemented!(),
            Ret => Ret,
        };
        vec![inst]
    }
}

impl Function {
    pub fn replace_stack_to_stack_ops(self, vars: &mut ex::var::Slab<ex::var::Data>) -> Self {
        let mut replace = ReplaceStackOps {
            vars: vars,
        };
        Function {
            args: self.args,
            stack_slots: self.stack_slots,
            block: replace.block(self.block),
        }
    }

    pub fn spill(&mut self, var: Var) {
        let mut spill = Spill {
            var: var,
            stack_slot: StackSlot(self.stack_slots),
        };
        self.block = spill.block(self.block.clone());
        self.stack_slots += 1;
    }
}

struct Spill {
    var: Var,
    stack_slot: StackSlot,
}

impl TransformBlock for Spill {
    fn lval(&mut self, lval: Lval) -> Lval {
        if let Lval::Var(var) = lval {
            if var == self.var {
                return self.stack_slot.into()
            }
        }
        lval
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use explicate::var;
    use raise::func;

    #[test]
    fn write_set() {
        let mut funcs: func::Slab<()> = func::Slab::new();
        let mut vars: var::Slab<()> = var::Slab::new();
        let x: Lval = vars.insert(()).into();
        let z: Rval = vars.insert(()).into();
        let f = funcs.insert(());
        use self::Inst::*;

        assert_eq!(Mov(x, z).write_set(), hash_set!(x));
        assert_eq!(Add(x, z).write_set(), hash_set!(x));
        assert_eq!(Neg(x).write_set(), hash_set!(x));
        assert_eq!(Push(z).write_set(), hash_set!());
        assert_eq!(Pop(x).write_set(), hash_set!(x));
        assert_eq!(CallIndirect(x).write_set(), hash_set!(Reg::EAX, Reg::ECX, Reg::EDX).into_iter()
                   .map(|reg| reg.into()).collect());
        assert_eq!(Call("hello".into()).write_set(), hash_set!(Reg::EAX, Reg::ECX, Reg::EDX).into_iter()
                   .map(|reg| reg.into()).collect());
        assert_eq!(Cmp(x, z).write_set(), hash_set!());
        assert_eq!(Sete(x).write_set(), hash_set!(x));
        assert_eq!(Setne(x).write_set(), hash_set!(x));
        assert_eq!(Or(x, z).write_set(), hash_set!(x));
        assert_eq!(And(x, z).write_set(), hash_set!(x));
        assert_eq!(Shr(x, 2).write_set(), hash_set!(x));
        assert_eq!(Shl(x, 2).write_set(), hash_set!(x));
        assert_eq!(MovLabel(x, f).write_set(), hash_set!(x));
        assert_eq!(Ret.write_set(), hash_set!());
    }

    #[test]
    fn read_set() {
        let mut funcs: func::Slab<()> = func::Slab::new();
        let mut vars: var::Slab<()> = var::Slab::new();
        let x: Lval = vars.insert(()).into();
        let z: Lval = vars.insert(()).into();
        let zr: Rval = z.into();
        let i: Rval = 3.into();
        let f = funcs.insert(());
        use self::Inst::*;

        assert_eq!(Mov(x, zr).read_set(), hash_set!(z));
        assert_eq!(Mov(x, i).read_set(), hash_set!());

        assert_eq!(Add(x, zr).read_set(), hash_set!(x, z));
        assert_eq!(Add(x, i).read_set(), hash_set!(x));

        assert_eq!(Neg(x).read_set(), hash_set!(x));

        assert_eq!(Push(zr).read_set(), hash_set!(z));
        assert_eq!(Push(i).read_set(), hash_set!());

        assert_eq!(Pop(x).read_set(), hash_set!());

        assert_eq!(CallIndirect(x).read_set(), hash_set!(x));

        assert_eq!(Call("hello".into()).read_set(), hash_set!());

        assert_eq!(Cmp(x, zr).read_set(), hash_set!(x, z));
        assert_eq!(Cmp(x, i).read_set(), hash_set!(x));

        assert_eq!(Sete(x).read_set(), hash_set!());
        assert_eq!(Setne(x).read_set(), hash_set!());

        assert_eq!(Or(x, zr).read_set(), hash_set!(x, z));
        assert_eq!(Or(x, i).read_set(), hash_set!(x));

        assert_eq!(And(x, zr).read_set(), hash_set!(x, z));
        assert_eq!(And(x, i).read_set(), hash_set!(x));

        assert_eq!(Shr(x, 2).read_set(), hash_set!(x));

        assert_eq!(Shl(x, 2).read_set(), hash_set!(x));

        assert_eq!(MovLabel(x, f).read_set(), hash_set!());

        assert_eq!(Ret.read_set(), hash_set!());
    }
}
