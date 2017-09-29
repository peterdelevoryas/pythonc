#![feature(conservative_impl_trait)]
extern crate python_ir as ir;
extern crate python_vm as vm;
extern crate python_trans as trans;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Val {
    Virtual(ir::Tmp),
    Register(trans::Register),
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Val::*;
        match *self {
            Virtual(tmp) => write!(f, "{}", tmp),
            Register(r) => write!(f, "{}", trans::Att(&r)),
        }
    }
}

pub struct Liveness {
    // see course notes, k is statement index
    pub k: usize,
    pub live_after_k: HashSet<Val>,
}

impl fmt::Display for Liveness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        if !self.live_after_k.is_empty() {
            let mut tmps: Vec<Val> = self.live_after_k.iter().map(|&tmp| tmp).collect();
            tmps.sort_by(|l, r| {
                use self::Val::*;
                use std::cmp::Ordering::*;
                match (*l, *r) {
                    (Virtual(l), Virtual(r)) => l.index.cmp(&r.index),
                    (Virtual(_), Register(_)) => Less,
                    (Register(_), Virtual(_)) => Greater,
                    (Register(l), Register(r)) =>
                        format!("{}", trans::Att(&l)).cmp(&format!("{}", trans::Att(&r)))
                }
            });
            write!(f, "{}", tmps[0])?;
            for tmp in &tmps[1..] {
                write!(f, ", {}", tmp)?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

pub fn compute_vm(vm: &vm::Program) -> Vec<Liveness> {
    let mut stack = Vec::new();
    let mut live_after_k: HashSet<Val> = HashSet::new();
    let mut live_before_k: HashSet<Val>;

    // iterate backwards, following algorithm from course notes
    for (k, instr) in vm.stack.iter().enumerate().rev() {
        //
        // live_before_k = (live_after_k - w(stmt_k)) U r(stmt_k);
        //
        live_before_k = (&live_after_k - &w_vm(instr)).union(&r_vm(instr)).map(|&tmp| tmp).collect();

        let live = Liveness { k, live_after_k: live_after_k.clone() };
        stack.push(live);

        // k = k - 1, so live_after_(k-1) == live_before_k
        live_after_k = live_before_k;
    }

    stack.reverse();
    stack
}

/*
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
*/

pub fn debug_print_vm(vm: &vm::Program) {
    let liveness = compute_vm(vm);
    for (l, s) in liveness.iter().zip(vm.stack.iter()) {
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

fn w_vm(instr: &vm::Instr) -> HashSet<Val> {
    use vm::Instr::*;
    match *instr {
        Mov(val, tmp) => set!(Val::Virtual(tmp)),
        Neg(tmp) => set!(Val::Virtual(tmp)),
        Add(val, tmp) => set!(Val::Virtual(tmp)),
        Call(_) => set!(
            Val::Register(trans::Register::EAX),
            Val::Register(trans::Register::ECX),
            Val::Register(trans::Register::EDX)
        ),
        _ => set!(),
    }
}

fn r_vm(instr: &vm::Instr) -> HashSet<Val> {
    use vm::Instr::*;
    match *instr {
        Mov(ref val, _) => r_val_vm(val),
        Add(ref val, tmp) => r_val_vm(val).union(&set!(Val::Virtual(tmp))).map(|&tmp| tmp).collect(),
        Push(ref val) => r_val_vm(val),
        _ => set!(),
    }
}

fn r_val_vm(val: &vm::Val) -> HashSet<Val> {
    match *val {
        vm::Val::Virtual(tmp) => set!(Val::Virtual(tmp)),
        vm::Val::Register(r) => set!(Val::Register(r)),
        _ => set!(),
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
