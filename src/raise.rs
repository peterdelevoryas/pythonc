use explicate::*;
use error::*;

pub mod func {
    use explicate::Var;
    use explicate::Closure;
    use super::Block;
    use std::collections::HashSet;

    impl_ref!(Func, "f");

    #[derive(Debug, Clone)]
    pub struct Data {
        pub free_vars: Vec<Var>,
        pub closure: Closure,
    }
}
pub use self::func::Func;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

pub struct TransUnit {
    pub main: Func,
    pub funcs: func::Slab<func::Data>,
}

pub struct Builder<'var_data> {
    // curr is a stack of blocks that are being created.
    // Each time a nested block is entered, the nested
    // block is pushed on top, and thus the top of the stack
    // is the actual current block. Each time a block
    // is exited, the current block is popped off the
    // stack and added to the slab of funcs.
    curr: Vec<Block>,
    funcs: func::Slab<func::Data>,
    var_data: &'var_data mut var::Slab<var::Data>,
}

impl<'var_data> Builder<'var_data> {
    pub fn build(heapified: Module, var_data: &'var_data mut var::Slab<var::Data>) -> TransUnit {
        let mut builder = Self {
            curr: vec![],
            funcs: func::Slab::new(),
            var_data: var_data,
        };
        builder.new_func();
        builder.add_to_curr_func(heapified.stmts);
        // no params for main function
        let main = builder.end_func(vec![], vec![]);
        TransUnit {
            main,
            funcs: builder.funcs,
        }
    }

    pub fn new_func(&mut self) {
        self.curr.push(Block { stmts: vec![] });
    }

    pub fn add_to_curr_func(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            let stmt = self.stmt(stmt);
            // would like to move this out of loop,
            // because logically curr should be at the same spot
            // because any new blocks should be ended before
            // returning here, but borrow checker can't know that.
            let curr = self.curr.last_mut().unwrap();
            curr.stmts.push(stmt);
        }
    }

    // moves curr.last to funcs
    pub fn end_func(&mut self, free_vars: Vec<Var>, mut params: Vec<Var>) -> Func {
        let fvs_list = self.new_temp();
        params.insert(0, fvs_list);
        let code = {
            let curr = self.curr.pop().expect("end_block with empty curr");
            let mut stmts = vec![];
            for (i, &fv) in free_vars.iter().enumerate() {
                stmts.push(
                    Assign {
                        target: fv.into(),
                        expr: Subscript {
                            base: fvs_list.into(),
                            elem: Const::Int(i as i32).into(),
                        }.into(),
                    }.into(),
                );
            }
            stmts.extend(curr.stmts);
            stmts
        };
        let data = func::Data {
            free_vars: free_vars,
            closure: Closure {
                args: params,
                code: code,
            },
        };
        self.funcs.insert(data)
    }

    fn new_temp(&mut self) -> Var {
        self.var_data.insert(var::Data::Temp)
    }
}

impl<'var_data> TransformAst for Builder<'var_data> {
    fn closure(&mut self, closure: Closure) -> Expr {
        let fvs: Vec<Var> = ::heapify::all_free_vars(&closure).into_iter().collect();
        self.new_func();
        self.add_to_curr_func(closure.code);
        let func = self.end_func(fvs.clone(), closure.args);
        trace!("all free vars for {}: {:?}", func, fvs);
        let list = List { exprs: fvs.into_iter().map(|v| v.into()).collect() };
        CallRuntime {
            name: "create_closure".into(),
            args: vec![func.into(), list.into()],
        }.into()
    }

    fn call_func(&mut self, call: CallFunc) -> Expr {
        // need to first evaluate target expr into a var
        let f = self.new_temp();
        let get_fun_ptr = CallRuntime {
            name: "get_fun_ptr".into(),
            args: vec![f.into()],
        };
        let get_free_vars = CallRuntime {
            name: "get_free_vars".into(),
            args: vec![f.into()],
        };
        let_(f, self.expr(call.expr), {
            let mut args = vec![];
            args.push(get_free_vars.into());
            for arg in call.args {
                let heapified = self.expr(arg);
                args.push(heapified);
            }
            CallFunc {
                expr: get_fun_ptr.into(),
                args: args,
            }
        }).into()
    }
}

pub trait TransformAst {
    fn stmt(&mut self, s: Stmt) -> Stmt {
        match s {
            Stmt::Printnl(p) => self.printnl(p),
            Stmt::Assign(a) => self.assign(a),
            Stmt::Expr(e) => self.expr(e).into(),
            Stmt::Return(r) => self.return_(r),
        }
    }

