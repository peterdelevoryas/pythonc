use std::collections::HashSet;
use explicate::*;
use raise::VisitAst;

pub fn free_vars<N>(node: &N) -> HashSet<Var>
where
    N: FreeVars
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
        collect.free_vars
    }
}

// get the free vars for a closure
impl FreeVars for Closure {
    fn free_vars(&self) -> HashSet<Var> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
struct Collect {
    free_vars: HashSet<Var>,
}

impl Collect {
    fn new() -> Collect {
        Collect { free_vars: HashSet::new() }
    }
}

impl VisitAst for Collect {
    fn closure(&mut self, closure: &Closure) {
        // don't enter closure, only visit ast nodes
        // in the scope we were called from!
    }
}
