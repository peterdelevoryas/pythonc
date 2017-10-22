use bb;
use val::Val;
use std::fmt;

#[derive(Debug)]
pub enum Term {
    Goto(bb::BasicBlock),
    SwitchInt {
        cond: Val,
        goto: Vec<(i32, bb::BasicBlock)>
    },
    Return,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term::Goto(ref bb) => write!(f, "goto {}", bb),
            Term::SwitchInt {
                cond,
                ref goto,
            } => {
                write!(f, "switch {} [", cond)?;
                assert!(!goto.is_empty());
                write!(f, "{} -> {}", goto[0].0, goto[0].1)?;
                for &(int, bb) in &goto[1..] {
                    write!(f, ", {} -> {}", int, bb)?;
                }
                write!(f, "]")
            }
            Term::Return => write!(f, "return"),
        }
    }
}
