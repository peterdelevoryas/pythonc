use std::collections::HashMap;
use error::*;
use explicate::*;
use raise::TransformAst;
use raise::VisitAst;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Builder<'var_data> {
    var_data: &'var_data mut var::Slab<var::Data>,
    needs_heapifying: NeedsHeapifying,
}

impl<'var_data> Builder<'var_data> {
    pub fn new(var_data: &'var_data mut var::Slab<var::Data>) -> Builder<'var_data> {
        Builder {
            var_data,
            needs_heapifying: NeedsHeapifying::new(),
        }
    }

    pub fn heapify_module(&mut self, module: Module) -> Module {
        self.needs_heapifying.stmts(module.stmts.as_slice());
        debug!("needs heapifying: {:?}", self.needs_heapifying.vars);
        let heapified = self.heapify_stmts(module.stmts);
        let free_vars = ::free_vars::free_vars(heapified.as_slice());
        Module { stmts: heapified }
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

impl<'var_data> TransformAst for Builder<'var_data> {
    fn target_var(&mut self, var: Var) -> Target {
        if self.needs_heapifying.get(var) {
            subscript_0(var).into()
        } else {
            var.into()
        }
    }

    fn var(&mut self, var: Var) -> Expr {
        if self.needs_heapifying.get(var) {
            subscript_0(var).into()
        } else {
            var.into()
        }
    }

    fn let_var(&mut self, var: Var) -> Var {
        if self.needs_heapifying.get(var) {
            panic!("lhs of let needs heapifying!!!")
        }
        var
    }

    fn closure(&mut self, closure: Closure) -> Expr {
        let closure = Closure {
            args: closure.args,
            code: self.heapify_stmts(closure.code),
        };
        // need to rename parameters to be heapified,
        // and move heapified parameters into body
        let heapified_params: Vec<Var> = closure.args
            .iter()
            .map(|&var| var)
            .filter(|&var| self.needs_heapifying.get(var))
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

        let heapified_locals: Vec<Stmt> = {
            let local_free_vars = ::free_vars::free_vars(closure.code.as_slice());
            local_free_vars
                .iter()
                .map(|&var| self.assign_list_0(var).into())
                .collect()
        };

        let code = {
            let mut code = vec![];
            code.extend(heapified_params_inits);
            code.extend(heapified_params_assigns);
            code.extend(heapified_locals);
            code.extend(closure.code);
            code
        };

        Closure {
            args: renamed_args,
            code: code,
        }.into()
    }
}

#[derive(Debug)]
struct NeedsHeapifying {
    vars: HashSet<Var>,
}

impl NeedsHeapifying {
    fn new() -> NeedsHeapifying {
        NeedsHeapifying {
            vars: HashSet::new()
        }
    }

    fn get(&self, var: Var) -> bool {
        self.vars.contains(&var)
    }
}

impl ::raise::VisitAst for NeedsHeapifying {
    fn closure(&mut self, closure: &Closure) {
        let free_vars = ::free_vars::free_vars(closure);
        self.vars.extend(free_vars);
        self.stmts(closure.code.as_slice());
    }
}
