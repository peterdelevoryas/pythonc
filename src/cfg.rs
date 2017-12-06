use explicate::Var;
use explicate::var;
use flatten::Expr;
use flatten as flat;
use std::collections::HashSet;
use std::collections::HashMap;
use raise;

type FlatBlock = Vec<flat::Stmt>;

pub struct Module {
    pub var_data: var::Slab<var::Data>,
    pub functions: HashMap<raise::Func, Function>,
    pub main: raise::Func,
}

impl Module {
    pub fn new(flattener: flat::Flattener) -> Module {
        let functions: HashMap<raise::Func, Function> = flattener.units.into_iter()
            .map(|(f, function)| {
                let function = Function::new(f, function);
                (f, function)
            })
            .collect();

        Module {
            var_data: flattener.var_data,
            functions: functions,
            main: flattener.main,
        }
    }
}

/// Contains all data for a function
pub struct Function {
    pub name: String,
    pub args: Vec<Var>,
    pub cfg: Cfg,
}

#[derive(Debug)]
pub enum Stmt {
    Def {
        lhs: Var,
        rhs: Expr,
    },
    Discard(Expr),
}

/// A statement which terminates a basic block
pub enum Term {
    /// Return from function
    Return,
    /// Unconditional jump
    Goto(bb::BasicBlock),
    /// Conditional jump
    ///     if cond {
    ///         goto then;
    ///     } else {
    ///         goto else_;
    ///     }
    Switch {
        cond: Var,
        then: bb::BasicBlock,
        else_: bb::BasicBlock,
    }
}

pub mod bb {
    use std::collections::HashSet;
    use super::Stmt;
    use super::Term;

    /// Basic block identifier
    impl_ref!(BasicBlock, "bb");

    /// Basic block data
    pub struct Data {
        /// Body of the basic block
        pub body: Vec<Stmt>,

        /// Last stmt in basic block,
        /// which results in leaving
        /// the basic block
        pub term: Option<Term>,

        /// Predecessors
        pub pred: HashSet<BasicBlock>,
    }

    pub type Set = Slab<Data>;
}

/// Control flow graph
pub struct Cfg {
    bbs: bb::Slab<bb::Data>,
}

impl Cfg {
    pub fn new(block: FlatBlock) -> Cfg {
        let mut builder = CfgBuilder::new();
        builder.enter_block();
        for stmt in block {
            builder.visit_stmt(stmt);
        }
        builder.exit_block();
        builder.complete()
    }
}

/// Builds a control flow graph from a flattened block.
struct CfgBuilder {
    curr: Vec<bb::BasicBlock>,
    bbs: bb::Set,
}

impl CfgBuilder {
    fn new() -> Self {
        Self {
            curr: Vec::new(),
            bbs: bb::Set::new(),
        }
    }

    fn visit_stmt(&mut self, stmt: flat::Stmt) {
        match stmt {
            flat::Stmt::Def(var, expr) => {
                self.current_basic_block().body.push(Stmt::Def { lhs: var, rhs: expr });
            }
            flat::Stmt::Discard(expr) => {
                self.current_basic_block().body.push(Stmt::Discard(expr));
            }
            flat::Stmt::Return(var) => {
                unimplemented!()
            }
            flat::Stmt::While(cond, header, body) => {
                unimplemented!()
            }
            flat::Stmt::If(cond, then, else_) => {
                unimplemented!()
            }
        }
    }

    /// Panics if there aren't any basic blocks
    fn current_basic_block(&mut self) -> &mut bb::Data {
        let &bb = self.curr.last().expect("no basic blocks");
        &mut self.bbs[bb]
    }

    fn enter_block(&mut self) {
        let data = bb::Data {
            body: Vec::new(),
            term: None,
            pred: HashSet::new(),
        };
        let bb = self.bbs.insert(data);
        self.curr.push(bb);
    }

    fn exit_block(&mut self) {
        let _ = self.curr.pop().expect("basic block not entered");
    }

    fn complete(self) -> Cfg {
        assert!(self.curr.is_empty());
        Cfg {
            bbs: self.bbs,
        }
    }
}

impl Function {
    pub fn new(f: raise::Func, function: flat::Function) -> Self {
        let mut cfg = Cfg::new(function.body);
        Function {
            name: format!("{}", f),
            args: function.args,
            cfg: cfg,
        }
    }
}

impl ::util::fmt::Fmt for Stmt {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::util::fmt::Result
    where
        W: ::std::io::Write,
    {
        use std::io::Write;
        match *self {
            Stmt::Def { ref lhs, ref rhs } => {
                write!(f, "{} = ", lhs)?;
                f.fmt(rhs)?;
            }
            Stmt::Discard(ref expr) => {
                f.fmt(expr)?;
            }
        }
        Ok(())
    }
}

impl ::util::fmt::Fmt for Term {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::util::fmt::Result
    where
        W: ::std::io::Write,
    {
        Ok(())
    }
}

impl ::util::fmt::Fmt for Cfg {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::util::fmt::Result
    where
        W: ::std::io::Write,
    {
        use std::io::Write;

        for (bb, data) in &self.bbs {
            writeln!(f, "{}:", bb)?;
            f.indent();
            for stmt in &data.body {
                f.fmt(stmt)?;
                writeln!(f)?;
            }
            if let Some(ref term) = data.term {
                f.fmt(term)?;
                writeln!(f)?;
            }
            f.dedent();
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'var_data> ::util::fmt::Fmt for Formatted<'var_data, Var> {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::util::fmt::Result
    where
        W: ::std::io::Write,
    {
        Ok(())
    }
}

impl<'var_data> ::util::fmt::Fmt for Formatted<'var_data, Function> {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::util::fmt::Result
    where
        W: ::std::io::Write,
    {
        use std::io::Write;

        write!(f, "func {}(", self.value.name)?;
        if !self.value.args.is_empty() {
            f.fmt(&Formatted {
                var_data: &self.var_data,
                value: &self.value.args[0]
            })?;
            for arg in &self.value.args[1..] {
                f.fmt(&Formatted {
                    var_data: &self.var_data,
                    value: arg,
                })?;
            }
        }
        writeln!(f, ") {{")?;
        f.fmt(&self.value.cfg)?;
        write!(f, "}}")?;

        Ok(())
    }
}

impl ::std::fmt::Display for Module {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use std::io::Write;

        let mut formatter = ::util::fmt::Formatter::new(Vec::new());
        for (f, function) in &self.functions {
            let formatted = Formatted {
                var_data: &self.var_data,
                value: function,
            };
            formatter.fmt(&formatted).map_err(|_| ::std::fmt::Error)?;
            writeln!(formatter).map_err(|_| ::std::fmt::Error)?;
        }
        let bytes = formatter.into_inner();
        let string = String::from_utf8(bytes).unwrap();
        write!(f, "{}", string)
    }
}

struct Formatted<'var_data, T: 'var_data> {
    var_data: &'var_data var::Slab<var::Data>,
    value: &'var_data T,
}

#[cfg(test)]
mod tests {
    use flatten;
    use super::*;

    const TESTS: &'static [&'static str] = &[
        "
x = 1
#if input():
    #x = 3
print x
        ",
    ];

    #[test]
    fn builder() {
        let mut pythonc = ::Pythonc::new();
        for test in TESTS {
            let flattener = pythonc.emit_flattened(test).unwrap();

        }
    }
}
