use ast;
use std::collections::HashMap;
use raise::Func;

const MASK: i32 = 3;
const SHIFT: i32 = 2;

const INT_TAG: i32 = 0;
const BOOL_TAG: i32 = 1;
const FLOAT_TAG: i32 = 2;
const BIG_TAG: i32 = 3;

macro_rules! set {
    ($($e:expr),*) => ({
        let mut set = HashSet::new();
        $(
            set.insert($e);
        )*
        set
    })
}

macro_rules! set_union {
    ($($e:expr),*) => ({
        let mut set = HashSet::new();
        $(
            set.extend($e);
        )*
        set
    })
}

pub fn explicate(ast: ast::Module) -> Module {
    let mut explicate = Explicate::new();
    explicate.module(ast)
}

pub mod var {
    /// Mutable reference, equivalent to Python's "Name",
    /// but called "Var" here to indicate that there
    /// can be multiple def's.
    impl_ref!(Var, "var");

    #[derive(Debug)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub stmts: Vec<Stmt>,
}

impl_wrapper_enum! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
            Var,
            Func
        ];
    }
}

impl_wrapper_enum! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Target {
        boxed: [];
        simple: [
            Var,
            Subscript
        ];
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Let {
    pub var: Var,
    pub rhs: Expr,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectTo {
    pub to: Ty,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InjectFrom {
    pub from: Ty,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallRuntime {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Int,
    Bool,
    Big,
    Pyobj,
    Func,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallFunc {
    pub expr: Expr,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Binary {
    pub op: Binop,
    pub left: Expr,
    pub right: Expr,
}

// TODO There must be some better way
// to do this than having Big and Small
// versions of every binary operator.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Binop {
    /// Primitive addition operation
    Add,
    /// Primitive equals operation
    Eq,
    /// Primitive not equals operation
    NotEq,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Unary {
    pub op: Unop,
    pub expr: Expr,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Unop {
    /// Primitive negation operation
    Neg,
    /// Not primitive! Just computes
    /// single bit not, not bitwise not
    /// of all bits.
    Not,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Const {
    Int(i32),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Printnl {
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Assign {
    pub target: Target,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct List {
    pub exprs: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dict {
    pub tuples: Vec<(Expr, Expr)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Subscript {
    pub base: Expr,
    pub elem: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfExp {
    pub cond: Expr,
    pub then: Expr,
    pub else_: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Closure {
    pub args: Vec<Var>,
    pub code: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Return {
    pub expr: Expr,
}

impl Ty {
    pub fn is_small(self) -> bool {
        match self {
            Ty::Int | Ty::Bool => true,
            _ => false,
        }
    }

    // Should Func be considered big??
    pub fn is_big(self) -> bool {
        match self {
            Ty::Big => true,
            _ => false,
        }
    }
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

impl Unop {
    pub fn ret_ty(self) -> Ty {
        match self {
            Unop::Neg => Ty::Int,
            Unop::Not => Ty::Bool,
        }
    }
}

#[derive(Debug)]
pub struct Explicate {
    pub var_data: var::Slab<var::Data>,
    names: HashMap<ast::Name, Var>,
}

const INTRINSICS: &[(&str, Ty)] = &[
    ("True", Ty::Pyobj),
    ("False", Ty::Pyobj),
    ("input", Ty::Pyobj),
];

impl Explicate {
    pub fn new() -> Explicate {
        let mut b = Explicate {
            var_data: var::Slab::new(),
            names: HashMap::new(),
        };
        // All the pre-defined intrinsics
        for &(name, _) in INTRINSICS {
            b.name(ast::Name(name.into()));
        }

        return b
    }

    pub fn lookup_var(&self, name: &ast::Name) -> Var {
        self.names[name]
    }

    fn force_insert_name(&mut self, name: &ast::Name) {
        let user_var = var::Data::User {
            source_name: name.0.clone(),
        };
        let var = self.var_data.insert(user_var);
        self.names.insert(name.clone(), var);
    }

    pub fn add_names_in_module(&mut self, module: &ast::Module) {
        for stmt in &module.stmts {
            if let ast::Stmt::Assign(ref assign) = *stmt {
                if let ast::Target::Name(ref name) = assign.target {
                    self.force_insert_name(name)
                }
            }
        }
    }

    pub fn add_names_in_function(&mut self, function: &ast::Function) {
        // relying on args and name getting set in body of function
        for stmt in &function.code {
            if let ast::Stmt::Assign(ref assign) = *stmt {
                if let ast::Target::Name(ref name) = assign.target {
                    self.force_insert_name(name)
                }
            }
        }
        for arg in &function.args {
            self.force_insert_name(&ast::Name(arg.clone()));
        }
    }

    pub fn add_names_in_lambda(&mut self, lambda: &ast::Lambda) {
        for arg in &lambda.args {
            self.force_insert_name(&ast::Name(arg.clone()));
        }
    }

    pub fn module(&mut self, module: ast::Module) -> Module {
        self.add_names_in_module(&module);
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
        }
    }

    pub fn target(&mut self, target: ast::Target) -> Target {
        match target {
            ast::Target::Name(name) => {
                self.name(name).into()
            }
            ast::Target::Subscript(s) => self.subscript(s).into(),
        }
    }

    pub fn function(&mut self, function: ast::Function) -> Assign {
        let name = self.name(ast::Name(function.name.clone()));
        let saved = self.names.clone();
        self.add_names_in_function(&function);
        let closure = Closure {
            args: function.args.into_iter().map(|arg| {
                self.name(ast::Name(arg))
            }).collect(),
            code: function.code.into_iter().map(|stmt| {
                self.stmt(stmt)
            }).collect(),
        };
        // restore the name environment
        self.names = saved;
        assign(name, inject_from(closure, Ty::Big))
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

    pub fn const_(&mut self, const_: ast::Const) -> InjectFrom {
        inject_from(Const::Int(const_.0), Ty::Int)
    }

    pub fn name(&mut self, name: ast::Name) -> Var {
        if let Some(&var) = self.names.get(&name) {
            return var
        }
        let user_var = var::Data::User {
            source_name: name.0.clone(),
        };
        let var = self.var_data.insert(user_var);
        self.names.insert(name.clone(), var);
        var
    }

    pub fn add(&mut self, add: ast::Add) -> Let {
        self.binop(add.left, add.right, Binop::Add)
    }

    pub fn unary_sub(&mut self, unary_sub: ast::UnarySub) -> InjectFrom {
        inject_from(
            Unary {
                op: Unop::Neg,
                expr: project_to(self.expr(unary_sub.expr), Ty::Int).into(),
            },
            Ty::Int
        )
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
        let binop = match c.op {
            ast::CompareOp::Eq => Binop::Eq,
            ast::CompareOp::NotEq => Binop::NotEq,
        };
        self.binop(c.left, c.right, binop)
    }

    pub fn or(&mut self, or: ast::Or) -> Let {
        let first_expr = self.expr(or.left);
        let first = self.new_temp();
        let_(first, first_expr, {
            IfExp {
                cond: CallRuntime {
                    name: "is_true".into(),
                    args: vec![
                        first.into(),
                    ],
                }.into(),
                then: first.into(),
                else_: self.expr(or.right),
            }
        })
    }

    pub fn and(&mut self, and: ast::And) -> Let {
        let first_expr = self.expr(and.left);
        let first = self.new_temp();
        let_(first, first_expr, {
            IfExp {
                cond: CallRuntime {
                    name: "is_true".into(),
                    args: vec![
                        first.into(),
                    ],
                }.into(),
                then: self.expr(and.right),
                else_: first.into(),
            }
        })
    }

    pub fn not(&mut self, not: ast::Not) -> ProjectTo {
        let is_true = CallRuntime {
            name: "is_true".into(),
            args: vec![
                self.expr(not.expr),
            ],
        };
        let logical_not = Unary {
            op: Unop::Not,
            expr: is_true.into(),
        };
        let pyobj = project_to(logical_not, Ty::Bool);
        pyobj
    }

    pub fn list(&mut self, list: ast::List) -> InjectFrom {
        let list = List {
            exprs: list.exprs.into_iter().map(|e| self.expr(e)).collect(),
        };
        inject_from(list, Ty::Big)
    }

    pub fn dict(&mut self, dict: ast::Dict) -> InjectFrom {
        let dict = Dict {
            tuples: dict.tuples.into_iter().map(|(l, r)| (self.expr(l), self.expr(r))).collect(),
        };
        inject_from(dict, Ty::Big)
    }

    pub fn subscript(&mut self, s: ast::Subscript) -> Subscript {
        Subscript {
            base: self.expr(s.base),
            elem: self.expr(s.elem),
        }
    }

    pub fn lambda(&mut self, l: ast::Lambda) -> InjectFrom {
        let saved = self.names.clone();
        self.add_names_in_lambda(&l);
        let closure = Closure {
            args: l.args.into_iter().map(|arg| self.name(ast::Name(arg))).collect(),
            code: vec![
                return_(self.expr(l.expr)).into()
            ],
        };
        self.names = saved;
        inject_from(closure, Ty::Big)
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

    pub fn var_data(&self, var: Var) -> &var::Data {
        &self.var_data[var]
    }

    pub fn formatter<'a, N: 'a + ?Sized>(&'a self, node: &'a N, show_casts: bool) -> Formatter<'a, N> {
        Formatter::new(self, node, show_casts)
    }
}

#[derive(Debug)]
pub struct Formatter<'a, N: 'a + ?Sized> {
    explicate: &'a Explicate,
    node: &'a N,
    indent: usize,
    show_casts: bool,
}

impl<'a, N: 'a + ?Sized> Formatter<'a, N> {
    pub fn new(explicate: &'a Explicate, node: &'a N, show_casts: bool) -> Formatter<'a, N> {
        Formatter {
            explicate,
            node,
            indent: 0,
            show_casts,
        }
    }

    pub fn fmt<M: 'a + ?Sized>(&self, node: &'a M) -> Formatter<'a, M> {
        Formatter {
            explicate: self.explicate,
            node,
            indent: self.indent,
            show_casts: self.show_casts,
        }
    }

    pub fn indented<M: 'a + ?Sized + fmt::Debug>(&self, node: &'a M) -> Formatter<'a, M> {
        Formatter {
            explicate: self.explicate,
            node,
            indent: self.indent + 1,
            show_casts: self.show_casts,
        }
    }

    pub fn width(&self) -> usize {
        self.indent * 4
    }

    pub fn indent(&self) -> String {
        " ".repeat(self.width())
    }
}

use std::fmt;

impl<'a> fmt::Display for Formatter<'a, ::raise::TransUnit> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "func main() -> i32 {{")?;
        writeln!(
            f,
            "{}{}",
            self.indent(),
            self.fmt(&self.node.funcs[self.node.main])
        )?;
        writeln!(f, "{}}}", self.indent())?;
        writeln!(f)?;
        for (func, data) in &self.node.funcs {
            if func == self.node.main {
                continue
            }
            let (_, params): (_, String) = data.args.iter().map(|var| {
                format!("{}", self.fmt(var))
            }).fold((true, "".into()), |(first, mut acc), s| {
                if !first {
                    acc.push_str(", ");
                }
                acc.push_str(&s);
                (false, acc)
            });
            writeln!(f, "{indent}func {func}({params}) -> i32 {{",
                indent=self.indent(),
                func=func,
                params=params)?;
            writeln!(
                f,
                "{}{}",
                self.indent(),
                self.fmt(&self.node.funcs[func])
            )?;
            writeln!(f, "{}}}", self.indent())?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Formatter<'a, ::raise::func::Data> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt(self.node.body.stmts.as_slice()))
    }
}

impl<'a> fmt::Display for Formatter<'a, Module> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use explicate::FreeVars;
        write!(f, "{}", self.indent())?;
        writeln!(f, "module {{")?;
        let free_vars: Vec<Var> = self.node.free_vars().into_iter().collect();
        writeln!(f, ".free_vars: {}", self.fmt(free_vars.as_slice()))?;
        for stmt in &self.node.stmts {
            write!(f, "{}", self.indent())?;
            writeln!(f, "{}", self.indented(stmt))?;
        }
        write!(f, "{}", self.indent())?;
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl<'a> fmt::Display for Formatter<'a, [Stmt]> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in self.node {
            write!(f, "{}", self.indent())?;
            writeln!(f, "{}", self.indented(stmt))?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Formatter<'a, Stmt> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.indent())?;
        use self::Stmt::*;
        match *self.node {
            Printnl(ref p) => write!(f, "{}", self.fmt(p)),
            Assign(ref a) => write!(f, "{}", self.fmt(a)),
            Expr(ref e) => write!(f, "{}", self.fmt(e)),
            Return(ref r) => write!(f, "{}", self.fmt(r)),
        }
    }
}

impl<'a> fmt::Display for Formatter<'a, Expr> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self.node {
            Let(box ref e) => write!(f, "{}", self.fmt(e)),
            ProjectTo(box ref p) => write!(f, "{}", self.fmt(p)),
            InjectFrom(box ref i) => write!(f, "{}", self.fmt(i)),
            CallFunc(box ref c) => write!(f, "{}", self.fmt(c)),
            CallRuntime(box ref c) => write!(f, "{}", self.fmt(c)),
            Binary(box ref b) => write!(f, "{}", self.fmt(b)),
            Unary(box ref u) => write!(f, "{}", self.fmt(u)),
            Subscript(box ref s) => write!(f, "{}", self.fmt(s)),
            List(box ref l) => write!(f, "{}", self.fmt(l)),
            Dict(box ref d) => write!(f, "{}", self.fmt(d)),
            IfExp(box ref i) => write!(f, "{}", self.fmt(i)),
            Closure(box ref c) => write!(f, "{}", self.fmt(c)),
            Const(ref c) => write!(f, "{}", self.fmt(c)),
            Var(ref v) => write!(f, "{}", self.fmt(v)),
            Func(ref func) => write!(f, "{}", func),
        }
    }
}

impl<'a> fmt::Display for Formatter<'a, Target> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Target::*;
        match *self.node {
            Var(ref v) => write!(f, "{}", self.fmt(v)),
            Subscript(ref s) => write!(f, "{}", self.fmt(s)),
        }
    }
}

impl<'a> fmt::Display for Formatter<'a, Printnl> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "print {}", self.fmt(&self.node.expr))
    }
}

impl<'a> fmt::Display for Formatter<'a, Assign> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}",
               self.fmt(&self.node.target),
               self.fmt(&self.node.expr))
    }
}

