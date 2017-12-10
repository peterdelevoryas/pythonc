use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Slot {
    Param { index: usize },
    Spill { index: usize },
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Slot::Param { index } => write!(f, "{}(%ebp)", 4 * (index + 5)),
            Slot::Spill { index } => write!(f, "{}(%ebp)", -4 * (index as i32 + 1)),
        }
    }
}
