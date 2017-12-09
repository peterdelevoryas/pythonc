use std::fmt;
use vm::Block;
use vm::Var;

pub enum Term {
    Return { var: Option<Var> },
    Goto { block: Block },
    Switch {
        cond: Var,
        then: Block,
        else_: Block,
    },
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term::Return { ref var } => {
                write!(f, "return")?;
                if let Some(ref var) = *var {
                    write!(f, " {}", var)?;
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