impl<'a> fmt::Display for Formatter<'a, Return> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "return {}", self.fmt(&self.node.expr))
    }
}

impl<'a> fmt::Display for Formatter<'a, Let> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // formats let x = 3 in expr as:
        // {
        //     let x = 3;
        //     expr
        // }
        // I think it's way easier to read this way
        writeln!(f, "{{")?;
        writeln!(f, "{indented}let {lhs} = {rhs};",
                 indented=self.indented(&self.node.var).indent(),
                 lhs=self.fmt(&self.node.var),
                 rhs=self.indented(&self.node.rhs))?;
        writeln!(f, "{indented}{expr}",
                 indented=self.indented(&self.node.var).indent(),
                 expr=self.indented(&self.node.body))?;
        writeln!(f, "{indent}}}", indent=self.indent())?;
        Ok(())
    }
}

impl<'a> fmt::Display for Formatter<'a, ProjectTo> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.show_casts {
            writeln!(f, "@project_to::<{}>(", self.fmt(&self.node.to))?;
            writeln!(f, "{}{}", self.indented(&self.node.expr).indent(), self.indented(&self.node.expr))?;
            write!(f, "{})", self.indent())
        } else {
            write!(f, "{}", self.fmt(&self.node.expr))
        }
    }
}

impl<'a> fmt::Display for Formatter<'a, InjectFrom> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.show_casts {
            writeln!(f, "@inject_from::<{}>(", self.fmt(&self.node.from))?;
            writeln!(f, "{}{}", self.indented(&self.node.expr).indent(), self.indented(&self.node.expr))?;
            write!(f, "{})", self.indent())
        } else {
            write!(f, "{}", self.fmt(&self.node.expr))
        }
    }
}

