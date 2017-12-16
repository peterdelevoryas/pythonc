use error::*;
use flatten::Flattener;
use std::fmt;

pub struct Program {
}

impl Program {
    pub fn from(flattener: Flattener) -> Program {
        Program {}
    }
}

struct Builder {

}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "ssa {{}}")?;
        Ok(())
    }
}
