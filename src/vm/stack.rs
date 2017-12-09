use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Slot {
    Param { index: usize },
    Spill { index: usize },
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Slot::Param { index } => write!(f, "[param {}]", index),
            Slot::Spill { index } => write!(f, "[spill {}]", index),
        }
    }
}