impl<'a> fmt::Display for Formatter<'a, CallFunc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "call_func({}, {})", self.fmt(&self.node.expr), self.fmt(self.node.args.as_slice()))
    }
}

impl<'a> fmt::Display for Formatter<'a, CallRuntime> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "call_runtime({}, {})", self.node.name, self.fmt(self.node.args.as_slice()))
    }
}

impl<'a> fmt::Display for Formatter<'a, Binary> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({}, {})",
            self.fmt(&self.node.op),
            self.fmt(&self.node.left),
            self.fmt(&self.node.right))
    }
}

impl<'a> fmt::Display for Formatter<'a, Binop> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.node {
            Binop::Add => write!(f, "add"),
            Binop::Eq => write!(f, "eq"),
            Binop::NotEq => write!(f, "not_eq"),
        }
    }
}

impl<'a> fmt::Display for Formatter<'a, Unary> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.fmt(&self.node.op), self.fmt(&self.node.expr))
    }
}

impl<'a> fmt::Display for Formatter<'a, Unop> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.node {
            Unop::Neg => write!(f, "neg"),
            Unop::Not => write!(f, "not"),
        }
    }
}

impl<'a> fmt::Display for Formatter<'a, Subscript> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{}]", self.fmt(&self.node.base), self.fmt(&self.node.elem))
    }
}

