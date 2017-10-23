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
    // If not zero, then, else.
    If {
        cond: Cond,
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
    And,
    Or,
    Sub,
    Shr,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cond {
    Eq(Val, Val),
    Zero(Val),
    Nonzero(Val),
    IsTrue(Val),
}
pub use self::Cond::*;

macro_rules! call {
    (
        $func_name:ident($($arg:expr),*)
    ) => ({
        let func_name = stringify!($func_name).into();
        let args = vec![$($arg),*];
        Call(Var(func_name), args)
    })
}

pub struct TmpAllocator {
    next: usize,
}

impl TmpAllocator {
    pub fn new(next: usize) -> TmpAllocator {
        TmpAllocator { next }
    }

    pub fn next(&mut self) -> usize {
        let next = self.next;
        self.next += 1;
        next
    }
}

pub struct BlockBuilder<'tmp> {
    tmp_allocator: &'tmp mut TmpAllocator,
    block: Vec<Stmt>,
}

impl<'tmp> BlockBuilder<'tmp> {
    fn new(tmp_allocator: &'tmp mut TmpAllocator) -> BlockBuilder<'tmp> {
        BlockBuilder {
            tmp_allocator,
            block: Vec::new(),
        }
    }

    fn complete(&mut self) -> Block {
        Block {
            stmts: ::std::mem::replace(&mut self.block, Vec::new()),
        }
    }

    fn nested_builder(&mut self) -> BlockBuilder {
        BlockBuilder {
            tmp_allocator: self.tmp_allocator,
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
                self.if_expr(cond, then, els)
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
            Dict(pairs) => {
                let big = self.tmp(call!(create_dict()));
                let dict = self.tmp(call!(inject_big(big)));
                for (k, v) in pairs {
                    let k = self.expression(k);
                    let v = self.expression(v);
                    let _ = self.tmp(call!(set_subscript(dict.clone(), k, v)));
                }
                dict
            }
            Add(box l, box r) => {
                let l = self.expression(l);
                let r = self.expression(r);
                let l_tag = self.tmp(call!(tag(l.clone())));
                let big_tag = self.tmp(Int(BIG_TAG));
                let if_big = Eq(l_tag, big_tag);

                let (small_res, small) = {
                    let mut b = self.nested_builder();
                    let shift = b.tmp(Int(2));
                    let l = b.tmp(Binop(Shr, shift.clone(), l.clone()));
                    let r = b.tmp(Binop(Shr, shift.clone(), r.clone()));
                    let tmp = b.tmp(Binop(Binop::Add, l, r));
                    let res = b.tmp(call!(inject_int(tmp)));
                    let block = b.complete();
                    (res, block)
                };

                let (big_res, big) = {
                    let mut b = self.nested_builder();
                    let l = b.tmp(call!(project_big(l)));
                    let r = b.tmp(call!(project_big(r)));
                    let big = b.tmp(call!(add(l, r)));
                    let res = b.tmp(call!(inject_big(big)));
                    let block = b.complete();
                    (res, block)
                };

                self.if_stmt(if_big, big, small);
                self.tmp(Phi(small_res, big_res))
            }
            LogicalOr(box l, box r) => {
                let l = self.expression(l);
                let l_is_true = self.tmp(call!(is_true(l)));
                let then = {
                };
                self.tmp(Phi(l, r))
            }
            _ => unimplemented!()
        }
    }

    fn block_expr(&mut self, e: Expression) -> (Val, Block) {
        let mut nested = self.nested_builder();
        let val = nested.expression(e);
        let block = nested.complete();
        (val, block)
    }

    fn if_stmt(&mut self, cond: Cond, then: Block, els: Block) {
        self.push_stmt(If {
            cond,
            then,
            els,
        });
    }

    fn if_expr(&mut self, cond: Expression, then: Expression, els: Expression) -> Val {
        let cond = self.expression(cond);
        let (then_val, then) = self.block_expr(then);
        let (els_val, els) = self.block_expr(els);
        self.push_stmt(If {
            cond: IsTrue(cond),
            then,
            els,
        });
        self.tmp(Phi(then_val, els_val))
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
        Tmp(self.tmp_allocator.next())
    }

    fn push_stmt(&mut self, st: Stmt) {
        self.block.push(st);
    }
}
