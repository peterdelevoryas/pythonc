use std::fmt;
use vm::Block;
use vm::Var;
use vm::Rval;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Return { rval: Option<Rval> },
    Goto { block: Block },
    Switch {
        cond: Rval,
        then: Block,
        else_: Block,
    },
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term::Return { ref rval } => {
                write!(f, "return")?;
                if let Some(ref rval) = *rval {
                    write!(f, " {}", rval)?;
                }
                Ok(())
            }
            Term::Goto { ref block } => {
                write!(f, "goto {}", block)
            }
            Term::Switch { ref cond, ref then, ref else_ } => {
                write!(f, "switch {} [{}, {}]", cond, then, else_)
            }
        }
    }
}