impl<'a> fmt::Display for Formatter<'a, List> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt(self.node.exprs.as_slice()))
    }
}

impl<'a> fmt::Display for Formatter<'a, Dict> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt(self.node.tuples.as_slice()))
    }
}

impl<'a> fmt::Display for Formatter<'a, IfExp> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "if {} {{", self.fmt(&self.node.cond))?;
        writeln!(f, "{}{}", self.indented(&self.node.then).indent(), self.indented(&self.node.then))?;
        writeln!(f, "{}}} else {{", self.indent())?;
        writeln!(f, "{}{}", self.indented(&self.node.else_).indent(), self.indented(&self.node.else_))?;
        write!(f, "{}}}", self.indent())
    }
}

impl<'a> fmt::Display for Formatter<'a, Closure> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "closure{} {{", self.fmt(self.node.args.as_slice()))?;
        let free_vars: Vec<Var> = self.node.free_vars().into_iter().collect();
        writeln!(f, "{}.free_vars: {}", self.indent(), self.fmt(free_vars.as_slice()))?;
        for stmt in &self.node.code {
            writeln!(f, "{}", self.indented(stmt))?;
        }
        write!(f, "{}", self.indent())?;
        writeln!(f, "}}")
    }
}

impl<'a> fmt::Display for Formatter<'a, Const> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Const::*;
        match *self.node {
            Int(i) => write!(f, "{}", i),
            Bool(b) => write!(f, "{}", b),
        }
    }
}

impl<'a> fmt::Display for Formatter<'a, Var> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.explicate.var_data(*self.node) {
            var::Data::User {
                ref source_name
            } => write!(f, "{}.{}", source_name, self.node.inner()),
            var::Data::Temp => write!(f, "%{}", self.node.inner()),
        }
    }
}

impl<'a> fmt::Display for Formatter<'a, Ty> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.node {
            Ty::Int => write!(f, "int"),
            Ty::Bool => write!(f, "bool"),
            Ty::Big => write!(f, "big"),
            Ty::Pyobj => write!(f, "pyobj"),
            Ty::Func => write!(f, "func"),
        }
    }
}

impl<'a> fmt::Display for Formatter<'a, [Expr]> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        if !self.node.is_empty() {
            write!(f, "{}", self.fmt(&self.node[0]))?;
            for expr in &self.node[1..] {
                write!(f, ", {}", self.fmt(expr))?;
            }
        }
        write!(f, "]")
    }
}

impl<'a> fmt::Display for Formatter<'a, [(Expr, Expr)]> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        if !self.node.is_empty() {
            write!(f, "{}: {}",
                self.fmt(&self.node[0].0),
                self.fmt(&self.node[0].1),
            )?;
            for tuple in &self.node[1..] {
                write!(f, ", {}: {})",
                    self.fmt(&tuple.0),
                    self.fmt(&tuple.1),
                )?;
            }
        }
        write!(f, "}}")
    }
}

impl<'a> fmt::Display for Formatter<'a, [Var]> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        if !self.node.is_empty() {
            write!(f, "{}", self.fmt(&self.node[0]))?;
            for expr in &self.node[1..] {
                write!(f, ", {}", self.fmt(expr))?;
            }
        }
        write!(f, ")")
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Ty::*;
        let ty = match *self {
            Int => "int",
            Bool => "bool",
            Big => "*big_pyobj",
            Pyobj => "pyobj",
            Func => "func",
        };
        write!(f, "{}", ty)
    }
}

use error::*;

