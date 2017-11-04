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
    fn heapify(mut self, builder: &mut Builder) -> Module {
        let heap_init: Vec<Stmt> = builder.needs_heapify.iter()
            .map(|&var| {
                Assign {
                    target: var.into(),
                    expr: List {
                        exprs: vec![
                            inject_from(Const::Int(0), Ty::Int).into(),
                        ],
                    }.into(),
                }.into()
            })
            .collect();

        let mut stmts = ::std::mem::replace(&mut self.stmts, heap_init);
        let stmts = stmts.heapify(builder);

        self.stmts.extend(stmts);

        self
    }
}

impl Heapify for Vec<Stmt> {
    type Output = Vec<Stmt>;
    fn heapify(self, builder: &mut Builder) -> Vec<Stmt> {
        self.into_iter().map(|stmt| {
            stmt.heapify(builder)
        }).collect()
    }
}

impl Heapify for Stmt {
    type Output = Stmt;
    fn heapify(mut self, builder: &mut Builder) -> Stmt {
        use self::Stmt::*;
        match *self {
            Printnl(ref mut x) => x.heapify(builder).into(),
            Assign(ref mut x) => x.heapify(builder).into(),
            Expr(ref mut x) => x.heapify(builder).into(),
            Return(ref mut x) => x.heapify(builder).into(),
        }
    }
}

impl Heapify for Expr {
    type Output = Expr;
    fn heapify(mut self, builder: &mut Builder) -> Expr {
        use self::Expr::*;
        match *self {
            Let(x) => x.heapify(builder).into(),
            ProjectTo(x) => x.heapify(builder).into(),
            InjectFrom(x) => x.heapify(builder).into(),
            CallFunc(x) => x.heapify(builder).into(),
            CallRuntime(x) => x.heapify(builder).into(),
            Binary(x) => x.heapify(builder).into(),
            Unary(x) => x.heapify(builder).into(),
            Subscript(x) => x.heapify(builder).into(),
            List(x) => x.heapify(builder).into(),
            Dict(x) => x.heapify(builder).into(),
            IfExp(x) => x.heapify(builder).into(),
            Closure(x) => x.heapify(builder).into(),
            Var(x) => x.heapify(builder).into(),
            Const(x) => x.heapify(builder).into(),
        }
    }
}

impl Heapify for Target {
    type Output = Target;
    fn heapify(&mut self, builder: &mut Builder) -> Target {
        unimplemented!()
    }
}

impl Heapify for Printnl {
    type Output = Stmt;
    fn heapify(&mut self, builder: &mut Builder) -> Stmt {
        unimplemented!()
    }
}

impl Heapify for Assign {
    type Output = Stmt;
    fn heapify(&mut self, builder: &mut Builder) -> Stmt {
        unimplemented!()
    }
}

impl Heapify for Return {
    fn heapify(&mut self, builder: &mut Builder) {
        self.expr.heapify(builder);
    }
}

impl Heapify for Let {
    fn heapify(&mut self, builder: &mut Builder) {
        // DON'T NEED TO HEAPIFY Tmp variable because it's
        // always a tmp!!!
        //self.var.heapify(builder);
        self.rhs.heapify(builder);
        self.body.heapify(builder);
    }
}
