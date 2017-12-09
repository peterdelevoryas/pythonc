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
            let func_map = m.functions.iter()
                .map(|(&f, function)| {
                    let func = Func::new(f.inner(), function.name.clone());
                    (f, func)
                })
                .collect();
            let mut b = Builder::new(&m.var_data, func_map);
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
        func_map: HashMap<raise::Func, Func>,
    }

    impl<'var_data> Builder<'var_data> {
        fn new(var_data: &'var_data VarData, func_map: HashMap<raise::Func, Func>) -> Self {
            let vars = VarEnv::from(var_data);
            let funcs = HashMap::new();
            Builder {
                var_data,
                vars,
                funcs,
                func_map,
            }
        }

        fn visit_function(&mut self, f: raise::Func, function: cfg::Function, _is_main: bool) {
            let b = FuncBuilder::new(&mut self.vars, self.var_data, self.func_map.clone());
            let func_data = b.build(f, function);
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
        User { name: String },
    }

    impl Var {
        pub fn temp(index: usize) -> Self {
            Var {
                inner: Inner::Temp,
                index: index,
            }
        }

        pub fn user(index: usize, name: String) -> Self {
            Var {
                inner: Inner::User { name },
                index: index,
            }
        }
    }

    pub struct Env {
        next: usize,
    }

    impl Env {
        pub fn from(var_data: &var::Slab<var::Data>) -> Env {
            let next = var_data
                .iter()
                .map(|(v, _)| v.inner())
                .max()
                .map(|max| max + 1)
                .unwrap_or(0);
            Env { next }
        }
    }

    impl fmt::Display for Var {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.inner {
                Inner::Temp => write!(f, "_{}", self.index),
                Inner::User { ref name } => write!(f, "{}.{}", name, self.index),
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
    use vm::Inst;
    use vm::Term;
    use explicate::VarData;
    use explicate::var;
    use explicate as ex;
    use cfg;
    use cfg::Stmt;
    use flatten::Expr;
    use raise;
    use vm::Lval;
    use vm::Rval;
    use vm::Reg::*;
    use vm::InstData;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Func {
        index: usize,
        name: String,
    }

    pub struct Data {
        pub name: Func,
        pub args: Vec<Var>,
        pub blocks: HashMap<Block, BlockData>,
        pub stack: StackLayout,
    }

    impl Func {
        pub fn new(index: usize, name: String) -> Func {
            Func { index, name }
        }
    }

    pub struct Builder<'vars, 'var_data> {
        vars: &'vars mut VarEnv,
        var_data: &'var_data VarData,
        funcs: HashMap<raise::Func, Func>,
    }

    impl<'vars, 'var_data> Builder<'vars, 'var_data> {
        pub fn new(vars: &'vars mut VarEnv,
                   var_data: &'var_data VarData,
                   funcs: HashMap<raise::Func, Func>) -> Self
        {
            Builder { vars, var_data, funcs }
        }

        pub fn build(self, f: raise::Func, function: cfg::Function) -> Data {
            let name = Func {
                index: f.inner(),
                name: function.name,
            };
            let args: Vec<Var> = self.convert_vars(&function.args).collect();
            let mut stack = StackLayout::new();

            let mut ret = Data {
                name: name,
                args: args,
                stack: stack,
                blocks: HashMap::new(),
            };

            for (block, data) in &function.cfg.blocks {
                let name = Block::from(block);
                let body: Vec<Inst> = self.convert_stmts(&data.body).collect();
                let term = data.term.as_ref().map(|term| self.convert_term(term));
                let term = match data.term {
                    Some(ref term) => self.convert_term(term),
                    None => panic!("control flow graph block didn't have a terminator!"),
                };
                let pred = self.convert_blocks(&data.pred).collect();
                let block_data = BlockData {
                    name: name.clone(),
                    body,
                    term,
                    pred,
                };

                ret.blocks.insert(name, block_data);
            }

            ret
        }

        fn convert_block(&self, block: cfg::Block) -> Block {
            Block::from(block)
        }

        fn convert_blocks<'blocks, I>(&self, blocks: I) -> impl Iterator<Item = Block>
        where
            I: IntoIterator<Item = &'blocks cfg::Block>,
        {
            blocks.into_iter().map(|&b| Block::from(b))
        }

        fn convert_var(&self, var: ex::Var) -> Var {
            let index = var.inner();
            match self.var_data[var] {
                ex::var::Data::Temp => Var::temp(index),
                ex::var::Data::User { ref source_name } => Var::user(index, source_name.clone()),
            }
        }

        fn convert_vars<'v, I>(&'v self, vars: I) -> impl 'v + Iterator<Item = Var>
        where
            I: IntoIterator<Item = &'v ex::Var>,
            <I as IntoIterator>::IntoIter: 'v,
        {
            vars.into_iter().map(move |&var| self.convert_var(var))
        }

        fn convert_expr(&self, expr: &Expr) -> InstData {
            use vm::inst::Unary::*;
            use vm::inst::Binary::*;
            use flatten as flat;
            match *expr {
                Expr::CallFunc(var, ref args) => {
                    let var = self.convert_var(var);
                    let args = self.convert_vars(args)
                        .map(|v| Rval::Lval(Lval::Var(v)))
                        .collect();
                    Inst::call_indirect(Lval::Var(var), args)
                }
                Expr::RuntimeFunc(ref name, ref args) => {
                    let func = name.clone();
                    let args = self.convert_vars(args)
                        .map(|v| Rval::Lval(Lval::Var(v)))
                        .collect();
                    Inst::call(func, args)
                }
                Expr::UnaryOp(op, arg) => {
                    let arg = self.convert_var(arg);
                    let opcode = match op {
                        flat::UnaryOp::NEGATE => Neg,
                        flat::UnaryOp::NOT => Not,
                    };
                    Inst::unary(opcode, Rval::Lval(Lval::Var(arg)))
                }
                Expr::BinOp(op, left, right) => {
                    let left = Rval::Lval(Lval::Var(self.convert_var(left)));
                    let right = Rval::Lval(Lval::Var(self.convert_var(right)));
                    let opcode = match op {
                        flat::BinOp::ADD => Add,
                        flat::BinOp::EQ => Sete,
                        flat::BinOp::NOTEQ => Setne,
                    };
                    InstData::Binary { opcode, left, right }
                }
                Expr::GetTag(var) => {
                    let var = self.convert_var(var);
                    InstData::Binary {
                        opcode: And,
                        left: Rval::Lval(Lval::Var(var)),
                        right: Rval::Imm(ex::MASK),
                    }
                }
                Expr::ProjectTo(var, ty) => {
                    let arg = Rval::Lval(Lval::Var(self.convert_var(var)));
                    match ty {
                        ex::Ty::Int | ex::Ty::Bool => InstData::Binary {
                            opcode: Shr,
                            left: arg,
                            right: Rval::Imm(ex::SHIFT),
                        },
                        ex::Ty::Big => InstData::Binary {
                            opcode: And,
                            left: arg,
                            right: Rval::Imm(!ex::MASK),
                        },
                        _ => panic!("Cannot project {} to {}", var, ty),
                    }
                }
                Expr::InjectFrom(var, ty) => {
                    let arg = Rval::Lval(Lval::Var(self.convert_var(var)));
                    match ty {
                        ex::Ty::Int => {
                            InstData::ShiftLeftThenOr {
                                arg: arg,
                                shift: ex::SHIFT,
                                or: ex::INT_TAG,
                            }
                        }
                        ex::Ty::Bool => {
                            InstData::ShiftLeftThenOr {
                                arg: arg,
                                shift: ex::SHIFT,
                                or: ex::BOOL_TAG,
                            }
                        }
                        ex::Ty::Big => {
                            InstData::Binary {
                                opcode: Or,
                                left: arg,
                                right: Rval::Imm(ex::BIG_TAG),
                            }
                        }
                        _ => panic!("Cannot inject {} from {}", var, ty),
                    }
                }
                Expr::Const(i) => InstData::Unary { opcode: Mov, arg: Rval::Imm(i) },
                Expr::LoadFunctionPointer(f) => {
                    let func = self.convert_func_name(f);
                    InstData::MovFuncLabel { func }
                }
                Expr::Copy(var) => {
                    let var = self.convert_var(var);
                    InstData::Unary {
                        opcode: Mov,
                        arg: Rval::Lval(Lval::Var(var)),
                    }
                }
            }
        }

        fn convert_func_name(&self, name: ::raise::Func) -> Func {
            unimplemented!()
        }

        /// Returns None (if a non-side-effecting stmt) or
        /// the stmt converted into an instruction.
        fn convert_stmt(&self, stmt: &Stmt) -> Option<Inst> {
            let inst = match *stmt {
                Stmt::Def { lhs, ref rhs } => {
                    let dst = self.convert_var(lhs);
                    self.convert_expr(rhs).dst(Lval::Var(dst))
                }
                // Only add side-effecting discards
                Stmt::Discard(ref e @ Expr::CallFunc(_, _)) |
                Stmt::Discard(ref e @ Expr::RuntimeFunc(_, _)) => {
                    self.convert_expr(e).dst(Lval::Reg(EAX))
                }
                Stmt::Discard(Expr::UnaryOp(_, _)) |
                Stmt::Discard(Expr::BinOp(_, _, _)) |
                Stmt::Discard(Expr::GetTag(_)) |
                Stmt::Discard(Expr::ProjectTo(_, _)) |
                Stmt::Discard(Expr::InjectFrom(_, _)) |
                Stmt::Discard(Expr::Const(_)) |
                Stmt::Discard(Expr::LoadFunctionPointer(_)) |
                Stmt::Discard(Expr::Copy(_)) => return None,
            };

            Some(inst)
        }

        fn convert_stmts<'stmts, I>(&'stmts self, stmts: I) -> impl Iterator<Item = Inst> + 'stmts
        where
            I: IntoIterator<Item = &'stmts Stmt>,
            <I as IntoIterator>::IntoIter: 'stmts,
        {
            stmts.into_iter().filter_map(
                move |stmt| self.convert_stmt(stmt),
            )
        }

        fn convert_term(&self, term: &cfg::Term) -> Term {
            match *term {
                cfg::Term::Return(ref var) => {
                    let var = var.map(|var| self.convert_var(var));
                    Term::Return { var }
                }
                cfg::Term::Goto(block) => {
                    let block = Block::from(block);
                    Term::Goto { block }
                }
                cfg::Term::Switch { cond, then, else_ } => {
                    let cond = self.convert_var(cond);
                    let then = self.convert_block(then);
                    let else_ = self.convert_block(else_);
                    Term::Switch { cond, then, else_ }
                }
            }
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
pub use self::reg::Reg::*;

pub mod inst {
    use std::fmt;
    use vm::Reg;
    use vm::StackSlot;
    use vm::Var;
    use vm::Func;
    use cfg::Stmt;
    use flatten::Expr;

    pub struct Inst {
        pub dst: Lval,
        pub data: Data,
    }

    pub enum Unary {
        Mov,
        Neg,
        Not,
        Push,
        Pop,
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
        Unary { opcode: Unary, arg: Rval },
        Binary {
            opcode: Binary,
            left: Rval,
            right: Rval,
        },
        CallIndirect { target: Lval, args: Vec<Rval> },
        Call { func: String, args: Vec<Rval> },

        /// XXX Oof! This is unfortunately here for now,
        /// a product of InjectFrom requiring two binary
        /// instructions
        ShiftLeftThenOr {
            arg: Rval,
            shift: Imm,
            or: Imm,
        },

        /// XXX Another oof!
        MovFuncLabel {
            func: Func,
        }
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

    impl Inst {
        pub fn call_indirect(target: Lval, args: Vec<Rval>) -> Data {
            Data::CallIndirect { target, args }
        }

        pub fn call(func: String, args: Vec<Rval>) -> Data {
            Data::Call { func, args }
        }

        pub fn unary(opcode: Unary, arg: Rval) -> Data {
            Data::Unary { opcode, arg }
        }
    }

    impl Data {
        pub fn dst(self, dst: Lval) -> Inst {
            Inst {
                dst: dst,
                data: self,
            }
        }
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
pub use self::inst::Unary;
pub use self::inst::Binary;

pub mod stack {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Slot {
        index: usize,
    }

    pub enum Data {
        Param { index: usize },
        Spill { index: usize },
    }

    pub struct Layout {}

    impl Layout {
        pub fn new() -> Self {
            Layout {}
        }
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
        Return { var: Option<Var> },
        Goto { block: Block },
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
    T: fmt::Display,
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
            _ => unreachable!(),
        }
    }

    return indented;
}
