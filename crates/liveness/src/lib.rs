#![feature(conservative_impl_trait)]
extern crate python_ir as ir;

use std::collections::HashSet;

pub struct Liveness {
    // see course notes, k is statement index
    k: usize,
    live_after_k: HashSet<ir::Tmp>,
}

pub fn compute(ir: &ir::Program) -> Vec<Liveness> {
    let mut stack = Vec::new();

    for (k, stmt) in ir.stmts.iter().enumerate().rev() {
        let live = Liveness {
            k,
            live_after_k: HashSet::new(),
        };
        stack.push(live);
    }

    stack
}

#[cfg(test)]
mod tests {
    use super::compute;
    use ir;

    #[test]
    fn enumerate_backwards() {
        let ir = ir::Program {
            stmts: vec![
                ir::Stmt::Print(ir::Val::Int(1)),
                ir::Stmt::Print(ir::Val::Int(2)),
                ir::Stmt::Print(ir::Val::Int(3)),
            ]
        };
        let liveness = compute(&ir);
        for (i, l) in liveness.iter().enumerate() {
            assert_eq!(l.k, liveness.len() - i - 1);
        }
    }
}
