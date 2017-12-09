use std::collections::HashSet;
use std::fmt;
use vm::fmt_indented;
use vm::Inst;
use vm::Term;
use cfg;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    name: String,
    index: usize,
}

pub struct Data {
    pub name: Block,
    pub body: Vec<Inst>,
    pub term: Term,
    pub pred: HashSet<Block>,
}

impl Block {
    pub fn from(b: cfg::Block) -> Block {
        let name = format!("{}", b);
        let index = b.inner();
        Block { name, index }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}:", self.name.name)?;
        for inst in &self.body {
            writeln!(f, "{}", fmt_indented(inst))?;
        }
        writeln!(f, "{}", fmt_indented(&self.term))?;
        Ok(())
    }
}