#[derive(Debug)]
pub struct TypeEnv<'a> {
    // Used for debug messages
    explicate: &'a Explicate,
    type_map: HashMap<Target, Option<Ty>>,
}

impl<'a> TypeEnv<'a> {
    pub fn new(explicate: &'a Explicate) -> Self {
        let type_map = {
            let mut map = HashMap::new();
            // initialize intrinsics with types
            for &(name, ty) in INTRINSICS {
                let var = explicate.lookup_var(&ast::Name(name.into()));
                map.insert(var.into(), Some(ty));
            }
            map
        };
        TypeEnv {
            explicate,
            type_map,
        }
    }

    pub fn def(&mut self, target: Target, ty: Option<Ty>) {
        self.type_map.insert(target, ty);
    }

    pub fn ty(&self, target: Target) -> Result<Option<Ty>> {
        match self.type_map.get(&target) {
            Some(&ty) => Ok(ty),
            //None => bail!("Type env doesn't contain {}",
                          //Formatter::new(self.explicate, &target)),
            // Allow names that get defined later to happen
            None => Ok(None)
        }
    }

    pub fn fmt<M: 'a + ?Sized>(&self, node: &'a M) -> Formatter<'a, M> {
        Formatter::new(self.explicate, node, true)
    }
}

pub trait TypeCheck {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>>;
}

impl TypeCheck for Module {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        for stmt in &self.stmts {
            let _ = stmt.type_check(env).chain_err(|| {
                format!("Error type checking stmt: {}", env.fmt(stmt))
            })?;
        }
        Ok(None)
    }
}

impl TypeCheck for Stmt {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        use self::Stmt::*;
        match *self {
            Printnl(ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking printnl: {}", env.fmt(inner))
                }),
            Assign(ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking assign: {}", env.fmt(inner))
                }),

            Expr(ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking expr: {}", env.fmt(inner))
                }),
            Return(ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking return: {}", env.fmt(inner))
                }),
        }
    }
}

impl TypeCheck for Printnl {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        let _ = self.expr.type_check(env).chain_err(|| {
            format!("Error type checking expr: {}", env.fmt(&self.expr))
        })?;
        Ok(None)
    }
}

impl TypeCheck for Assign {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        // Allow recursive functions
        env.def(self.target.clone(), None);
        let expr_ty = self.expr.type_check(env)?;
        env.def(self.target.clone(), expr_ty);
        Ok(None)
    }
}

impl TypeCheck for Return {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        self.expr.type_check(env)
    }
}

impl TypeCheck for Expr {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        use self::Expr::*;
        match *self {
            Let(box ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking let: {}", env.fmt(inner))
                }),

            ProjectTo(box ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking project to: {}", env.fmt(inner))
                }),

            InjectFrom(box ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking inject from: {}", env.fmt(inner))
                }),

            CallFunc(box ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking call func: {}", env.fmt(inner))
                }),

            CallRuntime(box ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking call runtime: {}", env.fmt(inner))
                }),

            Binary(box ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking binary: {}", env.fmt(inner))
                }),

            Unary(box ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking unary: {}", env.fmt(inner))
                }),

            Subscript(box ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking subscript: {}", env.fmt(inner))
                }),

            List(box ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking list: {}", env.fmt(inner))
                }),

            Dict(box ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking dict: {}", env.fmt(inner))
                }),

            IfExp(box ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking ifexp: {}", env.fmt(inner))
                }),

            Closure(box ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking closure: {}", env.fmt(inner))
                }),

            Const(ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking const: {}", env.fmt(inner))
                }),

            Var(ref inner) => inner.type_check(env)
                .chain_err(|| {
                    format!("Error type checking var: {}", env.fmt(inner))
                }),

            Func(ref func) => {
                bail!("Func's shouldn't be in the AST before type checking!")
            }
        }
    }
}

impl TypeCheck for Let {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        let rhs_ty = self.rhs.type_check(env)?;
        env.def(self.var.into(), rhs_ty);
        self.body.type_check(env)
    }
}

impl TypeCheck for ProjectTo {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        if self.to == Ty::Pyobj {
            bail!("ProjectTo Pyobj is invalid")
        }
        let ty = self.expr.type_check(env)?;
        match ty {
            Some(ty) if ty == Ty::Pyobj => Ok(Some(self.to)),
            Some(invalid_ty) => bail!("Cannot project from {} to {}", invalid_ty, self.to),
            None => Ok(Some(self.to))
        }
    }
}

impl TypeCheck for InjectFrom {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        if self.from == Ty::Pyobj {
            bail!("InjectFrom Pyobj is invalid")
        }
        let ty = match self.expr.type_check(env)? {
            Some(ty) => ty,
            // Can't do any type checking, just return output ty
            None => return Ok(Some(Ty::Pyobj))
        };
        match (self.from, ty) {
            // let smalls work with each other
            (Ty::Int, Ty::Int) => {}
            (Ty::Int, Ty::Bool) => {}
            (Ty::Bool, Ty::Bool) => {}
            (Ty::Bool, Ty::Int) => {}
            // big to big
            (Ty::Big, Ty::Big) => {}
            // don't allow any other injects
            (from, actual) => bail!("Cannot call @inject_from::<{}> on expr {} with type {}", from, env.fmt(&self.expr), actual)
        }
        Ok(Some(Ty::Pyobj))
    }
}

