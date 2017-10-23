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
    Phi(Val, Val),
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
        $func_name:ident($($arg:expr),*)
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

    fn complete(&mut self) -> Block {
        Block {
            stmts: ::std::mem::replace(&mut self.block, Vec::new()),
        }
    }

    fn nested_builder<'b>(&'b mut self) -> BlockBuilder<impl 'b + Iterator<Item=Val>> {
        BlockBuilder {
            tmp_generator: &mut self.tmp_generator,
            block: Vec::new(),
        }
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
                let (then, then_block) = {
                    let mut nested = self.nested_builder();
                    let val = nested.expression(then);
                    let block = nested.complete();
                    (val, block)
                };
                let (els, els_block) = {
                    let mut nested = self.nested_builder();
                    let val = nested.expression(els);
                    let block = nested.complete();
                    (val, block)
                };
                self.push_stmt(Stmt::If {
                    cond,
                    then: then_block,
                    els: els_block,
                });
                self.tmp(Phi(then, els))
            }
            List(elems) => {
                let int = self.tmp(Int(elems.len() as i32));
                let len = self.tmp(call!(inject_int(int)));
                let big = self.tmp(call!(create_list(len)));
                let list = self.tmp(call!(inject_big(big)));
                for (i, elem) in elems.into_iter().enumerate() {
                    let int = self.tmp(Int(i as i32));
                    let index = self.tmp(call!(inject_int(int)));
                    let tmp = self.expression(elem);
                    let _ = self.tmp(call!(set_subscript(list.clone(), index, tmp)));
                }
                list
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
