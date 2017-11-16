use explicate as ex;
use raise;

use std::collections::HashMap;
use std::collections::HashSet;
use std::convert;

use explicate::Var;

use fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BinOp {
    ADD,
    EQ,
    NOTEQ,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    NEGATE,
    NOT,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    UnaryOp(UnaryOp, Var),
    BinOp(BinOp, Var, Var),
    CallFunc(Var, Vec<Var>),
    RuntimeFunc(String, Vec<Var>),
    GetTag(Var),
    ProjectTo(Var, ex::Ty),
    InjectFrom(Var, ex::Ty),
    Const(i32),
    LoadFunctionPointer(raise::Func), // Is this necessary? -- who cares, should produce the fnptr for the given unit
    Alias(Var),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    Def(Var, Expr),
    Discard(Expr),
    Return(Option<Var>),
    If(Var, Vec<Stmt>, Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
    pub args: Vec<Var>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Flattener {
    pub var_data: ex::var::Slab<ex::var::Data>,
    pub units: HashMap<raise::Func, Function>,
    contexts: Vec<Vec<Stmt>>
}

#[derive(Debug)]
pub struct Formatter<'a, N: 'a + ?Sized> {
    flattener : &'a Flattener,
    node : &'a N,
    indent : usize,
}

impl Flattener {
    pub fn enter_context(&mut self) {
        self.contexts.push(vec![]);
    }
    pub fn push(&mut self, ir: Stmt) {
        if let Some(top) = self.contexts.last_mut() {
            top.push(ir);
        } else {
            panic!("Tried to push with no context.");
        }
    }
    pub fn commit_fn(&mut self, func: raise::Func, args: Vec<Var>) {
        if let Some(top) = self.contexts.pop() {
            let f = Function {
                args: args,
                body: top,
            };
            self.units.insert(func, f);
        } else {
            panic!("Tried to commit with no context.");
        }
    }
    pub fn clear(&mut self) -> Vec<Stmt> {
        self.contexts.pop().expect("Tried to clear with no context.")
    }
    pub fn mk_tmp_var(&mut self) -> Var {
        self.var_data.insert(ex::var::Data::Temp)
    }
    pub fn def(&mut self, e: Expr) -> Var {
        let tmp = self.mk_tmp_var();
        self.push(Stmt::Def(tmp, e));
        tmp
    }
}

impl<'a, N: 'a + ?Sized> Formatter<'a, N> {
    pub fn new(flattener: &'a Flattener, node: &'a N) -> Formatter<'a, N> {
        Formatter {
            flattener,
            node,
            indent: 0,
        }
    }
    pub fn fmt<M: 'a + ?Sized>(&self, node: &'a M) -> Formatter<'a, M> {
        Formatter {
            flattener: self.flattener,
            node,
            indent: self.indent
        }
    }
    pub fn indented<M: 'a + ?Sized + fmt::Debug>(&self, node: &'a M) -> Formatter<'a, M> {
        Formatter {
            flattener: self.flattener,
            node,
            indent: self.indent + 1,
        }
    }
    pub fn width(&self) -> usize {
        self.indent * 4
    }
    pub fn indent(&self) -> String {
        " ".repeat(self.width())
    }
}

impl convert::From<ex::Explicate> for Flattener {
    fn from(explicate: ex::Explicate) -> Flattener {
        Flattener {
            var_data: explicate.var_data,
            units: HashMap::new(),
            contexts: vec![],
        }
    }
}

pub trait Flatten {
    type Output;
    fn flatten(self, builder: &mut Flattener) -> Self::Output;
}

impl Flatten for raise::TransUnit {
    type Output = ();
    fn flatten(self, builder: &mut Flattener) {
        for (func, data) in self.funcs.into_iter() {
            builder.enter_context();
            data.clone().closure.code.flatten(builder);
            builder.commit_fn(func, data.clone().closure.args);
        }
    }
}

impl Flatten for Vec<ex::Stmt> {
    type Output = ();
    fn flatten(self, builder: &mut Flattener) {
        for stmt in self.into_iter() {
            stmt.flatten(builder);
        }
    }
}

impl Flatten for ex::Stmt {
    type Output = ();
    fn flatten(self, builder: &mut Flattener) {
        match self {
            ex::Stmt::Assign(a) => a.flatten(builder),
            ex::Stmt::Printnl(p) => p.flatten(builder),
            ex::Stmt::Expr(e) => { e.flatten(builder); }, // Discard case, need to throw away tmp, or maybe not -- who cares
            ex::Stmt::Return(r) => r.flatten(builder),
        }
    }
}

impl Flatten for ex::Assign {
    type Output = ();
    fn flatten(self, builder: &mut Flattener) {
        let val = self.expr.flatten(builder);

        match self.target {
            ex::Target::Var(var) => {
                builder.push(Stmt::Def(var, Expr::Alias(val)));
            }
            ex::Target::Subscript(subs) => {
                let base = subs.base.flatten(builder);
                let key = subs.elem.flatten(builder);

                // If the assign target is a subscript, we don't need the return,
                //  so discard it. -- who cares
                builder.push(Stmt::Discard(Expr::RuntimeFunc(
                    "set_subscript".into(),
                    vec![base, key, val],
                )));
            }
        };
    }
}

