use std::collections::HashMap;
use error::*;
use explicate::*;
use raise::TransformAst;
use raise::VisitAst;
use std::collections::HashSet;

pub fn heapify(var_data: &mut var::Slab<var::Data>, module: Module) -> Module {
    let all_free_vars = all_free_vars(&module);
    let main = Closure {
        args: vec![],
        code: module.stmts,
    };
    let mut builder = Builder {
        var_data: var_data,
        all_free_vars: all_free_vars,
        heapified: HashSet::new(),
    };
    let main = builder.heapify_closure(main);
    Module {
        stmts: main.code,
    }
}

#[derive(Debug)]
pub struct Builder<'var_data> {
    var_data: &'var_data mut var::Slab<var::Data>,
    all_free_vars: HashSet<Var>,
    heapified: HashSet<Var>,
}

impl<'var_data> Builder<'var_data> {
    fn heapify_stmts(&mut self, stmts: Vec<Stmt>) -> Vec<Stmt> {
        stmts.into_iter().map(|stmt| self.stmt(stmt)).collect()
    }

    fn new_temp(&mut self) -> Var {
        self.var_data.insert(var::Data::Temp)
    }

    fn assign_list_0(&mut self, var: Var) -> Assign {
        Assign {
            target: var.into(),
            expr: List {
                exprs: vec![Const::Int(0).into()]
            }.into()
        }
    }

    fn assign_to(&mut self, target: Var, source: Var) -> Assign {
        Assign {
            target: subscript_0(target).into(),
            expr: source.into(),
        }
    }

    /// This is the main closure heapification function,
    /// pretty much everything important happens here.
    fn heapify_closure(&mut self, closure: Closure) -> Closure {
        let locals = locals(&closure);
        trace!("locals: {:?}", locals);
        // make call on body recursively
        let heapified_body: Vec<Stmt> = closure.code.into_iter()
            .map(|stmt| self.stmt(stmt))
            .collect();

        // for all the arguments that are in the free vars
        // set, rename them, and then make a param init
        // statement that assigns the heapified (list_1)
        // renamed param
        let mut param_inits: Vec<Stmt> = vec![];
        let mut args = closure.args;
        for arg in &mut args {
            if self.all_free_vars.contains(arg) {
                let renamed_param = self.new_temp();
                let arg = ::std::mem::replace(arg, renamed_param);
                param_inits.push(assign(arg, list_1(renamed_param)).into());
            }
        }

        let mut local_inits: Vec<Stmt> = vec![];
        for local in locals {
            if self.all_free_vars.contains(&local) {
                local_inits.push(assign(local, list_1(Const::Int(-1))).into());
            }
        }

        let mut code = vec![];
        code.extend(param_inits);
        code.extend(local_inits);
        code.extend(heapified_body);

        Closure {
            args: args,
            code: code,
        }
    }
}

fn subscript_0(var: Var) -> Subscript {
    Subscript {
        base: var.into(),
        elem: Const::Int(0).into(),
    }
}

fn list_0() -> List {
    List {
        exprs: vec![Const::Int(0).into()],
    }
}

impl<'var_data> TransformAst for Builder<'var_data> {
    fn assign(&mut self, assign: Assign) -> Stmt {
        // recurse on assignment before anything.
        // the lhs might actually get heapified
        // during this step.
        let rhs = self.expr(assign.expr);

        // otherwise, do default behavior
        let target = self.target(assign.target);
        Assign {
            target,
            expr: rhs,
        }.into()
    }

    fn target_var(&mut self, var: Var) -> Target {
        if self.all_free_vars.contains(&var) {
            subscript_0(var).into()
        } else {
            var.into()
        }
    }

    fn var(&mut self, var: Var) -> Expr {
        if self.all_free_vars.contains(&var) {
            subscript_0(var).into()
        } else {
            var.into()
        }
    }

    fn let_var(&mut self, var: Var) -> Var {
        if self.all_free_vars.contains(&var) {
            panic!("lhs of let needs heapifying!!!")
        }
        var
    }

    fn closure(&mut self, closure: Closure) -> Expr {
        self.heapify_closure(closure).into()
    }
}


/// Finds free variables in local scope _and_ nested scopes
pub fn all_free_vars<T>(node: &T) -> HashSet<Var>
where
    T: AllFreeVars
{
    node.all_free_vars()
}

pub trait AllFreeVars {
    fn all_free_vars(&self) -> HashSet<Var>;
}

impl AllFreeVars for Module {
    fn all_free_vars(&self) -> HashSet<Var> {
        let mut collector = Collector::new();
        let local_free_vars = ::free_vars::free_vars(self.stmts.as_slice());
        collector.vars.extend(local_free_vars);
        collector.stmts(self.stmts.as_slice());
        collector.vars
    }
}

impl AllFreeVars for Closure {
    fn all_free_vars(&self) -> HashSet<Var> {
        let mut collector = Collector::new();
        collector.closure(self);
        collector.vars
    }
}

#[derive(Debug)]
struct Collector {
    vars: HashSet<Var>,
}

impl Collector {
    fn new() -> Self {
        Self {
            vars: HashSet::new()
        }
    }
}

impl ::raise::VisitAst for Collector {
    fn closure(&mut self, closure: &Closure) {
        let nested_free_vars = ::free_vars::free_vars(closure);
        trace!("nested free vars: {:?}", nested_free_vars);
        self.vars.extend(nested_free_vars);
        self.stmts(closure.code.as_slice());
    }
}

struct NestedFreeVars(HashSet<Var>);

impl ::raise::VisitAst for NestedFreeVars {
    fn closure(&mut self, closure: &Closure) {
        let locals = locals(closure);
        let all = all_free_vars(closure);
        for free_var in all {
            if !locals.contains(&free_var) {
                self.0.insert(free_var);
            }
        }
    }
}

pub fn nested_free_vars(closure: &Closure) -> HashSet<Var> {
    let mut nfvs = NestedFreeVars(HashSet::new());
    nfvs.closure(closure);
    nfvs.0
}

pub struct Locals {
    vars: HashSet<Var>,
}

pub fn locals(closure: &Closure) -> HashSet<Var> {
    let mut locals = Locals { vars: HashSet::new() };
    for stmt in &closure.code {
        locals.stmt(stmt);
    }
    locals.vars
}

impl ::raise::VisitAst for Locals {
    fn target_var(&mut self, &var: &Var) {
        self.vars.insert(var);
    }

    fn closure(&mut self, closure: &Closure) {
        // don't traverse into nested scope
    }
}
