use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tmp {
    pub index: usize,
}

impl Tmp {
    pub fn new(index: usize) -> Tmp {
        Tmp { index }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl fmt::Display for Tmp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "t{}", self.index)
    }
}

#[derive(Debug)]
pub struct Allocator {
    next: usize,
}

impl Allocator {
    pub fn new() -> Allocator {
        Allocator { next: 0 }
    }

    pub fn alloc(&mut self) -> Option<Tmp> {
        let tmp = Tmp { index: self.next };
        self.next = match self.next.checked_add(1) {
            Some(sum) => sum,
            None => return None,
        };
        Some(tmp)
    }
}