impl Flatten for ex::Printnl {
    type Output = ();
    fn flatten(self, builder: &mut Flattener) {
        let loc = self.expr.flatten(builder);
        builder.push(Stmt::Discard(Expr::RuntimeFunc(
            "print_any".into(),
            vec![loc]
        )));
    }
}

impl Flatten for ex::Return {
    type Output = ();
    fn flatten(self, builder: &mut Flattener) {
        let loc = self.expr.flatten(builder);
        builder.push(Stmt::Return(Some(loc)));
    }
}

impl Flatten for ex::Expr {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        match self {
            ex::Expr::Let(v) =>v.flatten(builder),
            ex::Expr::GetTag(v) => v.flatten(builder),
            ex::Expr::ProjectTo(v) => v.flatten(builder),
            ex::Expr::InjectFrom(v) => v.flatten(builder),
            ex::Expr::CallFunc(v) => v.flatten(builder),
            ex::Expr::CallRuntime(v) => v.flatten(builder),
            ex::Expr::Binary(v) => v.flatten(builder),
            ex::Expr::Unary(v) => v.flatten(builder),
            ex::Expr::Subscript(v) => v.flatten(builder),
            ex::Expr::List(v) => v.flatten(builder),
            ex::Expr::Dict(v) => v.flatten(builder),
            ex::Expr::IfExp(v) => v.flatten(builder),
            ex::Expr::Closure(v) => panic!("Encountered Closure in Flattening step."),
            ex::Expr::Const(c) => c.flatten(builder),
            ex::Expr::Var(v) => v.flatten(builder),
            ex::Expr::Func(f) => f.flatten(builder),
        }
    }
}

impl Flatten for ex::Let {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        let rhs = self.rhs.flatten(builder);
        builder.push(Stmt::Def(self.var, Expr::Alias(rhs)));
        self.body.flatten(builder)
    }
}

impl Flatten for ex::GetTag {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        let e = self.expr.flatten(builder);
        builder.def(Expr::GetTag(e))
    }
}

impl Flatten for ex::ProjectTo {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        let po = self.expr.flatten(builder);

        builder.def(Expr::ProjectTo(po, self.to))
    }
}

impl Flatten for ex::InjectFrom {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        let prim = self.expr.flatten(builder);
        builder.def(Expr::InjectFrom(prim, self.from))
    }
}

impl Flatten for ex::CallFunc {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        let base = self.expr.flatten(builder);
        let base_ptr = builder.def(Expr::RuntimeFunc(
            "get_fun_ptr".into(),
            vec![base]
        ));
        let freelist = builder.def(Expr::RuntimeFunc(
            "get_free_vars".into(),
            vec![base]
        ));

        let mut args = vec![freelist];

        for a in self.args {
          args.push(a.flatten(builder));
        }

        builder.def(Expr::CallFunc(base_ptr, args))
    }
}

impl Flatten for ex::CallRuntime {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        let args = self.args.into_iter().map(|expr| expr.flatten(builder)).collect();
        builder.def(Expr::RuntimeFunc(self.name, args))
    }
}

impl Flatten for ex::Binary {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        let op = match self.op {
            ex::Binop::Add => BinOp::ADD,
            ex::Binop::Eq => BinOp::EQ,
            ex::Binop::NotEq => BinOp::NOTEQ,
        };
        let lhs = self.left.flatten(builder);
        let rhs = self.right.flatten(builder);
        builder.def(Expr::BinOp(op, lhs, rhs))
    }
}

impl Flatten for ex::Unary {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        let op = match self.op {
            ex::Unop::Neg => UnaryOp::NEGATE,
            ex::Unop::Not => UnaryOp::NOT,
        };
        let loc = self.expr.flatten(builder);
        builder.def(Expr::UnaryOp(op, loc))
    }
}

impl Flatten for ex::Subscript {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        let base = self.base.flatten(builder);
        let elem = self.elem.flatten(builder);
        builder.def(Expr::RuntimeFunc(
            "get_subscript".into(),
            vec![base, elem],
        ))
    }
}

impl Flatten for ex::List {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        // TODO: Verify API
        let new_l_size = builder.def(Expr::Const(self.exprs.len() as i32));
        let new_l_size_injected = builder.def(Expr::InjectFrom(new_l_size, ex::Ty::Int));
        let new_l = builder.def(Expr::RuntimeFunc(
            "create_list".into(),
            vec![new_l_size_injected]
        ));

        for (i, expr) in self.exprs.iter().enumerate() {
            // Ugh, have to inject and store twice each index, but constant prop should remove this.
            // Really, who cares at this point.
            let i_loc = builder.def(Expr::Const(i as i32));
            let i_loc_injected = builder.def(Expr::InjectFrom(i_loc, ex::Ty::Int));

            let flat = expr.clone().flatten(builder);

            builder.push(Stmt::Discard(
                Expr::RuntimeFunc(
                    "set_subscript".into(),
                    vec![new_l, i_loc_injected, flat]
                )
            ));
        }
        new_l
    }
}

