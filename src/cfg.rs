use explicate::Var;
use explicate::var;
use flatten::Expr;
use flatten as flat;
use std::collections::HashMap;
use std::collections::HashSet;
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Stmt {
    Def {
        lhs: Var,
        rhs: Expr,
    },
    Discard(Expr),
}

impl Stmt {
    pub fn uses(&self) -> HashSet<Var> {
        match *self {
            Stmt::Def { ref rhs, .. } => rhs.uses(),
            Stmt::Discard(ref expr) => expr.uses(),
        }
    }

    pub fn defs(&self) -> HashSet<Var> {
        match *self {
            Stmt::Def { lhs, .. } => hash_set!(lhs),
            Stmt::Discard(_) => hash_set!(),
        }
    }
}

/// A statement which terminates a basic block
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    /// Return from function
    Return(Option<Var>),
    /// Unconditional jump
    Goto(Block),
    /// Conditional jump
    ///     if cond {
    ///         goto then;
    ///     } else {
    ///         goto else_;
    ///     }
    Switch {
        cond: Var,
        then: Block,
        else_: Block,
    }
}

impl Term {
    pub fn is_return(&self) -> bool {
        match *self {
            Term::Return(_) => true,
            _ => false,
        }
    }

    pub fn uses(&self) -> HashSet<Var> {
        match *self {
            Term::Return(Some(var)) => hash_set!(var),
            Term::Return(None) => hash_set!(),
            Term::Switch { cond, .. } => hash_set!(cond),
            Term::Goto(_) => hash_set!(),
        }
    }
}

pub mod block {
    use std::collections::HashSet;
    use super::Stmt;
    use super::Term;
    use explicate::Var;

    /// Block identifier
    impl_ref!(Block, "b");

    /// Basic block data
    pub struct Data {
        /// Body of the basic block
        pub body: Vec<Stmt>,

        /// Last stmt in basic block,
        /// which results in leaving
        /// the basic block
        pub term: Option<Term>,

        /// Predecessors
        pub pred: HashSet<Block>,
    }

    pub type Blocks = Slab<Data>;

    impl Data {
        pub fn is_terminated(&self) -> bool {
            self.term.is_some()
        }

        pub fn push(&mut self, stmt: Stmt) {
            self.body.push(stmt);
        }

        /// Returns an empty, unterminated block with no predecessors.
        pub fn empty() -> Self {
            Self {
                body: Vec::new(),
                term: None,
                pred: hash_set!(),
            }
        }

        pub fn predecessors(&self) -> HashSet<Block> {
            self.pred.clone()
        }

        pub fn successors(&self) -> HashSet<Block> {
            match *self.term.as_ref().unwrap() {
                Term::Return(_) => hash_set!(),
                Term::Goto(b) => hash_set!(b),
                Term::Switch { then, else_, .. } => hash_set!(then, else_),
            }
        }

        pub fn uses(&self) -> HashSet<Var> {
            let mut uses = hash_set!();

            for stmt in &self.body {
                uses.extend(stmt.uses());
            }

            if let Some(ref term) = self.term {
                uses.extend(term.uses());
            }

            uses
        }

        pub fn defs(&self) -> HashSet<Var> {
            let mut defs = hash_set!();

            for stmt in &self.body {
                defs.extend(stmt.defs());
            }

            defs
        }

        pub fn term(&self) -> &Term {
            self.term.as_ref().unwrap()
        }

        /// gen[pn] = gen[n] U (gen[p] - kill[n])
        pub fn gens(&self) -> HashSet<Var> {
            match self.body.len() {
                0 => hash_set!(),
                1 => self.body[0].uses(),
                _ => {
                    let mut gen_pn = {
                        let gen_n = &self.body[1].uses();
                        let gen_p = &self.body[0].uses();
                        let kill_n = &self.body[1].defs();
                        gen_n | &(gen_p - kill_n)
                    };
                    for stmt in &self.body[2..] {
                        let gen_n = &stmt.uses();
                        let kill_n = &stmt.defs();
                        gen_pn = gen_n | &(&gen_pn - kill_n);
                    }
                    gen_pn = &self.term().uses() | &gen_pn;
                    gen_pn
                }
            }
        }

        /// kill[pn] = kill[p] U kill[n]
        pub fn kills(&self) -> HashSet<Var> {
            let mut kill_pn = hash_set!();
            for stmt in &self.body {
                kill_pn = &kill_pn | &stmt.defs();
            }
            kill_pn
        }
    }
}
use self::block::Block;
use self::block::Blocks;
use self::block::Data as BlockData;

/// Control flow graph
pub struct Cfg {
    blocks: Blocks,
}

impl Cfg {
    pub fn new(block: FlatBlock) -> Cfg {
        let mut cfg = CfgBuilder::new();
        let root = cfg.new_block();
        cfg.enter(root);
        cfg.build_block(block);
        let last = cfg.exit();
        cfg.terminate(last, Term::Return(None));
        let cfg = cfg.complete();
        cfg.check_edges();
        assert_eq!(root, cfg.root());

        return cfg
    }

