use std::collections::HashSet;
use explicate::*;
use raise::VisitAst;

#[derive(Debug, Clone)]
pub struct FreeVars(HashSet<Var>);

impl ::std::ops::Deref for FreeVars {
    type Target = HashSet<Var>;
    fn deref(&self) -> &HashSet<Var> {
        &self.0
    }
}
