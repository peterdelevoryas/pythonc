use ast;
use util;
use std::collections::HashMap;

const MASK: i32 = 3;
const SHIFT: i32 = 2;

const INT_TAG: i32 = 0;
const BOOL_TAG: i32 = 1;
const FLOAT_TAG: i32 = 2;
const BIG_TAG: i32 = 3;

pub fn explicate(ast: ast::Module) -> Module {
    let mut builder = Builder::new();
    builder.module(ast)
}

pub mod var {
    /// Mutable reference, equivalent to Python's "Name",
    /// but called "Var" here to indicate that there
    /// can be multiple def's.
    impl_ref!(Var, "var");

    pub enum Data {
        /// A User created variable
        User {
            source_name: String,
        },
        /// A Compiler created temporary variable
        Temp,
    }
}
pub use self::var::Var;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    stmts: Vec<Stmt>,
}

impl_wrapper_enum! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Expr {
        boxed: [
            Let,
            ProjectTo,
            InjectFrom,
            CallFunc,
            CallRuntime,
            Binary,
            Unary,
            Subscript,
            List,
            Dict,
            IfExp,
            Closure
        ];
        simple: [
            Const,
            Var
        ];
    }
}

impl_wrapper_enum! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Stmt {
        boxed: [];
        simple: [
            Printnl,
            Assign,
            Expr,
            Return
        ];
    }
}

