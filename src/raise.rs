use explicate::*;
use error::*;

pub mod func {
    use explicate::Var;
    use super::Block;

    impl_ref!(Func, "f");

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Data {
        pub args: Vec<Var>,
        pub body: Block,
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

pub struct Builder {
    // curr is a stack of blocks that are being created.
    // Each time a nested block is entered, the nested
    // block is pushed on top, and thus the top of the stack
    // is the actual current block. Each time a block
    // is exited, the current block is popped off the
    // stack and added to the slab of funcs.
    curr: Vec<Block>,
    funcs: func::Slab<func::Data>,
}

impl Builder {
    pub fn build(heapified: Module) -> TransUnit {
        let mut builder = Self {
            curr: vec![],
            funcs: func::Slab::new(),
        };
        builder.new_func();
        builder.add_to_curr_func(heapified.stmts);
        // no params for main function
        let main = builder.end_func(vec![]);
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
    pub fn end_func(&mut self, params: Vec<Var>) -> Func {
        let curr = self.curr.pop().expect("end_block with empty curr");
        let data = func::Data {
            args: params,
            body: curr,
        };
        self.funcs.insert(data)
    }
}

impl TransformAst for Builder {
    fn closure(&mut self, closure: Closure) -> Expr {
        self.new_func();
        self.add_to_curr_func(closure.code);
        let func = self.end_func(closure.args);
        func.into()
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
        Printnl {
            expr: self.expr(printnl.expr),
        }.into()
    }

    fn assign(&mut self, assign: Assign) -> Stmt {
        Assign {
            target: self.target(assign.target),
            expr: self.expr(assign.expr),
        }.into()
    }

    fn return_(&mut self, return_: Return) -> Stmt {
        Return {
            expr: self.expr(return_.expr),
        }.into()
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
        List {
            exprs: list.exprs.into_iter().map(|expr| self.expr(expr)).collect()
        }.into()
    }

    fn dict(&mut self, dict: Dict) -> Expr {
        Dict {
            tuples: dict.tuples.into_iter().map(|(l, r)| (self.expr(l), self.expr(r))).collect()
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
            args: closure.args.into_iter().map(|var| self.closure_var(var)).collect(),
            code: closure.code.into_iter().map(|stmt| self.stmt(stmt)).collect(),
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