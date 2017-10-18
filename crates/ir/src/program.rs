use std::fmt;
use statement::Stmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (k, stmt) in self.stmts.iter().enumerate() {
            writeln!(f, "{:<3} {}", k, stmt)?;
        }
        Ok(())
    }
}
