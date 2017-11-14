use std::collections::HashMap;
use error::*;
use explicate::*;
use raise::TransformAst;
use raise::VisitAst;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Builder<'var_data> {
    var_data: &'var_data mut var::Slab<var::Data>,
    all_free_vars: HashSet<Var>,
    heapified: HashSet<Var>,
}

impl<'var_data> Builder<'var_data> {
    pub fn new(var_data: &'var_data mut var::Slab<var::Data>) -> Builder<'var_data> {
        Builder {
            var_data,
            all_free_vars: HashSet::new(),
            heapified: HashSet::new(),
        }
    }

    pub fn heapify_module(&mut self, module: Module) -> Module {
        self.all_free_vars = all_free_vars(&module);
        debug!("all free vars: {:?}", self.all_free_vars);
        let heapified = self.heapify_stmts(module.stmts);

        Module {
            stmts: heapified
        }
    }

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

        // need to rename parameters to be heapified,
        // and move heapified parameters into body
        let heapified_params: Vec<Var> = closure.args
            .iter()
            .map(|&var| var)
            .filter(|var| self.all_free_vars.contains(var))
            .collect();
        let heapified_params_inits: Vec<Stmt> = heapified_params
            .iter()
            .map(|&var| self.assign_list_0(var).into())
            .collect();

        let mut heapified_params_assigns: Vec<Stmt> = vec![];
        let renamed_args: Vec<Var> = closure.args
            .iter()
            .map(|&var| {
                if heapified_params.contains(&var) {
                    let renamed = self.new_temp();
                    heapified_params_assigns.push(self.assign_to(var, renamed).into());
                    renamed
                } else {
                    var
                }
            })
            .collect();

        let needs_heapifying_outside: HashSet<Var> = all_free_vars(&closure)
            .into_iter()
            .filter(|var| {
                !self.heapified.contains(var)
            })
            .collect();
        trace!("needs heapifying outside: {:?}", needs_heapifying_outside);
        for &var in &needs_heapifying_outside {
            self.heapified.insert(var);
        }

        let code = {
            let mut code = vec![];
            code.extend(heapified_params_inits);
            code.extend(heapified_params_assigns);
            code.extend(self.heapify_stmts(closure.code));
            code
        };

        let mut ret: Expr = Closure {
            args: renamed_args,
            code: code,
        }.into();

        for &var in &needs_heapifying_outside {
            ret = let_(var, list_0(), ret).into();
        }

        ret
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
