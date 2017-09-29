#![feature(conservative_impl_trait)]
extern crate python_ir as ir;

macro_rules! set {
    ($($e:expr),*) => ({
        let mut set = HashSet::new();
        $(
            set.insert($e);
        )*
        set
    })
}

use std::collections::HashSet;
use std::fmt;

pub struct Liveness {
    // see course notes, k is statement index
    k: usize,
    live_after_k: HashSet<ir::Tmp>,
}

impl fmt::Display for Liveness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        if !self.live_after_k.is_empty() {
            let tmps: Vec<ir::Tmp> = self.live_after_k.iter().map(|&tmp| tmp).collect();
            write!(f, "{}", tmps[0])?;
            for tmp in &tmps[1..] {
                write!(f, ", {}", tmp)?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

pub fn compute(ir: &ir::Program) -> Vec<Liveness> {
    let mut stack = Vec::new();
    let mut live_after_k: HashSet<ir::Tmp> = HashSet::new();
    let mut live_before_k: HashSet<ir::Tmp>;

    // iterate backwards, following algorithm from course notes
    for (k, stmt) in ir.stmts.iter().enumerate().rev() {
        //
        // live_before_k = (live_after_k - w(stmt_k)) U r(stmt_k);
        //
        live_before_k = (&live_after_k - &w(stmt)).union(&r(stmt)).map(|&tmp| tmp).collect();

        let live = Liveness { k, live_after_k: live_after_k.clone() };
        stack.push(live);

        // k = k - 1, so live_after_(k-1) == live_before_k
        live_after_k = live_before_k;
    }

    stack.reverse();
    stack
}

pub fn debug_print(ir: &ir::Program) {
    let liveness = compute(ir);
    for (l, s) in liveness.iter().zip(ir.stmts.iter()) {
        let s = format!("{}", s);
        println!("{: <3} {}", l.k, s);
        println!("{: <3} {:24} {}", "", "", l);
    }
}

fn w(stmt: &ir::Stmt) -> HashSet<ir::Tmp> {
    use ir::Stmt::*;
    match *stmt {
        Print(val) => set!(),
        Def(tmp, ref expr) => set!(tmp),
    }
}

fn r(stmt: &ir::Stmt) -> HashSet<ir::Tmp> {
    use ir::Stmt::*;
    match *stmt {
        Print(ref val) => r_val(val),
        Def(_, ref expr) => r_expr(expr),
    }
}

fn r_val(val: &ir::Val) -> HashSet<ir::Tmp> {
    use ir::Val::*;
    match *val {
        Int(_) => set!(),
        Ref(tmp) => set!(tmp),
    }
}

fn r_expr(expr: &ir::Expr) -> HashSet<ir::Tmp> {
    use ir::Expr::*;
    match *expr {
        UnaryNeg(ref val) => r_val(val),
        // r_val(l) U r_val(r)
        Add(ref l, ref r) => r_val(l).union(&r_val(r)).map(|&tmp| tmp).collect(),
        Input => set!(),
    }
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
