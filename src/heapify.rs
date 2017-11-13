use std::collections::HashMap;
use error::*;
use explicate::*;
use raise::TransformAst;
use std::collections::HashSet;

pub struct Builder {
    free_vars: HashSet<Var>,
}

impl Builder {
    pub fn build(m: Module) -> Module {
        let mut builder = Builder {
            free_vars: HashSet::new(),
        };
        let stmts = m.stmts.into_iter().map(|stmt| {
            builder.stmt(stmt)
        }).collect();
        Module { stmts }
    }
}

impl TransformAst for Builder {
}
