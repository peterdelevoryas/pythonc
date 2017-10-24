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

    /// PRIMITIVE AND INSTRUCTION
    And(Name, Name),

    /// PRIMITIVE NOT, FLIPS ALL BITS
    Not(Name),

    Project {
        name: Name, // pyobj
        to: Ty,     // output value type
    },

    Inject {
        name: Name, // non-pyobj
        from: Ty,   // type of "name"
    },
}

pub enum Const {
    Int(i32),
    Bool(bool),
}

pub enum Ty {
    
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
                let int = self.project_small_to_int(e);
                let neg = self.neg(int);
                let pyobj = self.inject_int(neg);

                pyobj
            }

            LogicalOr(box first, box second) => {
                self.logical_or(first, second)
            }

            LogicalAnd(box first, box second) => {
                self.logical_and(first, second)
            }

            LogicalNot(box e) => {
                self.logical_not(e)
            }

            _ => unimplemented!()
        }
    }

    pub fn project_small_to_int(&mut self, pyobj: Name) -> Name {
        let is_big = self.is_big(pyobj);
        self.if_expr(
            is_big,
            |then| then.abort("invalid operand to neg: big_pyobj"),
            // If not big, then call project_int or project_bool
            |els| {
                let is_int = els.is_int(pyobj);
                els.if_expr(
                    is_int,
                    |then| then.project_int(pyobj),
                    |els| els.project_bool(pyobj),
                )
            },
        )
    }

    pub fn inject_int(&mut self, int: Name) -> Name {
        self.call_static("inject_int", vec![int])
    }

    pub fn project_big(&mut self, pyobj: Name) -> Name {
        self.call_static("project_big", vec![pyobj])
    }

    pub fn project_bool(&mut self, pyobj: Name) -> Name {
        self.call_static("project_bool", vec![pyobj])
    }

    pub fn project_int(&mut self, pyobj: Name) -> Name {
        self.call_static("project_int", vec![pyobj])
    }

    pub fn abort(&mut self, msg: &'static str) -> Name {
        // Can't really do anything with a static str yet, so just throwing away.
        self.call_static("abort", vec![])
    }

    // Takes current stmts and returns them,
    // allowing NameMap to drop
    pub fn finish(self) -> Block {
        self.block
    }

    pub fn block<F>(&mut self, build: F) -> Block
    where
        F: FnOnce(&mut Builder<&mut NameMap>)
    {
        let mut block = self.builder();
        build(&mut block);
        block.finish()
    }

    pub fn block_expr<F>(&mut self, build: F) -> (Name, Block)
    where
        F: FnOnce(&mut Builder<&mut NameMap>) -> Name
    {
        let mut block = self.builder();
        let name = build(&mut block);
        (name, block.finish())
    }

    pub fn builder(&mut self) -> Builder<&mut NameMap> {
        Builder::new(self.name_map())
    }

    pub fn not(&mut self, val: Name) -> Name {
        self.tmp(Expr::Not(val))
    }

    pub fn and(&mut self, l: Name, r: Name) -> Name {
        self.tmp(Expr::And(l, r))
    }

    pub fn mask(&mut self, val: Name, mask: i32) -> Name {
        let mask = self.int(mask);
        self.and(val, mask)
    }

    pub fn neg(&mut self, val: Name) -> Name {
        self.tmp(Expr::Neg(val))
    }

    pub fn logical_not(&mut self, e: Expression) -> Name {
        let e = self.expr(e);
        let is_true = self.is_true(e);
        let flipped = self.not(is_true);
        let first_bit = self.mask(flipped, 1);
        first_bit
    }

    pub fn logical_and(&mut self, first: Expression, second: Expression) -> Name {
        let first = self.expr(first);
        let cond = self.is_true(first);
        self.if_expr(cond, move |b| b.expr(second), |b| first)
    }

    pub fn logical_or(&mut self, first: Expression, second: Expression) -> Name {
        let first = self.expr(first);
        let cond = self.is_true(first);
        self.if_expr(cond, |b| first, move |b| b.expr(second))
    }

    pub fn if_expr<F1, F2>(&mut self, cond: Name, then: F1, els: F2) -> Name
    where
        F1: FnOnce(&mut Builder<&mut NameMap>) -> Name,
        F2: FnOnce(&mut Builder<&mut NameMap>) -> Name,
    {
        let (then_expr, mut then) = self.block_expr(then);
        let (els_expr, mut els) = self.block_expr(els);
        // This is pretty hacky!!!
        // Modify each block by appending an assignment to "out".
        let out = self.name_map().create_tmp();
        then.stmts.push(Stmt::Def(out, Expr::Copy(then_expr)));
        els.stmts.push(Stmt::Def(out, Expr::Copy(els_expr)));
        self.push(Stmt::If {
            cond: cond,
            then: then,
            els: els,
        });

        out
    }

    pub fn if_stmt<F1, F2>(&mut self, cond: Name, then: F1, els: F2)
    where
        F1: FnOnce(&mut Builder<&mut NameMap>),
        F2: FnOnce(&mut Builder<&mut NameMap>),
    {
        let then = self.block(then);
        let els = self.block(els);
        self.push(Stmt::If {
            cond: cond,
            then: then,
            els: els,
        });
    }

    pub fn is(&mut self, left: Name, right: Name) -> Name {
        self.tmp(Expr::Is(left, right))
    }

    pub fn is_int(&mut self, val: Name) -> Name {
        let tag = self.tag(val);
        let int_tag = self.int_tag();
        self.is(tag, int_tag)
    }

    pub fn is_big(&mut self, pyobj: Name) -> Name {
        let tag = self.tag(pyobj);
        let big_tag = self.big_tag();
        self.is(tag, big_tag)
    }

    pub fn big_tag(&mut self) -> Name {
        self.int(3)
    }

    pub fn int_tag(&mut self) -> Name {
        // INT_TAG is 0
        self.int(0)
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

    pub fn tag(&mut self, val: Name) -> Name {
        self.call_static("tag", vec![val])
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
