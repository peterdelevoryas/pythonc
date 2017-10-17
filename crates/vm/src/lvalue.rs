use ir;
use trans;

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum LValue {
    Tmp(ir::Tmp),
    Register(trans::Register),
    Stack(usize),
}

impl LValue {
    /// If this LValue is a Tmp, it is replaced with the new value,
    /// otherwise it is not modified
    pub fn replace_with(&mut self, tmp: ir::Tmp, new: LValue) {
        use self::LValue::*;
        use std::mem;
        match *self {
            Tmp(t) if t == tmp => {
                mem::replace(self, new);
            }
            _ => {}
        }
    }

    /// TODO replace with stuff
    pub fn tmp(&self) -> HashSet<ir::Tmp> {
        use self::LValue::*;
        match *self {
            Tmp(t) => {
                let mut set = HashSet::new();
                set.insert(t);
                set
            }
            _ => HashSet::new(),
        }
    }
}

impl fmt::Display for LValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LValue::Tmp(tmp) => write!(f, "{}", tmp),
            LValue::Register(r) => write!(f, "{}", trans::Att(&r)),
            LValue::Stack(index) => {
                let offset = (index as i32 + 1) * -4;
                let mem = trans::Memory {
                    base: trans::Register::EBP,
                    index: None,
                    displacement: trans::Displacement(offset as i32),
                };
                write!(f, "{}", trans::Att(&mem))
            }
        }
    }
}
