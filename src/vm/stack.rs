use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Slot {
    index: usize,
}

pub enum Data {
    Param { index: usize },
    Spill { index: usize },
}

pub struct Layout {}

impl Layout {
    pub fn new() -> Self {
        Layout {}
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[stack slot] {}", self.index)
    }
}