impl_wrapper_enum! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Target {
        boxed: [];
        simple: [
            Var,
            Subscript
        ];
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Let {
    var: Var,
    rhs: Expr,
    body: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectTo {
    to: Ty,
    expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InjectFrom {
    from: Ty,
    expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallRuntime {
    name: String,
    args: Vec<Expr>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ty {
    Int,
    Bool,
    Big,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallFunc {
    expr: Expr,
    args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binary {
    op: Binop,
    left: Expr,
    right: Expr,
}

// TODO There must be some better way
// to do this than having Big and Small
// versions of every binary operator.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Binop {
    /// Primitive addition operation
    Add,
    /// Primitive equals operation
    Eq,
    /// Primitive not equals operation
    NotEq,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unary {
    op: Unop,
    expr: Expr,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Unop {
    /// Primitive negation operation
    Neg,
    /// Not primitive! Just computes
    /// single bit not, not bitwise not
    /// of all bits.
    Not,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Const {
    Int(i32),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Printnl {
    expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign {
    target: Target,
    expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    exprs: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dict {
    tuples: Vec<(Expr, Expr)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subscript {
    base: Expr,
    elem: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfExp {
    cond: Expr,
    then: Expr,
    else_: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Closure {
    args: Vec<Var>,
    code: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Return {
    expr: Expr,
}

// =======------------ CONVENIENCE CONSTRUCTORS ------------======

pub fn if_exp<C, T, E>(c: C, t: T, e: E) -> IfExp
where
    C: Into<Expr>,
    T: Into<Expr>,
    E: Into<Expr>,
{
    IfExp {
        cond: c.into(),
        then: t.into(),
        else_: e.into(),
    }
}

pub fn let_<E, B>(var: Var, rhs: E, body: B) -> Let
where
    E: Into<Expr>,
    B: Into<Expr>,
{
    Let {
        var,
        rhs: rhs.into(),
        body: body.into(),
    }
}

pub fn binary<L, R>(op: Binop, left: L, right: R) -> Binary
where
    L: Into<Expr>,
    R: Into<Expr>,
{
    Binary {
        op,
        left: left.into(),
        right: right.into(),
    }
}

pub fn project_to<E>(expr: E, to: Ty) -> ProjectTo
where
    E: Into<Expr>,
{
    ProjectTo {
        to,
        expr: expr.into(),
    }
}

pub fn inject_from<E>(expr: E, from: Ty) -> InjectFrom
where
    E: Into<Expr>,
{
    InjectFrom {
        from,
        expr: expr.into(),
    }
}

pub fn return_<E>(expr: E) -> Return
where
    E: Into<Expr>,
{
    Return {
        expr: expr.into(),
    }
}

pub fn assign<T, E>(target: T, expr: E) -> Assign
where
    T: Into<Target>,
    E: Into<Expr>,
{
    Assign {
        target: target.into(),
        expr: expr.into(),
    }
}

impl Binop {
    pub fn ret_ty(self) -> Ty {
        match self {
            Binop::Add => Ty::Int,
            Binop::Eq => Ty::Bool,
            Binop::NotEq => Ty::Bool,
        }
    }
}

pub struct Builder {
    var_data: var::Slab<var::Data>,
    names: HashMap<ast::Name, Var>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            var_data: var::Slab::new(),
            names: HashMap::new(),
        }
    }

    pub fn module(&mut self, module: ast::Module) -> Module {
        let stmts = module.stmts.into_iter()
            .map(|stmt| self.stmt(stmt))
            .collect();
        Module {
            stmts
        }
    }

    pub fn stmt(&mut self, stmt: ast::Stmt) -> Stmt {
        match stmt {
            ast::Stmt::Function(f) => self.function(f).into(),
            ast::Stmt::Printnl(p) => self.printnl(p).into(),
            ast::Stmt::Assign(a) => self.assign(a).into(),
            ast::Stmt::Expr(e) => self.expr(e).into(),
            ast::Stmt::Return(r) => self.return_(r).into(),
        }
    }

    pub fn expr(&mut self, expr: ast::Expr) -> Expr {
        match expr {
            ast::Expr::Const(c) => self.const_(c).into(),
            ast::Expr::Name(n) => self.name(n).into(),
            ast::Expr::Add(box a) => self.add(a).into(),
            ast::Expr::UnarySub(box u) => self.unary_sub(u).into(),
            ast::Expr::CallFunc(box c) => self.call_func(c).into(),
            ast::Expr::Compare(box c) => self.compare(c).into(),
            ast::Expr::Or(box or) => self.or(or).into(),
            ast::Expr::And(box and) => self.and(and).into(),
            ast::Expr::Not(box not) => self.not(not).into(),
            ast::Expr::List(box list) => self.list(list).into(),
            ast::Expr::Dict(box dict) => self.dict(dict).into(),
            ast::Expr::Subscript(box s) => self.subscript(s).into(),
            ast::Expr::Lambda(box l) => self.lambda(l).into(),
            ast::Expr::IfExp(box if_exp) => self.if_exp(if_exp).into(),
            _ => unimplemented!()
        }
    }

    pub fn target(&mut self, target: ast::Target) -> Target {
        match target {
            ast::Target::Name(n) => self.name(n).into(),
            ast::Target::Subscript(s) => self.subscript(s).into(),
        }
    }

    pub fn function(&mut self, function: ast::Function) -> Assign {
        let name = self.name(ast::Name(function.name));
        let closure = Closure {
            args: function.args.into_iter().map(|arg| {
                self.name(ast::Name(arg))
            }).collect(),
            code: function.code.into_iter().map(|stmt| {
                self.stmt(stmt)
            }).collect(),
        };
        assign(name, closure)
    }

    pub fn printnl(&mut self, printnl: ast::Printnl) -> Printnl {
        Printnl {
            expr: self.expr(printnl.expr),
        }
    }

    pub fn assign(&mut self, assign: ast::Assign) -> Assign {
        Assign {
            target: self.target(assign.target),
            expr: self.expr(assign.expr),
        }
    }

    pub fn return_(&mut self, assign: ast::Return) -> Return {
        Return {
            expr: self.expr(assign.expr)
        }
    }

    pub fn const_(&mut self, const_: ast::Const) -> Const {
        Const::Int(const_.0)
    }

    pub fn name(&mut self, name: ast::Name) -> Var {
        if let Some(&var) = self.names.get(&name) {
            return var
        }
        let user_var = var::Data::User {
            source_name: name.0.clone(),
        };
        let var = self.var_data.insert(user_var);
        self.names.insert(name, var);
        var
    }

    pub fn add(&mut self, add: ast::Add) -> Let {
        self.binop(add.left, add.right, Binop::Add)
    }

    pub fn unary_sub(&mut self, unary_sub: ast::UnarySub) -> Unary {
        Unary {
            op: Unop::Neg,
            expr: self.expr(unary_sub.expr),
        }
    }

    pub fn call_func(&mut self, c: ast::CallFunc) -> CallFunc {
        CallFunc {
            expr: self.expr(c.expr),
            args: c.args.into_iter().map(|e| self.expr(e)).collect(),
        }
    }
    
    pub fn binop(&mut self, left: ast::Expr, right: ast::Expr, binop: Binop) -> Let {
        let left_expr = self.expr(left);
        let right_expr = self.expr(right);
        let left = self.new_temp();
        let right = self.new_temp();
        let small_result = inject_from(
            // TODO Is this correct, to always project to Int in case of smalls?
            binary(binop, project_to(left, Ty::Int), project_to(right, Ty::Int)),
            binop.ret_ty()
        );

        let big_result = match binop {
            Binop::Add => inject_from(CallRuntime {
                name: "add".into(),
                args: vec![
                    project_to(left, Ty::Big).into(),
                    project_to(right, Ty::Big).into(),
                ],
            }, Ty::Big),
            Binop::Eq => inject_from(CallRuntime {
                name: "equal".into(),
                args: vec![
                    project_to(left, Ty::Big).into(),
                    project_to(right, Ty::Big).into(),
                ]
            }, Ty::Bool),
            Binop::NotEq => inject_from(CallRuntime {
                name: "not_equal".into(),
                args: vec![
                    project_to(left, Ty::Big).into(),
                    project_to(right, Ty::Big).into(),
                ],
            }, Ty::Bool),
        };

        let_(left, left_expr, {
            let_(right, right_expr, {
                IfExp {
                    cond: binary(Binop::Eq,
                           CallRuntime {
                               name: "get_tag".into(),
                               args: vec![left.into()]
                           },
                           Const::Int(BIG_TAG)
                    ).into(),
                    then: big_result.into(),
                    else_: small_result.into(),
                }
            })
        })
    }

    pub fn compare(&mut self, c: ast::Compare) -> Let {
        unimplemented!()
    }

    pub fn or(&mut self, or: ast::Or) -> IfExp {
        unimplemented!()
    }

    pub fn and(&mut self, and: ast::And) -> IfExp {
        unimplemented!()
    }

    pub fn not(&mut self, not: ast::Not) -> Unary {
        unimplemented!()
    }

    pub fn list(&mut self, list: ast::List) -> List {
        List {
            exprs: list.exprs.into_iter().map(|e| self.expr(e)).collect(),
        }
    }

    pub fn dict(&mut self, dict: ast::Dict) -> Dict {
        Dict {
            tuples: dict.tuples.into_iter().map(|(l, r)| (self.expr(l), self.expr(r))).collect(),
        }
    }

    pub fn subscript(&mut self, s: ast::Subscript) -> Subscript {
        Subscript {
            base: self.expr(s.base),
            elem: self.expr(s.elem),
        }
    }

    pub fn lambda(&mut self, l: ast::Lambda) -> Closure {
        Closure {
            args: l.args.into_iter().map(|arg| self.name(ast::Name(arg))).collect(),
            code: vec![
                return_(self.expr(l.expr)).into(),
            ],
        }
    }

    pub fn if_exp(&mut self, e: ast::IfExp) -> IfExp {
        IfExp {
            cond: CallRuntime {
                name: "is_true".into(),
                args: vec![
                    self.expr(e.cond),
                ],
            }.into(),
            then: self.expr(e.then),
            else_: self.expr(e.else_),
        }
    }

    fn new_temp(&mut self) -> Var {
        let temp_data = var::Data::Temp;
        self.var_data.insert(temp_data)
    }
}
