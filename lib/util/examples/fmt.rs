#[macro_use] extern crate util;

use std::fmt;
use std::io;
use std::io::Write;
use util::fmt::Fmt;

pub enum Stmt {
    Def,
    If {
        then: Vec<Stmt>,
    }
}

impl util::fmt::Fmt for Stmt {
    fn fmt<W>(&self, f: &mut util::fmt::Formatter<W>) -> io::Result<()>
    where
        W: Write,
    {
        match *self {
            Stmt::Def => {
                writeln!(f, "x = y;")?;
            }
            Stmt::If { ref then } => {
                f.indent();
                for stmt in then {
                    f.fmt(stmt)?;
                }
                f.dedent();
            }
        }
        Ok(())
    }
}

fn main() {
    let program = vec![
        Stmt::Def,
        Stmt::Def,
        Stmt::If {
            then: vec![
                Stmt::Def,
                Stmt::Def,
                Stmt::Def,
            ],
        },
        Stmt::Def,
    ];

    let mut f = util::fmt::Formatter::new(io::stdout());
    for stmt in &program {
        f.fmt(stmt);
    }
}
