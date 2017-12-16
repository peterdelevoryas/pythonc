use ssa::Val;
use ssa::Rval;
use ssa::Block;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Ret {
        ret: Option<Rval>,
    },
    Goto {
        block: Block,
    },
    Switch {
        cond: Rval,
        then: Block,
        else_: Block,
    },
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term::Ret { ref ret } => {
                write!(f, "return")?;
                if let Some(rval) = *ret {
                    write!(f, " {}", rval)?;
                }
                Ok(())
            }
            Term::Goto { block } => {
                write!(f, "goto {}", block)
            }
            Term::Switch { cond, then, else_ } => {
                write!(f, "switch {} [{}, {}]", cond, then, else_)
            }
        }
    }
}
