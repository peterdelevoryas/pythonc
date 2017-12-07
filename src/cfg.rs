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
        let main = flattener.main;
        let functions: HashMap<raise::Func, Function> = flattener.units.into_iter()
            .map(|(f, function)| {
                let function = Function::new(f, function, f == main);
                (f, function)
            })
            .collect();

        Module {
            var_data: flattener.var_data,
            functions: functions,
            main: main,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    /// Return from function
    Return(Option<Var>),
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

impl Term {
    pub fn is_return(&self) -> bool {
        match *self {
            Term::Return(_) => true,
            _ => false,
        }
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

    impl Data {
        pub fn is_terminated(&self) -> bool {
            self.term.is_some()
        }
    }
}

/// Control flow graph
pub struct Cfg {
    bbs: bb::Slab<bb::Data>,
}

impl Cfg {
    pub fn new(block: FlatBlock) -> Cfg {
        let mut builder = CfgBuilder::new();
        let last = builder.build_block(block);
        builder.terminate(last, Term::Return(None));
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

    /// Terminates bb with term and adds bb as a predecessor
    /// of the destination block (unless it's a return)
    fn terminate(&mut self, bb: bb::BasicBlock, term: Term) {
        // add predecessor/s
        match term {
            Term::Return(_) => {}
            Term::Goto(next) => {
                self.add_predecessor(next, bb);
            }
            Term::Switch { then, else_, .. } => {
                self.add_predecessor(then, bb);
                self.add_predecessor(else_, bb);
            }
        }

        // if the block is already terminated with a return,
        // then don't add a goto or switch
        // if we're trying to terminate a return with a return, panic!
        self.basic_block(bb).term = match self.basic_block(bb).term {
            Some(Term::Return(var)) if term.is_return() => {
                // don't replace, just use original return
                Some(Term::Return(var))
            }
            // just leave returns alone, if replace with goto or switch
            Some(Term::Return(var)) => Some(Term::Return(var)),
            Some(_) => {
                panic!("attempting to terminate a goto or switch block!");
            }
            None => Some(term),
        };
    }

    /// Adds pred to bb's set of predecssors
    fn add_predecessor(&mut self, bb: bb::BasicBlock, pred: bb::BasicBlock) {
        self.basic_block(bb).pred.insert(pred);
    }

    /// build_block takes a block of statements, which may contain nested
    /// blocks of statements, creates a series of basic block nodes that
    /// represent the control flow, and returns the final, unifying basic block
    /// identifier.
    fn build_block(&mut self, block: FlatBlock) -> bb::BasicBlock {
        let mut current = self.enter_new_block();
        for stmt in block {
            match stmt {
                flat::Stmt::Def(var, expr) => {
                    self.current_basic_block().body.push(Stmt::Def { lhs: var, rhs: expr });
                }
                flat::Stmt::Discard(expr) => {
                    self.current_basic_block().body.push(Stmt::Discard(expr));
                }
                flat::Stmt::Return(var) => {
                    let bb = self.exit_block();
                    self.terminate(bb, Term::Return(var));
                    // If there are still statements left in the block, then
                    // they don't get processed! we immediately return
                    // from here. This is an optimization. It shouldn't
                    // be possible for there to be any successors to this
                    // block.
                    return bb;
                }
                flat::Stmt::While(cond, header, body) => {
                    unimplemented!()
                }
                flat::Stmt::If(cond, then, else_) => {
                    // For an if statement, we first need to compute the
                    // `then` and `else` blocks. Then we can terminate
                    // the current block with a switch to those blocks.
                    // After that, we create a new block, which will become
                    // the current block, and represents the control
                    // flow after the if statement. We also have to
                    // terminate the `then` and `else` blocks with
                    // goto's to the new current block, and add predecessors
                    // to everything.

                    let prev = self.exit_block();
                    let then = self.build_block(then);
                    let else_ = self.build_block(else_);

                    current = self.enter_new_block();

                    self.terminate(then, Term::Goto(current));
                    self.terminate(else_, Term::Goto(current));
                    self.terminate(prev, Term::Switch { cond, then, else_ });

                }
            }
        }
        self.exit_block()
    }

    /// Panics if there aren't any basic blocks
    fn current_basic_block(&mut self) -> &mut bb::Data {
        let &bb = self.curr.last().expect("no basic blocks");
        self.basic_block(bb)
    }

    fn basic_block(&mut self, bb: bb::BasicBlock) -> &mut bb::Data {
        &mut self.bbs[bb]
    }

    fn re_enter_block(&mut self, bb: bb::BasicBlock) {
        assert!(self.bbs.contains(bb));
        self.curr.push(bb);
    }

    fn enter_new_block(&mut self) -> bb::BasicBlock {
        let data = bb::Data {
            body: Vec::new(),
            term: None,
            pred: HashSet::new(),
        };
        let bb = self.bbs.insert(data);
        trace!("entering block {}", bb);
        self.curr.push(bb);
        bb
    }

    fn exit_block(&mut self) -> bb::BasicBlock {
        let bb = self.curr.pop().expect("basic block not entered");
        trace!("exiting block {}", bb);
        bb
    }

    fn complete(self) -> Cfg {
        trace!("completing control flow graph");
        assert!(self.curr.is_empty());
        Cfg {
            bbs: self.bbs,
        }
    }
}

impl Function {
    pub fn new(f: raise::Func, function: flat::Function, is_main: bool) -> Self {
        let mut cfg = Cfg::new(function.body);
        let name = if is_main { "main".into() } else { format!("{}", f) };
        Function {
            name: name,
            args: function.args,
            cfg: cfg,
        }
    }
}

impl<'module> ::util::fmt::Fmt for Formatted<'module, Expr> {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::util::fmt::Result
    where
        W: ::std::io::Write,
    {
        use std::io::Write;
        match *self.value {
            Expr::UnaryOp(op, var) => {
                write!(f, "{} ", op)?;
                f.fmt(&self.formatted(&var))?;
            }
            Expr::BinOp(op, l, r) => {
                f.fmt(&self.formatted(&l))?;
                write!(f, " {} ", op)?;
                f.fmt(&self.formatted(&r))?;
            }
            Expr::CallFunc(target, ref args) => {
                f.fmt(&self.formatted(&target))?;
                write!(f, "(")?;
                if !args.is_empty() {
                    f.fmt(&self.formatted(&args[0]))?;
                    for arg in &args[1..] {
                        write!(f, ", ")?;
                        f.fmt(&self.formatted(arg))?;
                    }
                }
                write!(f, ")")?;
            }
            Expr::RuntimeFunc(ref name, ref args) => {
                write!(f, "{}(", name)?;
                if !args.is_empty() {
                    f.fmt(&self.formatted(&args[0]))?;
                    for arg in &args[1..] {
                        write!(f, ", ")?;
                        f.fmt(&self.formatted(arg))?;
                    }
                }
                write!(f, ")")?;
            }
            Expr::GetTag(var) => {
                write!(f, "get_tag ")?;
                f.fmt(&self.formatted(&var))?;
            }
            Expr::ProjectTo(var, ty) => {
                write!(f, "project ")?;
                f.fmt(&self.formatted(&var))?;
                write!(f, " to {}", ty)?;
            }
            Expr::InjectFrom(var, ty) => {
                write!(f, "inject ")?;
                f.fmt(&self.formatted(&var))?;
                write!(f, " from {}", ty)?;
            }
            Expr::Const(i) => {
                write!(f, "const i32 {}", i)?;
            }
            Expr::LoadFunctionPointer(ref func) => {
                write!(f, "{}", self.module.functions[func].name)?;
            }
            Expr::Copy(var) => {
                write!(f, "copy ")?;
                f.fmt(&self.formatted(&var))?;
            }
        }

        Ok(())
    }
}

impl<'module> ::util::fmt::Fmt for Formatted<'module, Stmt> {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::util::fmt::Result
    where
        W: ::std::io::Write,
    {
        use std::io::Write;
        match *self.value {
            Stmt::Def { ref lhs, ref rhs } => {
                f.fmt(&self.formatted(lhs))?;
                write!(f, " = ")?;
                f.fmt(&self.formatted(rhs))?;
            }
            Stmt::Discard(ref expr) => {
                f.fmt(&self.formatted(expr))?;
            }
        }
        Ok(())
    }
}

impl<'module> ::util::fmt::Fmt for Formatted<'module, Term> {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::util::fmt::Result
    where
        W: ::std::io::Write,
    {
        use std::io::Write;

        match *self.value {
            Term::Return(var) => {
                write!(f, "return")?;
                if let Some(var) = var {
                    write!(f, " ")?;
                    f.fmt(&self.formatted(&var))?;
                }
            }
            Term::Goto(bb) => {
                write!(f, "goto {}", bb)?;
            }
            Term::Switch { cond, then, else_ } => {
                write!(f, "if ")?;
                f.fmt(&self.formatted(&cond))?;
                writeln!(f, " {{")?;
                f.indent();
                writeln!(f, "goto {}", then)?;
                f.dedent();
                writeln!(f, "}} else {{")?;
                f.indent();
                writeln!(f, "goto {}", else_)?;
                f.dedent();
                write!(f, "}}")?;
            }
        }

        Ok(())
    }
}

impl<'module> ::util::fmt::Fmt for Formatted<'module, Cfg> {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::util::fmt::Result
    where
        W: ::std::io::Write,
    {
        use std::io::Write;

        for (bb, data) in &self.value.bbs {
            writeln!(f, "{}:", bb)?;
            f.indent();
            for stmt in &data.body {
                f.fmt(&self.formatted(stmt))?;
                writeln!(f)?;
            }
            if let Some(ref term) = data.term {
                f.fmt(&self.formatted(term))?;
                writeln!(f)?;
            }
            f.dedent();
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'module> ::util::fmt::Fmt for Formatted<'module, Var> {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::util::fmt::Result
    where
        W: ::std::io::Write,
    {
        use std::io::Write;

        match self.module.var_data[*self.value] {
            var::Data::User { ref source_name } => {
                write!(f, "{}.{}", source_name, self.value.inner())
            }
            var::Data::Temp => {
                write!(f, "%{}", self.value.inner())
            }
        }
    }
}

impl<'module> ::util::fmt::Fmt for Formatted<'module, Function> {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::util::fmt::Result
    where
        W: ::std::io::Write,
    {
        use std::io::Write;

        write!(f, "func {}(", self.value.name)?;
        if !self.value.args.is_empty() {
            f.fmt(&self.formatted(&self.value.args[0]))?;
            for arg in &self.value.args[1..] {
                write!(f, ", ")?;
                f.fmt(&self.formatted(arg))?;
            }
        }
        writeln!(f, ") {{")?;
        f.fmt(&self.formatted(&self.value.cfg))?;
        writeln!(f, "}}")?;

        Ok(())
    }
}

impl<'module> ::util::fmt::Fmt for Formatted<'module, Module> {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::util::fmt::Result
    where
        W: ::std::io::Write
    {
        use std::io::Write;

        for (_, function) in &self.value.functions {
            let formatted = self.formatted(function);
            f.fmt(&formatted)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

impl ::std::fmt::Display for Module {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut formatter = ::util::fmt::Formatter::new(Vec::new());
        let formatted = Formatted::new(self, self);
        formatter.fmt(&formatted).map_err(|_| ::std::fmt::Error)?;
        let bytes = formatter.into_inner();
        let string = String::from_utf8(bytes).unwrap();
        write!(f, "{}", string)
    }
}

struct Formatted<'module, T: 'module> {
    module: &'module Module,
    value: &'module T,
}

impl<'module, T> Formatted<'module, T>
where
    T: 'module
{
    fn new(module: &'module Module, value: &'module T) -> Self {
        Formatted {
            module: module,
            value: value,
        }
    }

    fn formatted<V>(&self, value: &'module V) -> Formatted<'module, V> {
        Formatted {
            module: self.module,
            value: value,
        }
    }
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