    fn printnl(&mut self, printnl: Printnl) -> Stmt {
        Printnl { expr: self.expr(printnl.expr) }.into()
    }

    fn assign(&mut self, assign: Assign) -> Stmt {
        Assign {
            target: self.target(assign.target),
            expr: self.expr(assign.expr),
        }.into()
    }

    fn return_(&mut self, return_: Return) -> Stmt {
        Return { expr: self.expr(return_.expr) }.into()
    }

    fn target(&mut self, target: Target) -> Target {
        match target {
            Target::Var(var) => self.target_var(var),
            Target::Subscript(subscript) => self.target_subscript(subscript),
        }
    }

    fn target_var(&mut self, var: Var) -> Target {
        var.into()
    }

    fn target_subscript(&mut self, subscript: Subscript) -> Target {
        self.subscript(subscript).into()
    }

    fn var(&mut self, var: Var) -> Expr {
        var.into()
    }

    fn subscript(&mut self, subscript: Subscript) -> Subscript {
        Subscript {
            base: self.expr(subscript.base),
            elem: self.expr(subscript.elem),
        }
    }

    fn expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Let(box l) => self.let_(l),
            Expr::GetTag(box g) => self.get_tag(g),
            Expr::ProjectTo(box p) => self.project_to(p),
            Expr::InjectFrom(box x) => self.inject_from(x),
            Expr::CallFunc(box x) => self.call_func(x),
            Expr::CallRuntime(box x) => self.call_runtime(x),
            Expr::Binary(box x) => self.binary(x),
            Expr::Unary(box x) => self.unary(x),
            Expr::Subscript(box s) => self.subscript(s).into(),
            Expr::List(box l) => self.list(l),
            Expr::Dict(box d) => self.dict(d),
            Expr::IfExp(box e) => self.if_exp(e),
            Expr::Closure(box c) => self.closure(c),
            Expr::Const(c) => self.const_(c),
            Expr::Var(v) => self.var(v),
            Expr::Func(f) => self.func(f),
        }
    }

    fn let_(&mut self, let_: Let) -> Expr {
        Let {
            var: self.let_var(let_.var),
            rhs: self.expr(let_.rhs),
            body: self.expr(let_.body),
        }.into()
    }

    fn let_var(&mut self, var: Var) -> Var {
        var
    }

    fn get_tag(&mut self, get_tag: GetTag) -> Expr {
        GetTag { expr: self.expr(get_tag.expr) }.into()
    }

    fn project_to(&mut self, project_to: ProjectTo) -> Expr {
        ProjectTo {
            to: project_to.to,
            expr: self.expr(project_to.expr),
        }.into()
    }

    fn inject_from(&mut self, inject_from: InjectFrom) -> Expr {
        InjectFrom {
            from: inject_from.from,
            expr: self.expr(inject_from.expr),
        }.into()
    }

    fn call_func(&mut self, call: CallFunc) -> Expr {
        CallFunc {
            expr: self.expr(call.expr),
            args: call.args.into_iter().map(|expr| self.expr(expr)).collect(),
        }.into()
    }

    fn call_runtime(&mut self, call: CallRuntime) -> Expr {
        CallRuntime {
            name: call.name,
            args: call.args.into_iter().map(|expr| self.expr(expr)).collect(),
        }.into()
    }

    fn binary(&mut self, binary: Binary) -> Expr {
        Binary {
            op: binary.op,
            left: self.expr(binary.left),
            right: self.expr(binary.right),
        }.into()
    }

    fn unary(&mut self, unary: Unary) -> Expr {
        Unary {
            op: unary.op,
            expr: self.expr(unary.expr),
        }.into()
    }

    fn list(&mut self, list: List) -> Expr {
        List { exprs: list.exprs.into_iter().map(|expr| self.expr(expr)).collect() }.into()
    }

    fn dict(&mut self, dict: Dict) -> Expr {
        Dict {
            tuples: dict.tuples
                .into_iter()
                .map(|(l, r)| (self.expr(l), self.expr(r)))
                .collect(),
        }.into()
    }

    fn if_exp(&mut self, if_exp: IfExp) -> Expr {
        IfExp {
            cond: self.expr(if_exp.cond),
            then: self.expr(if_exp.then),
            else_: self.expr(if_exp.else_),
        }.into()
    }

    fn closure(&mut self, closure: Closure) -> Expr {
        Closure {
            args: closure
                .args
                .into_iter()
                .map(|var| self.closure_var(var))
                .collect(),
            code: closure
                .code
                .into_iter()
                .map(|stmt| self.stmt(stmt))
                .collect(),
        }.into()
    }

    fn const_(&mut self, const_: Const) -> Expr {
        const_.into()
    }

    fn func(&mut self, func: Func) -> Expr {
        func.into()
    }

    fn closure_var(&mut self, var: Var) -> Var {
        var
    }
}