impl TypeCheck for CallFunc {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        let expr_ty = self.expr.type_check(env)?;
        expect_ty(expr_ty, Ty::Pyobj)
            .chain_err(|| format!("call target {} is not a func", env.fmt(&self.expr)))?;
        for arg in &self.args {
            let ty = arg.type_check(env)?;
            expect_ty(ty, Ty::Pyobj)
                .chain_err(|| "call argument is not a pyobj")?;
        }
        Ok(None)
    }
}

impl TypeCheck for CallRuntime {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        let (ret_ty, arg_ty, args_len) = match self.name.as_str() {
            "add" => (Ty::Big, Ty::Big, 2),
            "equal" | "not_equal" => (Ty::Int, Ty::Big, 2),
            "is_true" | "get_tag" => (Ty::Int, Ty::Pyobj, 1),
            _ => {
                 unimplemented!()
            }
        };
        if self.args.len() != args_len {
            bail!("incorrect number of arguments to runtime function")
        }
        for arg in &self.args {
            let ty = arg.type_check(env)?;
            expect_ty(ty, arg_ty)
                .chain_err(|| format!("Invalid argument type {} to {}", ty.unwrap(), self.name))?;
        }
        Ok(Some(ret_ty))
    }
}

impl TypeCheck for Binary {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        let left = self.left.type_check(env)?;
        expect_ty(left, Ty::Int)
            .chain_err(|| "left arg to binary is not int")?;
        let right = self.right.type_check(env)
            .chain_err(|| "right arg to binary is not int")?;
        Ok(Some(Ty::Int))
    }
}

impl TypeCheck for Unary {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        let expr = self.expr.type_check(env)?;
        expect_ty(expr, Ty::Int)
            .chain_err(|| "arg to unary is not int")?;
        Ok(Some(Ty::Int))
    }
}

impl TypeCheck for Subscript {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        if let Some(base_ty) = self.base.type_check(env)? {
            if base_ty != Ty::Pyobj {
                bail!("base of subscript is not pyobj: {}", base_ty)
            }
        }
        if let Some(elem_ty) = self.elem.type_check(env)? {
            if elem_ty != Ty::Pyobj {
                bail!("elem of subscript is not pyobj: {}", elem_ty)
            }
        }
        Ok(Some(Ty::Pyobj))
    }
}

impl TypeCheck for List {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        for expr in &self.exprs {
            let ty = expr.type_check(env)?;
            expect_ty(ty, Ty::Pyobj)
                .chain_err(|| "Elem of list is not a pyobj")?;
        }
        // Or should this be Pyobj?
        Ok(Some(Ty::Big))
    }
}

fn expect_ty(ty: Option<Ty>, expected: Ty) -> Result<()> {
    match ty {
        Some(ty) if ty == expected => Ok(()),
        Some(invalid_ty) => bail!("expected {}, got {}", expected, invalid_ty),
        None => Ok(())
    }
}

impl TypeCheck for Dict {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        for &(ref k, ref v) in &self.tuples {
            let key_ty = k.type_check(env)?;
            expect_ty(key_ty, Ty::Pyobj)
                .chain_err(|| "key type in dict is not Pyobj")?;
            let val_ty = v.type_check(env)?;
            expect_ty(val_ty, Ty::Pyobj)
                .chain_err(|| "val type in dict is not Pyobj")?;
        }
        Ok(Some(Ty::Big))
    }
}

impl TypeCheck for IfExp {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        let cond_ty = match self.cond.type_check(env)? {
            Some(ty) => ty,
            None => bail!("Type of condition is not known, should be impossible"),
        };
        if cond_ty != Ty::Int {
            bail!("Type of condition in IfExp is not int: {}",
                  Formatter::new(env.explicate, self, true))
        }
        let then_ty = self.then.type_check(env)?;
        let else_ty = self.else_.type_check(env)?;
        // If both branches have same output type,
        // then return that. Otherwise, None
        if then_ty == else_ty {
            Ok(then_ty)
        } else {
            Ok(None)
        }
    }
}

impl TypeCheck for Closure {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        for &arg in &self.args {
            env.def(arg.into(), None);
        }
        let mut ret_ty: Option<Option<Ty>> = None;
        for stmt in &self.code {
            match *stmt {
                Stmt::Return(ref r) => {
                    let ty = r.type_check(env)?;
                    ret_ty = match ret_ty {
                        Some(ref ret_ty) if *ret_ty != ty => None,
                        _ => Some(ty),
                    };
                }
                ref stmt => {
                    let _ = stmt.type_check(env)?;
                }
            }
        }
        // above figures out what the return type is,
        // but then we don't really have anything to do with it
        Ok(Some(Ty::Big))
    }
}

impl TypeCheck for Const {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        match *self {
            Const::Int(_) => Ok(Some(Ty::Int)),
            Const::Bool(_) => Ok(Some(Ty::Bool)),
        }
    }
}

impl TypeCheck for Var {
    fn type_check(&self, env: &mut TypeEnv) -> Result<Option<Ty>> {
        env.ty((*self).into())
    }
}

use std::collections::HashSet;

pub trait AddFreeVars {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>);
}

impl AddFreeVars for Module {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        self.stmts.add_free_vars(free_vars);
    }
}

impl AddFreeVars for [Stmt] {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        for stmt in self {
            stmt.add_free_vars(free_vars);
        }
    }
}

