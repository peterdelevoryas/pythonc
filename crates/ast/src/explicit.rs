use name::Name;
use name::Map as NameMap;
use ::Expression;
use ::Statement;
use ::Target;

pub struct Block {
    pub stmts: Vec<Stmt>,
}

pub enum Stmt {
    Def(Name, Expr),

    // could be removed if allow Unit-type Lval's
    Expr(Expr),

    If {
        cond: Expr,
        then: Block,
        els: Block,
    }
}

pub enum Expr {
    Const(Const),
    Copy(Name),

    /// For now, all functions are primitive, so
    /// we can statically assert only intrinsics
    /// are being used!
    Call(Name, Vec<Name>),

    /// PRIMITIVE ADD!!!! NOT POLYMORPHIC
    Add(Name, Name),

    /// PRIMITIVE BITWISE CMP!!! NOT POLYMORPHIC
    Is(Name, Name),

    /// PRIMITIVE NEGATE!!! Does not project arg
    Neg(Name),
}

pub enum Const {
    Int(i32),
    Bool(bool),
}

use std::borrow::BorrowMut;

pub struct Builder<M>
where
    M: BorrowMut<NameMap>,
{
    name_map: M,
    block: Block,
}

impl<M> Builder<M>
where
    M: BorrowMut<NameMap>,
{
    pub fn new(name_map: M) -> Self {
        Self {
            name_map: name_map,
            block: Block {
                stmts: vec![]
            },
        }
    }

    pub fn name_map(&mut self) -> &mut NameMap {
        self.name_map.borrow_mut()
    }

    pub fn expr(&mut self, e: Expression) -> Name {
        use self::Statement::*;
        use self::Expression::*;
        use self::Target::*;
        match e {
            Target(Name(ref name)) => {
                self.name_map().insert_name(name)
            }

            Target(Tmp(_)) => panic!("dead code"),

            // basically just assume that base and elem are pyobj's
            Target(Subscript(box base, box elem)) => {
                let base = self.expr(base);
                let elem = self.expr(elem);
                unimplemented!()
            }

            _ => unimplemented!()
        }
    }

    pub fn call(&mut self, func: Name, args: Vec<Name>) -> Name {
        self.tmp(Expr::Call(func, args))
    }

    pub fn tmp(&mut self, e: Expr) -> Name {
        let tmp = self.name_map().create_tmp();
        self.def(tmp, e);
        tmp
    }

    pub fn def(&mut self, name: Name, e: Expr) {
        self.push(Stmt::Def(name, e));
    }

    pub fn push(&mut self, s: Stmt) {
        self.block.stmts.push(s);
    }
}
