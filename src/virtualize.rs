use explicate as ex;
use flatten as flat;
use raise;

use explicate::Var;

use std::collections::HashMap;

#[derive(Debug, Clone, Hash)]
pub enum Instr {
    Mov(Lval, Rval),
    Add(Lval, Rval),
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

    fn function(&mut self, function: flat::Function) {
        for arg in function.args {
            self.function_arg(arg);
        }
        for stmt in function.body {
            self.stmt(stmt);
        }
    }

    fn function_arg(&mut self, var: Var) {
        // do nothing by default
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
            flat::Expr::Copy(Var) => {

            }
        }
    }
}