impl AddFreeVars for Stmt {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        use explicate::Stmt::*;
        match *self {
            Printnl(ref x) => x.add_free_vars(free_vars),
            Assign(ref x) => x.add_free_vars(free_vars),
            Expr(ref x) => x.add_free_vars(free_vars),
            Return(ref x) => x.add_free_vars(free_vars),
        }
    }
}

impl AddFreeVars for Expr {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        use explicate::Expr::*;
        match *self {
            Let(ref x) => x.add_free_vars(free_vars),
            ProjectTo(ref x) => x.add_free_vars(free_vars),
            InjectFrom(ref x) => x.add_free_vars(free_vars),
            CallFunc(ref x) => x.add_free_vars(free_vars),
            CallRuntime(ref x) => x.add_free_vars(free_vars),
            Binary(ref x) => x.add_free_vars(free_vars),
            Unary(ref x) => x.add_free_vars(free_vars),
            Subscript(ref x) => x.add_free_vars(free_vars),
            List(ref x) => x.add_free_vars(free_vars),
            Dict(ref x) => x.add_free_vars(free_vars),
            IfExp(ref x) => x.add_free_vars(free_vars),
            Closure(ref x) => x.add_free_vars(free_vars),
            Const(ref x) => x.add_free_vars(free_vars),
            Var(ref x) => x.add_free_vars(free_vars),
            Func(ref x) => {},
        }
    }
}

impl AddFreeVars for Target {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        use explicate::Target::*;
        match *self {
            Var(var) => var.add_free_vars(free_vars),
            Subscript(ref s) => s.add_free_vars(free_vars),
        }
    }
}

impl AddFreeVars for Printnl {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        self.expr.add_free_vars(free_vars)
    }
}

impl AddFreeVars for Assign {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        self.target.add_free_vars(free_vars);
        self.expr.add_free_vars(free_vars);
    }
}

impl AddFreeVars for Return {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        self.expr.add_free_vars(free_vars)
    }
}

impl AddFreeVars for Let {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        self.var.add_free_vars(free_vars);
        self.rhs.add_free_vars(free_vars);
        self.body.add_free_vars(free_vars);
    }
}

impl AddFreeVars for ProjectTo {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        self.expr.add_free_vars(free_vars)
    }
}

impl AddFreeVars for InjectFrom {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        self.expr.add_free_vars(free_vars)
    }
}

impl AddFreeVars for CallFunc {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        self.expr.add_free_vars(free_vars);
        for arg in &self.args {
            arg.add_free_vars(free_vars);
        }
    }
}

impl AddFreeVars for CallRuntime {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        for arg in &self.args {
            arg.add_free_vars(free_vars);
        }
    }
}

impl AddFreeVars for Binary {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        self.left.add_free_vars(free_vars);
        self.right.add_free_vars(free_vars);
    }
}

impl AddFreeVars for Unary {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        self.expr.add_free_vars(free_vars);
    }
}

impl AddFreeVars for Subscript {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        self.base.add_free_vars(free_vars);
        self.elem.add_free_vars(free_vars);
    }
}

impl AddFreeVars for List {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        for expr in &self.exprs {
            expr.add_free_vars(free_vars);
        }
    }
}

impl AddFreeVars for Dict {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        for tuple in &self.tuples {
            tuple.0.add_free_vars(free_vars);
            tuple.1.add_free_vars(free_vars);
        }
    }
}

impl AddFreeVars for IfExp {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        self.cond.add_free_vars(free_vars);
        self.then.add_free_vars(free_vars);
        self.else_.add_free_vars(free_vars);
    }
}

impl AddFreeVars for Closure {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        free_vars.extend(self.free_vars());
        for stmt in &self.code {
            stmt.add_free_vars(free_vars);
        }
    }
}

impl AddFreeVars for Var {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        // empty
    }
}

impl AddFreeVars for Const {
    fn add_free_vars(&self, free_vars: &mut HashSet<Var>) {
        // empty
    }
}

pub trait FreeVars {
    fn free_vars(&self) -> HashSet<Var>;
}

impl FreeVars for Module {
    fn free_vars(&self) -> HashSet<Var> {
        self.stmts.free_vars()
    }
}

impl FreeVars for [Stmt] {
    fn free_vars(&self) -> HashSet<Var> {
        let defs = self.defs();
        let uses = self.uses();
        uses.difference(&defs).map(|&v| v).collect()
    }
}

impl FreeVars for Closure {
    fn free_vars(&self) -> HashSet<Var> {
        let body_free_vars = self.code.free_vars();
        let params = set_union!(self.args.iter());
        body_free_vars.difference(&params).map(|&v| v).collect()
    }
}

pub trait Defs {
    fn defs(&self) -> HashSet<Var>;
}

impl Defs for [Stmt] {
    fn defs(&self) -> HashSet<Var> {
        let mut defs = HashSet::new();
        for s in self {
            if let Stmt::Assign(ref assign) = *s {
                if let Target::Var(var) = assign.target {
                    defs.insert(var);
                }
            }
        }
        defs
    }
}

impl Defs for Closure {
    fn defs(&self) -> HashSet<Var> {
        let mut defs = HashSet::new();
        // add all parameter definitions
        for &arg in &self.args {
            defs.insert(arg);
        }
        // add body definitions
        defs.extend(self.code.defs());

        defs
    }
}

