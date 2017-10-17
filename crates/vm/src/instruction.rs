use ir;
use LValue;
use RValue;

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Instruction {
    Mov(RValue, LValue),
    Neg(LValue),
    Add(RValue, LValue),
    Push(RValue),
    Call(String),
}

impl Instruction {
    pub fn replace_with(&mut self, tmp: ir::Tmp, new: LValue) {
        use self::Instruction::*;
        use self::LValue::*;
        use self::RValue::*;
        match *self {
            Mov(Int(_), ref mut lval) => lval.replace_with(tmp, new),
            Mov(LValue(ref mut l), ref mut r) => {
                l.replace_with(tmp, new);
                r.replace_with(tmp, new);
            }
            Neg(ref mut lval) => lval.replace_with(tmp, new),
            Add(Int(_), ref mut lval) => lval.replace_with(tmp, new),
            Add(LValue(ref mut l), ref mut r) => {
                l.replace_with(tmp, new);
                r.replace_with(tmp, new);
            }
            Push(Int(_)) => {}
            Push(LValue(ref mut lval)) => lval.replace_with(tmp, new),
            Call(_) => {}
        }
    }

    pub fn replace_with_stack(&mut self, tmp: ir::Tmp, stack_index: usize) {
        self.replace_with(tmp, LValue::Stack(stack_index));
    }

    pub fn tmps(&self) -> HashSet<ir::Tmp> {
        use self::Instruction::*;
        use self::LValue::*;
        use self::RValue::*;

        fn union(lhs: HashSet<ir::Tmp>, rhs: HashSet<ir::Tmp>) -> HashSet<ir::Tmp> {
            lhs.union(&rhs).map(|&v| v).collect()
        }

        match *self {
            Mov(LValue(left), right) => union(left.tmp(), right.tmp()),
            Mov(Int(_), right) => right.tmp(),
            Neg(lval) => lval.tmp(),
            Add(LValue(left), right) => union(left.tmp(), right.tmp()),
            Add(Int(_), right) => right.tmp(),
            Push(LValue(lval)) => lval.tmp(),
            Push(_) => HashSet::new(),
            Call(_) => HashSet::new(),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;
        match *self {
            Mov(rval, lval) => write!(f, "movl {}, {}", rval, lval),
            Neg(lval) => write!(f, "negl {}", lval),
            Add(rval, lval) => write!(f, "addl {}, {}", rval, lval),
            Push(rval) => write!(f, "pushl {}", rval),
            Call(ref s) => write!(f, "call {}", s),
        }
    }
}
