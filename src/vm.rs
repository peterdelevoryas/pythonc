use std::collections::HashMap;

pub struct Module {
    pub vars: VarEnv,
    pub funcs: HashMap<Func, FuncData>,
}

pub mod var {
    use std::fmt;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Var {
        inner: Inner,
        index: usize,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum Inner {
        Temp,
        User {
            name: String,
        },
    }

    pub struct Env {
        next: usize,
    }

    impl fmt::Display for Var {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.inner {
                Inner::Temp => {
                    write!(f, "_{}", self.index)
                }
                Inner::User { ref name } => {
                    write!(f, "{}.{}", name, self.index)
                }
            }
        }
    }
}
pub use self::var::Var;
pub use self::var::Env as VarEnv;

pub mod func {
    use std::collections::HashMap;
    use std::fmt;
    use vm::Var;
    use vm::Block;
    use vm::BlockData;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Func {
        name: String,
    }

    pub struct Data {
        pub name: Func,
        pub args: Vec<Var>,
        pub blocks: HashMap<Block, BlockData>,
    }

    impl fmt::Display for Data {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "func {name}({args}) {{",
                name=self.name.name,
                args=::itertools::join(&self.args, ", "),
            )?;

            for (_, block) in &self.blocks {
                writeln!(f, "{}", block)?;
            }

            writeln!(f, "}}")?;

            Ok(())
        }
    }
}
pub use self::func::Func;
pub use self::func::Data as FuncData;

pub mod reg {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum Reg {
        EAX,
        EBX,
        ECX,
        EDX,
        ESI,
        EDI,
        ESP,
        EBP,
    }
}
pub use self::reg::Reg;

pub mod inst {
    use std::fmt;
    use vm::Reg;
    use vm::StackSlot;
    use vm::Var;

    pub struct Inst {
        pub dst: Lval,
        pub data: Data,
    }

    pub enum Data {
        Unary(Rval),
        Binary(Rval, Rval),
    }

    pub type Imm = i32;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Lval {
        Reg(Reg),
        StackSlot(StackSlot),
        Var(Var),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Rval {
        Imm(Imm),
        Lval(Lval),
    }

    impl fmt::Display for Inst {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }
}
pub use self::inst::Inst;
pub use self::inst::Data as InstData;
pub use self::inst::Imm;
pub use self::inst::Lval;
pub use self::inst::Rval;

pub mod stack {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Slot {
        index: usize,
    }

    pub enum Data {
        Param {
            index: usize,
        },
        Spill {
            index: usize,
        },
    }

}
pub use self::stack::Slot as StackSlot;

pub mod term {
    use std::fmt;
    use vm::Block;
    use vm::Var;

    pub enum Term {
        Return {
            var: Option<Var>,
        },
        Goto {
            block: Block,
        },
        Switch {
            cond: Var,
            then: Block,
            else_: Block,
        },
    }

    impl fmt::Display for Term {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            unimplemented!()
        }
    }
}
pub use self::term::Term;

pub mod block {
    use std::collections::HashSet;
    use std::fmt;
    use vm::fmt_indented;
    use vm::Inst;
    use vm::Term;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Block {
        name: String,
    }

    pub struct Data {
        pub name: Block,
        pub body: Vec<Inst>,
        pub term: Term,
        pub pred: HashSet<Block>,
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
}
pub use self::block::Block;
pub use self::block::Data as BlockData;

use cfg;

impl Module {
    pub fn new(m: cfg::Module) -> Self {
        unimplemented!()
    }
}

use std::fmt;

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_, func) in &self.funcs {
            write!(f, "{}", func)?;
        }
        Ok(())
    }
}

pub fn fmt_indented<T>(data: &T) -> String
where
    T: fmt::Display
{
    let s = format!("{}", data);
    indented(&s)
}

pub fn indented(s: &str) -> String {
    let mut indented = String::new();
    // just saw end of line
    let mut eol = true;
    for c in s.chars() {
        match c {
            '\n' if eol => {
                indented.push(c);
            }
            '\n' if !eol => {
                indented.push(c);
                eol = true;
            }
            c if eol => {
                indented.push_str("    ");
                indented.push(c);
                eol = false;
            }
            c if !eol => {
                indented.push(c);
            }
            _ => unreachable!()
        }
    }

    return indented
}
