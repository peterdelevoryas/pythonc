use std::collections::HashMap;
use std::fmt;
use vm::Var;
use vm::VarEnv;
use vm::Block;
use vm::BlockData;
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
use vm::StackSlot;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Func {
    index: usize,
    name: String,
}

pub struct Data {
    pub name: Func,
    pub args: Vec<Var>,
    pub stack_slots: usize,
    pub blocks: HashMap<Block, BlockData>,
}

impl Func {
    pub fn new(index: usize, name: String) -> Func {
        Func { index, name }
    }
}

impl Data {
    pub fn root(&self) -> &BlockData {
        let mut root = None;
        for (_, block) in &self.blocks {
            if block.pred.is_empty() {
                assert!(root.is_none());
                root = Some(block);
            }
        }

        root.unwrap()
    }

    pub fn root_mut(&mut self) -> &mut BlockData {
        let mut root = None;
        for (_, block) in &mut self.blocks {
            if block.pred.is_empty() {
                assert!(root.is_none());
                root = Some(block)
            }
        }
        root.unwrap()
    }

    pub fn block(&self, block: &Block) -> &BlockData {
        &self.blocks[block]
    }

    pub fn block_mut(&mut self, block: &Block) -> &mut BlockData {
        self.blocks.get_mut(block).unwrap()
    }

    pub fn name(&self) -> &str {
        &self.name.name
    }

    pub fn allocate_registers(&mut self, env: &mut VarEnv) {
        loop {
            use vm::interference::DSaturResult::*;
            use vm::InterferenceGraph;
            let mut g = InterferenceGraph::build(self);
            match g.run_dsatur() {
                Success => {
                    g.assign_homes(self);
                    break;
                }
                Spill(u) => {
                    self.spill(u);
                    self.replace_stack_to_stack_ops(env);
                }
            }
        }
    }

    pub fn spill(&mut self, var: Var) {
        fn spill_rval(rval: &mut Rval, var: Var, slot: StackSlot) {
            match *rval {
                Rval::Imm(_) => {}
                Rval::Lval(ref mut lval) => spill_lval(lval, var, slot),
            }
        }

        fn spill_lval(lval: &mut Lval, var: Var, slot: StackSlot) {
            match *lval {
                Lval::StackSlot(_) | Lval::Reg(_) => return,
                Lval::Var(v) if v != var => return,
                Lval::Var(_) => {}
            }
            *lval = Lval::StackSlot(slot);
        }

        use vm::InstData::*;
        use vm::Term::*;
        let slot = StackSlot::Spill { index: self.stack_slots };
        for (_, block) in &mut self.blocks {
            for inst in &mut block.body {
                spill_lval(&mut inst.dst, var, slot);
                match inst.data {
                    Unary { ref mut arg, .. } => spill_rval(arg, var, slot),
                    Binary { ref mut left, ref mut right, .. } => {
                        spill_rval(left, var, slot);
                        spill_rval(right, var, slot);
                    }
                    CallIndirect { ref mut target, ref mut args } => {
                        spill_lval(target, var, slot);
                        for arg in args.iter_mut() {
                            spill_rval(arg, var, slot);
                        }
                    }
                    Call { ref mut args, .. } => {
                        for arg in args.iter_mut() {
                            spill_rval(arg, var, slot);
                        }
                    }
                    ShiftLeftThenOr { ref mut arg, .. } => {
                        spill_rval(arg, var, slot);
                    }
                    MovFuncLabel { .. } => {}
                }
            }
            match block.term {
                Return { ref mut rval } => {
                    if let Some(ref mut rval) = *rval {
                        spill_rval(rval, var, slot);
                    }
                }
                Goto { .. } => {}
                Switch { ref mut cond, .. } => {
                    spill_rval(cond, var, slot);
                }
            }
        }
        self.stack_slots += 1;
    }

    pub fn replace_stack_to_stack_ops(&mut self, env: &mut VarEnv) {
        use std::mem;
        use vm::Unary::Mov;

        for (_, block) in &mut self.blocks {
            let mut transformed = Vec::new();
            for inst in &block.body {
                if inst.is_stack_to_stack() {
                    let tmp = env.new_temp();
                    let mut inst = inst.clone();
                    let dst = mem::replace(&mut inst.dst, Lval::Var(tmp));
                    let store = Inst {
                        dst: dst,
                        data: InstData::Unary {
                            opcode: Mov,
                            arg: Rval::Lval(Lval::Var(tmp)),
                        },
                    };
                    transformed.push(inst);
                    transformed.push(store);
                } else {
                    transformed.push(inst.clone());
                }
            }
            block.body = transformed;

            // XXX Term doesn't need to be modified...I think...
        }
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

        let mut ret = Data {
            name: name,
            args: args,
            stack_slots: 0,
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
        Var::from(var)
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
        self.funcs[&name].clone()
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
                let rval = var.map(|var| Rval::Lval(Lval::Var(var)));
                Term::Return { rval }
            }
            cfg::Term::Goto(block) => {
                let block = Block::from(block);
                Term::Goto { block }
            }
            cfg::Term::Switch { cond, then, else_ } => {
                let cond = self.convert_var(cond);
                let cond = Rval::Lval(Lval::Var(cond));
                let then = self.convert_block(then);
                let else_ = self.convert_block(else_);
                Term::Switch { cond, then, else_ }
            }
        }
    }
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
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