    /// Looks at all blocks in cfg, makes sure that there aren't
    /// any blocks whos predecessors or successors are incorrect.
    pub fn check_edges(&self) {
        for (b, block) in &self.blocks {
            for p in block.predecessors() {
                // assert predecessor contains self in successors
                assert!(self.block(p).successors().contains(&b));
            }
            for s in block.successors() {
                // assert successors contain self in predecessors
                assert!(self.block(s).predecessors().contains(&b));
            }
        }
    }

    pub fn block(&self, b: Block) -> &BlockData {
        &self.blocks[b]
    }

    /// Returns block with no predecessors
    pub fn root(&self) -> Block {
        let mut root = None;
        for (b, block) in &self.blocks {
            if block.predecessors().is_empty() {
                assert!(root.is_none());
                root = Some(b);
            }
        }

        root.unwrap()
    }
}

/// Builds a control flow graph from a flattened block.
struct CfgBuilder {
    curr: Option<Block>,
    blocks: Blocks,
}

impl CfgBuilder {
    fn new() -> Self {
        let curr = None;
        let blocks = Blocks::new();

        Self { curr, blocks }
    }

    /// Terminates block with term and adds block as a predecessor
    /// of the destination block of term (unless term is a return,
    /// in which case there is no destination block)
    fn terminate(&mut self, b: Block, t: Term) {
        // add predecessor/s
        match t {
            Term::Return(_) => {}
            Term::Goto(next) => {
                self.add_predecessor(next, b);
            }
            Term::Switch { then, else_, .. } => {
                self.add_predecessor(then, b);
                self.add_predecessor(else_, b);
            }
        }

        // if the block is already terminated with a return,
        // then don't add a goto or switch
        // if we're trying to terminate a return with a return, panic!
        self.block(b).term = match self.block(b).term {
            Some(Term::Return(var)) if t.is_return() => {
                // don't replace, just use original return
                Some(Term::Return(var))
            }
            // just leave returns alone, if replace with goto or switch
            Some(Term::Return(var)) => Some(Term::Return(var)),
            Some(_) => {
                panic!("attempting to terminate a goto or switch block!");
            }
            None => Some(t),
        };
    }

    /// Adds p to b's set of predecssors.
    fn add_predecessor(&mut self, b: Block, p: Block) {
        self.block(b).pred.insert(p);
    }

    /// build_block takes a block of statements, which may contain nested
    /// blocks of statements, creates a series of basic block nodes that
    /// represent the control flow, and returns the first and last basic block
    /// identifiers. These may be the same if the nested block is just a single
    /// basic block, but could be different if the block introduces branching.
    ///
    ///
    /// ```ignore
    ///            ...
    ///
    ///             |
    ///             |
    ///             v
    ///
    ///     [ before_while ]
    ///
    ///             |
    ///             | goto header_start
    ///             v
    ///
    ///     [ header_start ]   <----------------------------------------------+
    ///                                                                       |
    ///             |                                                         |
    ///             |                                                         |
    ///             v                                                         |
    ///                                                                       |
    ///            ...                                                        |
    ///                                                                       |
    ///             |                                                         |
    ///             |                                       goto header_start |
    ///             v                                                         |
    ///                                                                       |
    ///     [  header_end  ] // last header block                             |
    ///                                                                       |
    ///             |                                                         |
    ///             | switch cond [true -> body_start, false -> after_while]  |
    ///             |                                                         |
    ///             +---------> [ body_start ] -->  ...  --> [ body_end ] ----+
    ///             |
    ///             |
    ///             |
    ///             |
    ///             v
    ///
    ///     [  after_while ]
    ///
    ///             |
    ///             |
    ///             v
    ///
    ///            ...
    /// ```
    ///
    fn build_block(&mut self, flat_block: FlatBlock) {
        for stmt in flat_block {
            match stmt {
                flat::Stmt::Def(var, expr) => {
                    self.push(Stmt::Def { lhs: var, rhs: expr });
                }
                flat::Stmt::Discard(expr) => {
                    self.push(Stmt::Discard(expr));
                }
                flat::Stmt::Return(var) => {
                    let curr = self.curr.unwrap();
                    self.terminate(curr, Term::Return(var));
                    // If there are still statements left in the block, then
                    // they don't get processed! we immediately return
                    // from here. This is an optimization. It shouldn't
                    // be possible for there to be any successors to this
                    // block.

                    break
                }

                flat::Stmt::While(cond, header, body) => {
                    let before_while = self.exit();
                    let header_start = self.new_block();
                    self.terminate(before_while, Term::Goto(header_start));
                    self.enter(header_start);
                    self.build_block(header);
                    let header_end = self.exit();

                    let body_start = self.new_block();
                    self.enter(body_start);
                    self.build_block(body);
                    let body_end = self.exit();
                    self.terminate(body_end, Term::Goto(header_start));

                    let after_while = self.new_block();
                    self.terminate(header_end, Term::Switch { cond, then: body_start, else_: after_while });
                    self.enter(after_while);
                }
                flat::Stmt::If(cond, then, else_) => {
                    let before_if = self.exit();

                    let then_start = self.new_block();
                    self.enter(then_start);
                    self.build_block(then);
                    let then_end = self.exit();

                    let else_start = self.new_block();
                    self.enter(else_start);
                    self.build_block(else_);
                    let else_end = self.exit();

                    self.terminate(before_if, Term::Switch { cond, then: then_start, else_: else_start });

                    let after_if = self.new_block();
                    self.terminate(then_end, Term::Goto(after_if));
                    self.terminate(else_end, Term::Goto(after_if));
                    self.enter(after_if);
                }
            }
        }
    }

