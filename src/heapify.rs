use std::collections::HashMap;
use std::collections::HashSet;
use error::*;
use explicate::*;

fn find_vars_to_heapify<T: AddFreeVars>(ast: &T) -> HashSet<Var> {
    let mut fv = HashSet::new();
    ast.add_free_vars(&mut fv);
    fv
}

pub fn heapify(mut ast: Module) -> Module {
    let mut b = Builder {
        needs_heapify: find_vars_to_heapify(&ast),
    };
    ast.heapify(&mut b)
}

pub struct Builder {
    needs_heapify: HashSet<Var>,
}

pub trait Heapify {
    type Output;
    fn heapify(self, builder: &mut Builder) -> Self::Output;
}

impl Heapify for Module {
    type Output = Module;
    fn heapify(mut self, builder: &mut Builder) -> Self::Output {
        Module {
            stmts: self.stmts.heapify(builder),
        }
    }
}

impl Heapify for Vec<Stmt> {
    type Output = Vec<Stmt>;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        self.into_iter().map(|stmt| {
            stmt.heapify(builder)
        }).collect()
    }
}

impl Heapify for Stmt {
    type Output = Stmt;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        use self::Stmt::*;
        match self {
            Printnl(x) => x.heapify(builder).into(),
            Assign(x) => x.heapify(builder).into(),
            Expr(x) => x.heapify(builder).into(),
            Return(x) => x.heapify(builder).into(),
        }
    }
}

impl Heapify for Expr {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        use self::Expr::*;
        match self {
            Let(box x) => x.heapify(builder).into(),
            ProjectTo(box x) => x.heapify(builder).into(),
            InjectFrom(box x) => x.heapify(builder).into(),
            CallFunc(box x) => x.heapify(builder).into(),
            CallRuntime(box x) => x.heapify(builder).into(),
            Binary(box x) => x.heapify(builder).into(),
            Unary(box x) => x.heapify(builder).into(),
            Subscript(box x) => x.heapify(builder).into(),
            List(box x) => x.heapify(builder).into(),
            Dict(box x) => x.heapify(builder).into(),
            IfExp(box x) => x.heapify(builder).into(),
            Closure(box x) => x.heapify(builder).into(),
            Var(x) => x.heapify(builder).into(),
            Const(x) => x.heapify(builder).into(),
            Func(x) => panic!("Func in AST during heapification"),
        }
    }
}

impl Heapify for Target {
    type Output = Target;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        match self {
            Target::Var(var) if builder.needs_heapify.contains(&var) => {
                Subscript {
                    base: var.into(),
                    elem: inject_from(Const::Int(0), Ty::Int).into(),
                }.into()
            }
            target => target,
        }
    }
}

impl Heapify for Printnl {
    type Output = Stmt;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        Printnl {
            expr: self.expr.heapify(builder)
        }.into()
    }
}

impl Heapify for Assign {
    type Output = Stmt;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        Assign {
            target: self.target.heapify(builder),
            expr: self.expr.heapify(builder),
        }.into()
    }
}

impl Heapify for Return {
    type Output = Stmt;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        Return {
            expr: self.expr.heapify(builder)
        }.into()
    }
}

impl Heapify for Let {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        // ignore var, does not need to ever be modified
        // since it can only be a temporary created by compiler
        Let {
            var: self.var,
            rhs: self.rhs.heapify(builder),
            body: self.body.heapify(builder),
        }.into()
    }
}

impl Heapify for ProjectTo {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        ProjectTo {
            to: self.to,
            expr: self.expr.heapify(builder)
        }.into()
    }
}

impl Heapify for InjectFrom {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        InjectFrom {
            from: self.from,
            expr: self.expr.heapify(builder)
        }.into()
    }
}

impl Heapify for CallFunc {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        CallFunc {
            expr: self.expr.heapify(builder),
            args: self.args.into_iter().map(|arg| arg.heapify(builder)).collect()
        }.into()
    }
}

impl Heapify for CallRuntime {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        CallRuntime {
            name: self.name,
            args: self.args.into_iter().map(|arg| arg.heapify(builder)).collect()
        }.into()
    }
}

impl Heapify for Binary {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        Binary {
            op: self.op,
            left: self.left.heapify(builder),
            right: self.right.heapify(builder),
        }.into()
    }
}

impl Heapify for Unary {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        Unary {
            op: self.op,
            expr: self.expr.heapify(builder)
        }.into()
    }
}

impl Heapify for Subscript {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        Subscript {
            base: self.base.heapify(builder),
            elem: self.elem.heapify(builder),
        }.into()
    }
}

impl Heapify for List {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        List {
            exprs: self.exprs.into_iter().map(|e| e.heapify(builder)).collect()
        }.into()
    }
}

impl Heapify for Dict {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        Dict {
            tuples: self.tuples.into_iter().map(|(l, r)| {
                (l.heapify(builder), r.heapify(builder))
            }).collect()
        }.into()
    }
}

impl Heapify for IfExp {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        IfExp {
            cond: self.cond.heapify(builder),
            then: self.then.heapify(builder),
            else_: self.else_.heapify(builder),
        }.into()
    }
}

impl Heapify for Closure {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        let free_vars = self.free_vars();
        // arguments shouldn't need heapification here
        let code = self.code.heapify(builder);
        let mut ret: Expr = Closure {
            args: self.args,
            code: code,
        }.into();
        for free_var in free_vars {
            ret = let_(free_var,
                       List {
                           exprs: vec![
                               inject_from(Const::Int(0), Ty::Int).into()
                           ],
                       },
                       ret).into();
        }
        ret
    }
}

impl Heapify for Var {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        if builder.needs_heapify.contains(&self) {
            Subscript {
                base: self.into(),
                elem: inject_from(Const::Int(0), Ty::Int).into(),
            }.into()
        } else {
            self.into()
        }
    }
}

impl Heapify for Const {
    type Output = Expr;
    fn heapify(self, builder: &mut Builder) -> Self::Output {
        self.into()
    }
}
