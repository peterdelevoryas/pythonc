use std::collections::HashSet;
use explicate::*;
use raise::VisitAst;

pub fn free_vars<N: ?Sized>(node: &N) -> HashSet<Var>
where
    N: FreeVars,
{
    node.free_vars()
}

pub trait FreeVars {
    fn free_vars(&self) -> HashSet<Var>;
}

// get the free vars for a module
impl FreeVars for [Stmt] {
    fn free_vars(&self) -> HashSet<Var> {
        let mut collect = Collect::new();
        collect.stmts(self);
        collect.free_vars()
    }
}

// get the free vars for a closure
impl FreeVars for Closure {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = self.code.free_vars();
        // remove free variables that are
        // defined by args list to closure
        free_vars.retain(|fv| !self.args.contains(fv));
        free_vars
    }
}

#[derive(Debug, Clone)]
struct Collect {
    defined: HashSet<Var>,
    used: HashSet<Var>,
}

impl Collect {
    fn new() -> Collect {
        Collect {
            defined: HashSet::new(),
            used: HashSet::new(),
        }
    }

    fn defined(&mut self, var: Var) {
        self.defined.insert(var);
    }

    fn is_defined(&self, var: Var) -> bool {
        self.defined.contains(&var)
    }

    fn used(&mut self, var: Var) {
        self.used.insert(var);
    }

    fn is_used(&self, var: Var) -> bool {
        self.used.contains(&var)
    }

    fn free_vars(&self) -> HashSet<Var> {
        self.used.difference(&self.defined).map(|&v| v).collect()
    }
}

impl VisitAst for Collect {
    fn closure(&mut self, closure: &Closure) {
        // don't enter closure, only visit ast nodes
        // in the scope we were called from!
    }

    fn target_var(&mut self, &var: &Var) {
        self.defined(var);
    }

    fn var(&mut self, &var: &Var) {
        self.used(var);
    }

    fn closure_var(&mut self, var: &Var) {
        panic!("Closure's should not be entered during free_vars::Collect!");
    }

    fn let_var(&mut self, &var: &Var) {
        self.defined(var);
    }
}
