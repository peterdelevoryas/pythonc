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
        // The condition is just going to be tested for zero
        // or nonzero, all is_true calls should be created
        // in explicate builder.
        cond: Name,
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
                self.get_subscript(base, elem)
            }

            DecimalI32(int) => {
                self.int(int)
            }

            Boolean(b) => {
                self.boolean(b)
            }

            Input => {
                self.input()
            }

            UnaryNeg(box e) => {
                let e = self.expr(e);
                unimplemented!()
            }

            LogicalOr(box first, box second) => {
                self.or(first, second)
            }

            _ => unimplemented!()
        }
    }

    // Takes current stmts and returns them.
    // Sets current stmts to empty.
    // Leaves NameMap untouched.
    pub fn finish_block(&mut self) -> Block {
        let empty = Block {
            stmts: vec![]
        };
        ::std::mem::replace(&mut self.block, empty)
    }

    // Creates a new builder for use in creating a new block
    pub fn block(&mut self) -> Builder<&mut NameMap> {
        Builder::new(self.name_map())
    }

    pub fn or(&mut self, first: Expression, second: Expression) -> Name {
        let first = self.expr(first);
        let cond = self.is_true(first);
        // output name for or (gets assigned two once in each branch)
        let out = self.name_map().create_tmp();
        let then = {
            let mut b = self.block();
            b.copy(out, first);
            b.finish_block()
        };
        let els = {
            let mut b = self.block();
            // only compute second within branch block
            let second = b.expr(second);
            // note order is dst, src
            b.copy(out, second);
            b.finish_block()
        };
        let stmt = Stmt::If {
            cond: cond,
            then: then,
            els: els,
        };
        self.push(stmt);
        out
    }

    pub fn copy(&mut self, dst: Name, src: Name) {
        self.def(dst, Expr::Copy(src));
    }

    pub fn int(&mut self, int: i32) -> Name {
        self.constant(Const::Int(int))
    }

    pub fn boolean(&mut self, b: bool) -> Name {
        self.constant(Const::Bool(b))
    }

    pub fn constant(&mut self, c: Const) -> Name {
        self.tmp(Expr::Const(c))
    }

    pub fn input(&mut self) -> Name {
        self.call_static("input", vec![])
    }

    pub fn is_true(&mut self, val: Name) -> Name {
        self.call_static("is_true", vec![val])
    }

    pub fn get_subscript(&mut self, base: Name, elem: Name) -> Name {
        self.call_static("get_subscript", vec![base, elem])
    }

    // Call a function with a known name
    pub fn call_static(&mut self, func: &str, args: Vec<Name>) -> Name {
        let func = self.name_map().insert_name(func);
        self.call(func, args)
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