impl Flatten for ex::Dict {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        let new_d = builder.def(Expr::RuntimeFunc(
            "create_dict".into(),
            vec![]
        ));

        for (k, v) in self.tuples {
            let k_l = k.clone().flatten(builder);
            let v_l = v.clone().flatten(builder);

            builder.push(Stmt::Discard(
                Expr::RuntimeFunc(
                    "set_subscript".into(),
                    vec![new_d, k_l, v_l]
                )
            ));
        }

        new_d
    }
}

impl Flatten for ex::IfExp {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {

        let rloc = builder.mk_tmp_var();

        let flatc = self.cond.flatten(builder);

        builder.enter_context();
        let flatt = self.then.flatten(builder);
        let mut t_code = builder.clear();
        t_code.push(Stmt::Def(rloc, Expr::Alias(flatt)));

        builder.enter_context();
        let flate = self.else_.flatten(builder);
        let mut e_code = builder.clear();
        e_code.push(Stmt::Def(rloc, Expr::Alias(flate)));

        builder.push(Stmt::If(flatc, t_code, e_code));

        rloc
    }
}

impl Flatten for ex::Const {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        // Gross
        builder.def(
            match self {
                ex::Const::Bool(b) => Expr::Const( if b { 1 } else { 0 }),
                ex::Const::Int(i) => Expr::Const(i),
            }
        )

    }
}

impl Flatten for Var {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        self
    }
}

impl Flatten for raise::Func {
    type Output = Var;
    fn flatten(self, builder: &mut Flattener) -> Var {
        builder.def(Expr::LoadFunctionPointer(self))
    }
}

impl<'a> fmt::Display for Formatter<'a, ()> {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        for (label, func) in &self.flattener.units {
            writeln!(f, "{indent}{label}:", indent=self.indent(), label=label)?;
            writeln!(f, "{indent}{stmts}", indent=self.indent(), stmts=self.indented(func.body.as_slice()))?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Formatter<'a, [Stmt]> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in self.node {
            writeln!(f, "{indent}{stmt}", indent=self.indent(), stmt=self.fmt(stmt))?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Formatter<'a, Stmt> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.node {
            Stmt::Def(tmp, ref expr) => write!(f, "{} = {}", tmp, self.fmt(expr)),
            Stmt::Discard(ref expr) => write!(f, "{}", self.fmt(expr)),
            Stmt::Return(ref loc) => match *loc {
                Some(loc) => write!(f, "return {}", loc),
                None => write!(f, "return"),
            },
            Stmt::If(c, ref t_block, ref e_block) => {
                writeln!(f, "if {} {{", c)?;
                writeln!(f, "{indent}then: ", indent=self.indent())?;
                writeln!(f, "{block}", block=self.indented(t_block.as_slice()))?;
                writeln!(f, "{indent}else: ", indent=self.indent())?;
                writeln!(f, "{block}", block=self.indented(e_block.as_slice()))?;
                write!(f, "{indent}}}", indent=self.indent())?;
                Ok(())
            }
        }
    }
}

impl<'a> fmt::Display for Formatter<'a, Expr> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn write_args_list(f: &mut fmt::Formatter, args: &[Var]) -> fmt::Result {
            if !args.is_empty() {
                write!(f, "{}", args[0])?;
            }
            for arg in &args[1..] {
                write!(f, ", {}", arg)?;
            }
            Ok(())
        }
        match *self.node {
            Expr::UnaryOp(op, loc) => write!(f, "{} {}", op, loc),
            Expr::BinOp(op, l, r) => write!(f, "{} {} {}", l, op, r),
            Expr::CallFunc(func, ref args) => {
                write!(f, "{}(", func)?;
                write_args_list(f, args)?;
                write!(f, ")")
            }
            Expr::RuntimeFunc(ref name, ref args) => {
                write!(f, "@{}(", name)?;
                write_args_list(f, args)?;
                write!(f, ")")
            }
            Expr::GetTag(var) => {
                write!(f, "get_tag {}", var)
            }
            Expr::ProjectTo(loc, ty) => {
                write!(f, "project {} to {}", loc, ty)
            }
            Expr::InjectFrom(loc, ty) => {
                write!(f, "inject {} from {}", loc, ty)
            }
            Expr::Const(i) => {
                write!(f, "const i32 {}", i)
            }
            Expr::LoadFunctionPointer(ref name) => {
                write!(f, "const fn {}", name)
            }
            Expr::Alias(v) => {
                write!(f, "{}", v)
            }
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            UnaryOp::NEGATE => "neg",
            UnaryOp::NOT => "not",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            BinOp::ADD => "+",
            BinOp::EQ => "==",
            BinOp::NOTEQ => "!=",
        };
        write!(f, "{}", s)
    }
}
