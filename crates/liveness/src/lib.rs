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
                    (Register(l), Register(r)) => {
                        format!("{}", trans::Att(&l)).cmp(&format!("{}", trans::Att(&r)))
                    }
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
        live_before_k = (&live_after_k - &write_set(instr))
            .union(&read_set(instr))
            .map(|&tmp| tmp)
            .collect();

        let live = Liveness {
            k,
            live_after_k: live_after_k.clone(),
        };
        stack.push(live);

        // k = k - 1, so live_after_(k-1) == live_before_k
        live_after_k = live_before_k;
    }

    stack.reverse();
    stack
}

pub fn debug_print(vm: &vm::Program) {
    let liveness = compute_vm(vm);
    for (l, s) in liveness.iter().zip(vm.stack.iter()) {
        let s = format!("{}", s);
        println!("{: <3} {}", l.k, s);
        println!("{: <3} {:24} {}", "", "", l);
    }
}

fn lval_to_val(lval: vm::LVal) -> Option<Val> {
    match lval {
        vm::LVal::Tmp(tmp) => Some(Val::Virtual(tmp)),
        vm::LVal::Register(r) => Some(Val::Register(r)),
        vm::LVal::Stack(_) => None,
    }
}

fn rval_to_val(rval: vm::RVal) -> Option<Val> {
    match rval {
        vm::RVal::LVal(lval) => lval_to_val(lval),
        vm::RVal::Int(i) => None,
    }
}

fn option_to_set(opt: Option<Val>) -> HashSet<Val> {
    match opt {
        Some(val) => set!(val),
        None => set!(),
    }
}

fn union(lhs: HashSet<Val>, rhs: HashSet<Val>) -> HashSet<Val> {
    lhs.union(&rhs).map(|&v| v).collect()
}

fn rval_(rval: vm::RVal) -> HashSet<Val> {
    option_to_set(rval_to_val(rval))
}
fn lval_(lval: vm::LVal) -> HashSet<Val> {
    option_to_set(lval_to_val(lval))
}

fn write_set(instr: &vm::Instr) -> HashSet<Val> {
    use vm::Instr::*;
    match *instr {
        // writes the destination
        Mov(_, lval) => lval_(lval),
        // writes the destination
        Neg(lval) => lval_(lval),
        // writes the destination
        Add(_, lval) => lval_(lval),
        // writes the stack, so nothing
        Push(_) => set!(),
        // writes caller-save registers
        Call(_) => {
            set!(
                Val::Register(trans::Register::EAX),
                Val::Register(trans::Register::ECX),
                Val::Register(trans::Register::EDX)
            )
        }
        //_ => set!(),
    }
}

fn read_set(instr: &vm::Instr) -> HashSet<Val> {
    use vm::Instr::*;
    match *instr {
        // only read from the source of mov's
        Mov(rval, _) => rval_(rval),
        // negation first reads from destination before writing
        Neg(lval) => lval_(lval),
        // read from the source and destination on add's
        Add(rval, lval) => union(rval_(rval), lval_(lval)),
        // just read from the rval
        Push(rval) => rval_(rval),
        // this just reads from the stack
        Call(_) => set!(),
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
            ],
        };
        let liveness = compute(&ir);
        for (i, l) in liveness.iter().enumerate() {
            assert_eq!(l.k, liveness.len() - i - 1);
        }
    }
}