pub trait VisitAst {
    fn stmts(&mut self, stmts: &[Stmt]) {
        for s in stmts {
            self.stmt(s);
        }
    }

    fn stmt(&mut self, s: &Stmt) {
        match *s {
            Stmt::Printnl(ref p) => self.printnl(p),
            Stmt::Assign(ref a) => self.assign(a),
            Stmt::Expr(ref e) => self.expr(e).into(),
            Stmt::Return(ref r) => self.return_(r),
        }
    }

    fn printnl(&mut self, printnl: &Printnl) {
        self.expr(&printnl.expr)
    }

    fn assign(&mut self, assign: &Assign) {
        self.target(&assign.target);
        self.expr(&assign.expr);
    }

    fn return_(&mut self, return_: &Return) {
        self.expr(&return_.expr);
    }

    fn target(&mut self, target: &Target) {
        match *target {
            Target::Var(ref var) => self.target_var(var),
            Target::Subscript(ref subscript) => self.target_subscript(subscript),
        }
    }

    fn target_var(&mut self, var: &Var) {
        // nothing to do by default
    }

    fn target_subscript(&mut self, subscript: &Subscript) {
        self.subscript(subscript);
    }

    fn var(&mut self, var: &Var) {
        // nothing to do by default
    }

    fn subscript(&mut self, subscript: &Subscript) {
        self.expr(&subscript.base);
        self.expr(&subscript.elem);
    }

    fn expr(&mut self, e: &Expr) {
        match *e {
            Expr::Let(box ref l) => self.let_(l),
            Expr::GetTag(box ref g) => self.get_tag(g),
            Expr::ProjectTo(box ref p) => self.project_to(p),
            Expr::InjectFrom(box ref x) => self.inject_from(x),
            Expr::CallFunc(box ref x) => self.call_func(x),
            Expr::CallRuntime(box ref x) => self.call_runtime(x),
            Expr::Binary(box ref x) => self.binary(x),
            Expr::Unary(box ref x) => self.unary(x),
            Expr::Subscript(box ref s) => self.subscript(s).into(),
            Expr::List(box ref l) => self.list(l),
            Expr::Dict(box ref d) => self.dict(d),
            Expr::IfExp(box ref e) => self.if_exp(e),
            Expr::Closure(box ref c) => self.closure(c),
            Expr::Const(ref c) => self.const_(c),
            Expr::Var(ref v) => self.var(v),
            Expr::Func(ref f) => self.func(f),
        }
    }

    fn let_(&mut self, let_: &Let) {
        self.let_var(&let_.var);
        self.expr(&let_.rhs);
        self.expr(&let_.body);
    }

    fn let_var(&mut self, var: &Var) {
        // nothing to do by default
    }

    fn get_tag(&mut self, get_tag: &GetTag) {
        self.expr(&get_tag.expr);
    }

    fn project_to(&mut self, project_to: &ProjectTo) {
        self.expr(&project_to.expr);
    }

    fn inject_from(&mut self, inject_from: &InjectFrom) {
        self.expr(&inject_from.expr);
    }

    fn call_func(&mut self, call: &CallFunc) {
        self.expr(&call.expr);
        for arg in &call.args {
            self.expr(arg);
        }
    }

    fn call_runtime(&mut self, call: &CallRuntime) {
        for arg in &call.args {
            self.expr(arg);
        }
    }

    fn binary(&mut self, binary: &Binary) {
        self.expr(&binary.left);
        self.expr(&binary.right);
    }

    fn unary(&mut self, unary: &Unary) {
        self.expr(&unary.expr);
    }

    fn list(&mut self, list: &List) {
        for expr in &list.exprs {
            self.expr(expr);
        }
    }

    fn dict(&mut self, dict: &Dict) {
        for &(ref l, ref r) in &dict.tuples {
            self.expr(l);
            self.expr(r);
        }
    }

    fn if_exp(&mut self, if_exp: &IfExp) {
        self.expr(&if_exp.cond);
        self.expr(&if_exp.then);
        self.expr(&if_exp.else_);
    }

    fn closure(&mut self, closure: &Closure) {
        for var in &closure.args {
            self.closure_var(var);
        }
        for stmt in &closure.code {
            self.stmt(stmt);
        }
    }

    fn const_(&mut self, const_: &Const) {
        // do nothing by default
    }

    fn func(&mut self, func: &Func) {
        // do nothing by default
    }

    fn closure_var(&mut self, var: &Var) {
        // do nothing by default
    }
}