    /// Pushes a statement onto the current block.
    fn push(&mut self, stmt: Stmt) {
        self.curr().push(stmt);
    }

    /// Returns a mutable reference to the current block's data.
    fn curr(&mut self) -> &mut BlockData {
        let curr = self.curr.expect("no current block");
        self.block(curr)
    }

    /// Returns a mutable reference to the requested block's data.
    fn block(&mut self, b: Block) -> &mut BlockData {
        &mut self.blocks[b]
    }

    /// Creates a new, empty, unterminated basic block with no predecessors.
    fn new_block(&mut self) -> Block {
        let b = self.blocks.insert(BlockData::empty());
        trace!("created {}", b);
        b
    }


    /// "Enters" a block (sets current block to `b`)
    fn enter(&mut self, b: Block) {
        assert!(self.curr.is_none());
        self.curr = Some(b);
        trace!("entered {}", b);
    }

    /// "Exits" the current block, returning what it was.
    fn exit(&mut self) -> Block {
        let b = self.curr.take().expect("exit called without entering first");
        trace!("exited {}", b);
        b
    }

    /// Terminates the current block and returns the set of blocks as a control flow graph.
    fn complete(self) -> Cfg {
        trace!("completing control flow graph");

        Cfg {
            blocks: self.blocks,
        }
    }
}

impl Function {
    pub fn new(f: raise::Func, function: flat::Function, is_main: bool) -> Self {
        let cfg = Cfg::new(function.body);
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
                write!(f, "@{}(", name)?;
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
                write!(f, "switch ")?;
                f.fmt(&self.formatted(&cond))?;
                write!(f, " [{}, {}]", then, else_)?;
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

        for (b, data) in &self.value.blocks {
            writeln!(f, "{}:", b)?;
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
                write!(f, "_{}", self.value.inner())
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

        write!(f, "fun {}(", self.value.name)?;
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

pub struct Liveness<'cfg> {
    cfg: &'cfg Cfg,

    /// Directly from Modern Compiler Implementation,
    /// using the aggregate optimization presented
    /// in Chapter 17 (basic blocks)
    uses: HashMap<Block, HashSet<Var>>,
    defs: HashMap<Block, HashSet<Var>>,
}

impl<'cfg> Liveness<'cfg> {
    pub fn new(cfg: &'cfg Cfg) -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use flatten;
    use super::*;
    use explicate::var;
    use flatten::Expr;

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

    #[test]
    fn uses() {
        let mut vars: var::Slab<var::Data> = var::Slab::new();
        let x = vars.insert(var::Data::Temp);
        let y = vars.insert(var::Data::Temp);
        let mut blocks = block::Blocks::new();
        let b0 = blocks.insert(block::Data::empty());
        let b1 = blocks.insert(block::Data::empty());

        let block = block::Data {
            body: vec![
                Stmt::Def { lhs: x, rhs: Expr::Copy(y) },
            ],
            term: Some(Term::Return(Some(x))),
            pred: hash_set!(),
        };

        assert_eq!(block.uses(), hash_set!(x, y));

        assert_eq!(Term::Switch { cond: x, then: b0, else_: b1 }.uses(), hash_set!(x));
        assert_eq!(Term::Goto(b0).uses(), hash_set!());
    }

    #[test]
    fn defs() {
        let mut vars: var::Slab<var::Data> = var::Slab::new();
        let x = vars.insert(var::Data::Temp);
        let y = vars.insert(var::Data::Temp);
        let z = vars.insert(var::Data::Temp);
        let mut blocks = block::Blocks::new();
        let b0 = blocks.insert(block::Data::empty());
        let b1 = blocks.insert(block::Data::empty());

        let block = block::Data {
            body: vec![
                Stmt::Def { lhs: x, rhs: Expr::Copy(y) },
                Stmt::Def { lhs: z, rhs: Expr::Copy(x) },
                Stmt::Discard(Expr::Copy(y)),
            ],
            term: Some(Term::Return(None)),
            pred: hash_set!(),
        };

        assert_eq!(block.defs(), hash_set!(x, z));
    }
}
