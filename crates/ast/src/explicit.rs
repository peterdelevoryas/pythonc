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
    Name(Name),

    /// For now, all functions are primitive, so
    /// we can statically assert only intrinsics
    /// are being used!
    Call(Box<Expr>, Vec<Expr>),

    /// PRIMITIVE ADD!!!! NOT POLYMORPHIC
    Add(Box<Expr>, Box<Expr>),

    /// PRIMITIVE BITWISE CMP!!! NOT POLYMORPHIC
    Is(Box<Expr>, Box<Expr>),

    /// PRIMITIVE NEGATE!!! Does not project arg
    Neg(Box<Expr>),
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
            name_map,
            block: Block {
                stmts: vec![]
            },
        }
    }

    pub fn explicate<N>(&mut self, node: N)
    where
        N: Explicate
    {
        node.explicate(self);
    }

    pub fn name_map(&mut self) -> &mut NameMap {
        self.name_map.borrow_mut()
    }
}

pub trait Explicate {
    fn explicate<M>(&self, b: &mut Builder<M>)
    where
        M: BorrowMut<NameMap>;
}

impl Explicate for Expression {
    fn explicate<M>(&self, b: &mut Builder<M>)
    where
        M: BorrowMut<NameMap>
    {
        use self::Statement::*;
        use self::Expression::*;
        use self::Target::*;
        match *self {
            Target(Name(ref name)) => {
                let name = b.name_map().insert_name(name.clone());

            }
            Target(Tmp(_)) => panic!("this should get removed"),
            _ => unimplemented!()
        }
    }
}