pub trait Uses {
    fn uses(&self) -> HashSet<Var>;
}

impl Uses for [Stmt] {
    fn uses(&self) -> HashSet<Var> {
        let mut uses = HashSet::new();
        for stmt in self {
            uses.extend(stmt.uses());
        }
        uses
    }
}

impl Uses for Stmt {
    fn uses(&self) -> HashSet<Var> {
        use self::Stmt::*;
        match *self {
            Printnl(ref x) => x.uses(),
            Assign(ref x) => x.uses(),
            Expr(ref x) => x.uses(),
            Return(ref x) => x.uses(),
        }
    }
}

impl Uses for Expr {
    fn uses(&self) -> HashSet<Var> {
        use self::Expr::*;
        match *self {
            Let(ref x) => x.uses(),
            ProjectTo(ref x) => x.uses(),
            InjectFrom(ref x) => x.uses(),
            CallFunc(ref x) => x.uses(),
            CallRuntime(ref x) => x.uses(),
            Binary(ref x) => x.uses(),
            Unary(ref x) => x.uses(),
            Subscript(ref x) => x.uses(),
            List(ref x) => x.uses(),
            Dict(ref x) => x.uses(),
            IfExp(ref x) => x.uses(),
            // Filter Closures out of expressions
            Closure(ref x) => HashSet::new(),
            Const(ref x) => x.uses(),
            Var(ref x) => x.uses(),
            Func(ref f) => HashSet::new(),
        }
    }
}

impl Uses for Target {
    fn uses(&self) -> HashSet<Var> {
        use self::Target::*;
        match *self {
            Var(ref x) => HashSet::new(),
            Subscript(ref x) => {
                let mut uses = HashSet::new();
                uses.extend(x.base.uses());
                uses.extend(x.elem.uses());
                uses
            }
        }
    }
}

impl Uses for Printnl {
    fn uses(&self) -> HashSet<Var> {
        self.expr.uses()
    }
}

impl Uses for Assign {
    fn uses(&self) -> HashSet<Var> {
        let mut uses = HashSet::new();
        uses.extend(self.target.uses());
        uses.extend(self.expr.uses());
        uses
    }
}

impl Uses for Return {
    fn uses(&self) -> HashSet<Var> {
        self.expr.uses()
    }
}

impl Uses for Let {
    fn uses(&self) -> HashSet<Var> {
        set_union!(
            self.rhs.uses(),
            self.body.uses()
        )
    }
}

impl Uses for ProjectTo {
    fn uses(&self) -> HashSet<Var> {
        self.expr.uses()
    }
}

impl Uses for InjectFrom {
    fn uses(&self) -> HashSet<Var> {
        self.expr.uses()
    }
}

impl Uses for CallFunc {
    fn uses(&self) -> HashSet<Var> {
        let mut uses = self.expr.uses();
        for arg in &self.args {
            uses.extend(arg.uses());
        }
        uses
    }
}

impl Uses for CallRuntime {
    fn uses(&self) -> HashSet<Var> {
        let mut uses = HashSet::new();
        for arg in &self.args {
            uses.extend(arg.uses());
        }
        uses
    }
}

impl Uses for Binary {
    fn uses(&self) -> HashSet<Var> {
        set_union!(
            self.left.uses(),
            self.right.uses()
        )
    }
}

impl Uses for Unary {
    fn uses(&self) -> HashSet<Var> {
        self.expr.uses()
    }
}

impl Uses for Subscript {
    fn uses(&self) -> HashSet<Var> {
        set_union!(
            self.base.uses(),
            self.elem.uses()
        )
    }
}

impl Uses for List {
    fn uses(&self) -> HashSet<Var> {
        let mut uses = HashSet::new();
        for expr in &self.exprs {
            uses.extend(expr.uses());
        }
        uses
    }
}

impl Uses for Dict {
    fn uses(&self) -> HashSet<Var> {
        let mut uses = HashSet::new();
        for tuple in &self.tuples {
            uses.extend(tuple.0.uses());
            uses.extend(tuple.1.uses());
        }
        uses
    }
}

impl Uses for IfExp {
    fn uses(&self) -> HashSet<Var> {
        let mut uses = HashSet::new();
        uses.extend(self.cond.uses());
        uses.extend(self.then.uses());
        uses.extend(self.else_.uses());
        uses
    }
}

impl Uses for Closure {
    fn uses(&self) -> HashSet<Var> {
        let mut uses = HashSet::new();
        for stmt in &self.code {
            uses.extend(stmt.uses());
        }
        uses
    }
}

impl Uses for Const {
    fn uses(&self) -> HashSet<Var> {
        // no new uses in a constant
        HashSet::new()
    }
}

impl Uses for Var {
    fn uses(&self) -> HashSet<Var> {
        set!(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Defs;
    use ast;

    #[test]
    fn defs() {
        let mut explicate = Explicate::new();
        let x = explicate.name(ast::Name("x".into()));
        let y = explicate.name(ast::Name("y".into()));
        let stmts: Vec<Stmt> = vec![
            Assign {
                target: x.into(),
                expr: y.into(),
            }.into(),
            Printnl {
                expr: x.into(),
            }.into(),
        ];
        let defs = stmts.defs();
        for def in defs {
            println!("def for {}", explicate.formatter(&def));
        }
    }
}
