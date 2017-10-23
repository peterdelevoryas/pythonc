// cond ? then : else =>
// 
// switch cond [bb0, bb1]
// bb0:
//     tmp = then
// bb1:
//     tmp = else
// ret tmp
//

use Statement;
use Expression;
use Target;
use Module;
use Program;
use std::ops::RangeFrom;

const INT_TAG: i32 = 0;
const BOOL_TAG: i32 = 1;
const FLOAT_TAG: i32 = 2;
const BIG_TAG: i32 = 3;
const MASK: i32 = 3;
const SHIFT: i32 = 2;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    If {
        cond: Val,
        then: Block,
        els: Block,
    },
    Def(Val, Expr),
}
pub use self::Stmt::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Call(Val, Vec<Val>),
    Binop(Binop, Val, Val),
    Unop(Unop, Val),
    Int(i32),
    Bool(bool),
}
pub use self::Expr::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Binop {
    Add,
    // Bitwise! not logical
    Or,
    Cmp,
}
pub use self::Binop::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Unop {
    Not,
    Neg,
    Copy,
}
pub use self::Unop::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Val {
    Var(String),
    Tmp(usize),
}
pub use self::Val::*;

macro_rules! call {
    (
        $func_name:ident($($arg:ident),*)
    ) => ({
        let func_name = stringify!($func_name).into();
        let args = vec![$($arg),*];
        Call(Var(func_name), args)
    })
}

pub struct BlockBuilder<G>
where
    G: Iterator<Item=Val>,
{
    tmp_generator: G,
    block: Vec<Stmt>,
}

impl<G> BlockBuilder<G>
where
    G: Iterator<Item=Val>,
{
    fn new() -> BlockBuilder<impl Iterator<Item=Val>> {
        BlockBuilder {
            tmp_generator: (0..).map(|i| Val::Tmp(i)),
            block: Vec::new(),
        }
    }

    fn block(&mut self, statements: Vec<Statement>) -> Block {
        for st in statements {
            self.statement(st);
        }

        Block {
            stmts: ::std::mem::replace(&mut self.block, Vec::new()),
        }
    }

    fn nested_block(&mut self, statements: Vec<Statement>) -> Block {
        let mut nested_builder = BlockBuilder {
            tmp_generator: &mut self.tmp_generator,
            block: Vec::new(),
        };
        nested_builder.block(statements)
    }

    fn statement(&mut self, st: Statement) {
        use self::Statement::*;
        use self::Target::*;
        match st {
            Print(e) => {
                let tmp = self.expression(e);
                let _ = self.tmp(call!(print_any(tmp)));
            }
            Assign(Name(name), e) => {
                let tmp = self.expression(e);
                self.def(Var(name), Unop(Copy, tmp));
            }
            Assign(Subscript(box base, box elem), e) => {
                let base = self.expression(base);
                let elem = self.expression(elem);
                let tmp = self.expression(e);
                let _ = self.tmp(call!(set_subscript(base, elem, tmp)));
            }
            Expression(e) => {
                let _ = self.expression(e);
            }
            Newline => {}
        }
    }

    fn expression(&mut self, e: Expression) -> Val {
        use self::Expression::*;
        use self::Target::*;
        match e {
            Target(Name(name)) => Var(name),
            Target(Subscript(box base, box elem)) => {
                let base = self.expression(base);
                let elem = self.expression(elem);
                self.tmp(call!(get_subscript(base, elem)))
            }
            DecimalI32(i) => {
                let i = self.tmp(Int(i));
                self.tmp(call!(inject_int(i)))
            }
            Boolean(b) => {
                let b = self.tmp(Bool(b));
                self.tmp(call!(inject_bool(b)))
            }
            Input => self.tmp(call!(input_int())),
            UnaryNeg(box e) => {
                let tmp = self.expression(e);
                self.tmp(Unop(Neg, tmp))
            }
            If(box cond, box then, box els) => {
                let cond = self.expression(cond);
                //let then = self.nested_block
                unimplemented!()
            }
            _ => unimplemented!()
        }
    }

    fn tmp(&mut self, e: Expr) -> Val {
        let tmp = self.new_tmp_val();
        // clone here isn't a big deal,
        // because we know it's just a Tmp
        self.def(tmp.clone(), e);
        tmp
    }

    fn def(&mut self, val: Val, e: Expr) {
        self.push_stmt(Def(val, e));
    }

    fn new_tmp_val(&mut self) -> Val {
        self.tmp_generator.next().unwrap()
    }

    fn push_stmt(&mut self, st: Stmt) {
        self.block.push(st);
    }
}
