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
        let stack = StackLayout::new();

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

