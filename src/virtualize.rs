use explicate as ex;
use flatten as flat;
use raise;

use explicate::Var;

use std::collections::HashMap;
use std::convert;

pub enum VirtInstr {
    MOV(Src, Dst),
    ADD(Src, Dst),
    PUSH(Src),
    POP(Dst),
    CALL(Src),
    VirtualIf(Src, Vec<VirtInstr>, Vec<VirtInstr>),
    CMP(Src, Src),
    JNZ(Src),
    JZ(Src),
    SETE(Dst),
    SETNE(Dst),
}

pub enum Src {
    Dst(Dst),
    Const(i32),
}

pub enum Dst {
    Reg(Register),
    Stack(i32),
    Tmp(Var),
}

impl convert::From<Dst> for Src {
    fn from(from: Dst) -> Src {
        Src::Dst(from)
    }
}

impl convert::From<Var> for Src {
    fn from(from: Var) -> Src {
        Src::Dst(from.into())
    }
}

impl convert::From<Var> for Dst {
    fn from(from: Var) -> Dst {
        Dst::Tmp(from)
    }
}

pub fn Param(index: i32) -> Src {
    Src::Dst(Dst::Stack(index))
}

pub enum Register {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
}

pub struct Virtualizer {
    pub var_data: ex::var::Slab<ex::var::Data>,
    pub units: HashMap<raise::Func, Vec<VirtInstr>>,
    contexts: Vec<Vec<VirtInstr>>,
}

impl Virtualizer {
    pub fn new(data: ex::var::Slab<ex::var::Data>) -> Virtualizer {
        Virtualizer {
            var_data: data,
            units: HashMap::new(),
            contexts: vec![],
        }
    }

    pub fn push(&mut self, instr: VirtInstr) {
        if let Some(top) = self.contexts.last_mut() {
            top.push(instr);
        } else {
            panic!("Tried to push with no context.");
        }
    }

    pub fn commit_fn(&mut self, func: raise::Func) {
        if let Some(top) = self.contexts.pop() {
            self.units.insert(func, top);
        } else {
            panic!("Tried to commit with no context.");
        }
    }

    pub fn scope(&mut self) {
        self.contexts.push(vec![]);
    }

    pub fn mk_tmp_var(&mut self) -> Var {
        self.var_data.insert(ex::var::Data::Temp)
    }
}

/*
impl Virtualize for () {
    type Output = ();
    pub fn virtualize(self, builder: &mut Virtualizer) -> () {
        ()
    }
}
*/

fn expr_as(e: flat::Expr, alias: Var) -> Vec<VirtInstr> {
    use self::flat::Expr;
    match e {
        Expr::Alias(v) => vec![VirtInstr::MOV(v.into(), alias.into())],
        Expr::BinOp(bop, v1, v2) => binop_as(bop, v1, v2, alias),
        Expr::CallFunc(f, args) => {
            let mut ac = args.clone();
            let mut instrs = vec![];
            for a in args {
                instrs.push(VirtInstr::PUSH(a.into()));

                //TODO
            }
            instrs
        }
        _ => unimplemented!(),
    }
}

fn binop_as(bop: flat::BinOp, v1: Var, v2: Var, alias: Var) -> Vec<VirtInstr> {
    use self::flat::BinOp;
    match bop {
        BinOp::ADD => {
            vec![
                VirtInstr::MOV(v2.into(), alias.into()),
                VirtInstr::ADD(v1.into(), alias.into()),
            ]
        }
        BinOp::EQ => {
            vec![
                VirtInstr::CMP(v1.into(), v2.into()),
                VirtInstr::SETE(alias.into()),
            ]
        }
        BinOp::NOTEQ => {
            vec![
                VirtInstr::CMP(v1.into(), v2.into()),
                VirtInstr::SETNE(alias.into()),
            ]
        }
    }
}

pub trait Virtualize {
    type Output;
    fn virtualize(self, builder: &mut Virtualizer) -> Self::Output;
}

impl Virtualize for HashMap<raise::Func, flat::Function> {
    type Output = ();
    fn virtualize(self, builder: &mut Virtualizer) -> () {
        for (fn_name, func) in self {
            builder.scope();
            func.body.virtualize(builder);
            builder.commit_fn(fn_name);
        }
    }
}

impl Virtualize for Vec<flat::Stmt> {
    type Output = ();
    fn virtualize(self, builder: &mut Virtualizer) -> () {
        for stmt in self {
            stmt.virtualize(builder);
        }
    }
}

impl Virtualize for flat::Stmt {
    type Output = ();
    fn virtualize(self, builder: &mut Virtualizer) -> () {
        use self::flat::Stmt;
        match self {
            Stmt::Def(v, expr) => {
                for instr in expr_as(expr, v) {
                    builder.push(instr);
                }
            }
            Stmt::Discard(expr) => {
                () // TODO
            }
        }
    }
}
