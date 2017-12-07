use std::collections::HashMap;

pub mod module {
    use std::collections::HashMap;
    use std::fmt;
    use cfg;
    use vm::VarEnv;
    use vm::Func;
    use vm::FuncData;
    use vm::func::Builder as FuncBuilder;
    use explicate::VarData;
    use raise;

    pub struct Module {
        pub vars: VarEnv,
        pub funcs: HashMap<Func, FuncData>,
    }

    impl Module {
        pub fn new(m: cfg::Module) -> Self {
            let mut b = Builder::new(&m.var_data);
            for (f, function) in m.functions {
                b.visit_function(f, function, f == m.main);
            }
            b.build()
        }
    }

    struct Builder<'var_data> {
        var_data: &'var_data VarData,
        vars: VarEnv,
        funcs: HashMap<Func, FuncData>,
    }

    impl<'var_data> Builder<'var_data> {
        fn new(var_data: &'var_data VarData) -> Self {
            let vars = VarEnv::from(var_data);
            let funcs = HashMap::new();
            Builder { var_data, vars, funcs }
        }

        fn visit_function(&mut self, f: raise::Func, function: cfg::Function, is_main: bool) {
            let mut b = FuncBuilder::new(&mut self.vars, self.var_data);
            let func_data = b.build(f, function, is_main);
            self.funcs.insert(func_data.name.clone(), func_data);
        }

        fn build(self) -> Module {
            Module {
                vars: self.vars,
                funcs: self.funcs,
            }
        }
    }

    impl fmt::Display for Module {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for (_, func) in &self.funcs {
                write!(f, "{}", func)?;
            }
            Ok(())
        }
    }
}
pub use self::module::Module;

pub mod var {
    use std::fmt;
    use explicate::var;

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

    impl Env {
        pub fn from(var_data: &var::Slab<var::Data>) -> Env {
            let next = var_data.iter()
                .map(|(v, _)| v.inner())
                .max()
                .unwrap_or(0);
            Env { next }
        }
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
    use vm::VarEnv;
    use vm::Block;
    use vm::BlockData;
    use vm::StackLayout;
    use explicate::VarData;
    use cfg;
    use raise;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Func {
        name: String,
    }

    pub struct Data {
        pub name: Func,
        pub args: Vec<Var>,
        pub blocks: HashMap<Block, BlockData>,
        pub stack: StackLayout,
    }

    pub struct Builder<'vars, 'var_data> {
        vars: &'vars mut VarEnv,
        var_data: &'var_data VarData,
    }

    impl<'vars, 'var_data> Builder<'vars, 'var_data> {
        pub fn new(vars: &'vars mut VarEnv, var_data: &'var_data VarData) -> Self {
            unimplemented!()
        }

        pub fn build(self, f: raise::Func, function: cfg::Function, is_main: bool) -> Data {
            unimplemented!()
        }
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

    pub enum Unary {
        Mov,
        Neg,
        Push,
        Pop,
        MovLabel,
    }

    pub enum Binary {
        Add,
        Sub,
        Sete,
        Setne,
        Or,
        And,
        Shr,
        Shl,
    }

    pub enum Data {
        Unary {
            opcode: Unary,
            arg: Rval
        },
        Binary {
            opcode: Binary,
            left: Rval,
            right: Rval
        },
        CallIndirect {
            arg: Rval,
        },
        Call {
            func: String,
        },
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

    pub struct Layout {

    }
}
pub use self::stack::Slot as StackSlot;
pub use self::stack::Data as StackSlotData;
pub use self::stack::Layout as StackLayout;

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

use std::fmt;

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
