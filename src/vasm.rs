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
    Call(Rval),
    If(Rval, Block, Block),
}

#[derive(Debug, Clone, Hash)]
pub struct Block {
    instrs: Vec<Instr>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Rval {
    Lval(Lval),
    Const(i32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Lval {
    Reg(Reg),
    StackSlot(StackSlot),
    Var(Var),
}

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
            block: Block {
                instrs: vec![]
            }
        }
    }

    fn stmt(&mut self, stmt: flat::Stmt) {
        match stmt {
            flat::Stmt::Def(var, expr) => {
            }
            flat::Stmt::Discard(expr) => {
                self.expr(expr)
            }
            flat::Stmt::Return(var) => {

            }
            flat::Stmt::If(cond, then, else_) => {

            }
        }
    }

    fn expr(&mut self, expr: flat::Expr) {
        match expr {
            flat::Expr::UnaryOp(op, var) => {

            }
            flat::Expr::BinOp(op, left, right) => {
            }
            flat::Expr::CallFunc(f, args) => {

            }
            flat::Expr::RuntimeFunc(name, args) => {
            }
            flat::Expr::GetTag(var) => {

            }
            flat::Expr::ProjectTo(var, ty) => {

            }
            flat::Expr::InjectFrom(var, ty) => {

            }
            flat::Expr::Const(i) => {

            }
            flat::Expr::LoadFunctionPointer(func) => {

            }
            flat::Expr::Copy(var) => {

            }
        }
    }
}

impl fmt::Fmt for Module {
    fn fmt<W: ::std::io::Write>(&self, f: &mut fmt::Formatter<W>) -> ::std::io::Result<()> {
        use std::io::Write;

        writeln!(f, "vasm {{")?;
        writeln!(f, "}}")?;
        Ok(())
    }
}
