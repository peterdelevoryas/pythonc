use ssa::Block;
use ssa::BlockData;
use ssa::BlockGen;
use ssa::Val;
use ssa::ValGen;
use ssa::Expr;
use ssa::Inst;
use ssa::Rval;
use ssa::Unary;
use ssa::Binary;
use ssa::Term;
use std::collections::HashMap;
use std::collections::HashSet;
use explicate::Var;
use raise::Func as FlatFunc;
use flatten::Function as FlatFunction;
use flatten::Stmt as FlatStmt;
use flatten::Expr as FlatExpr;
use explicate as ex;

impl_ref!(Func, "fn");

pub struct FuncData {
    pub main: bool,
    pub root: Block,
    pub args: Vec<Val>,
    pub block_gen: BlockGen,
    pub blocks: HashMap<Block, BlockData>,
    pub sealed_blocks: HashSet<Block>,
    pub defs: HashMap<Block, HashMap<Var, Val>>,
    pub vals: HashMap<Val, Expr>,
    pub val_gen: ValGen,
    pub incomplete_phis: HashMap<Block, HashMap<Var, Val>>,
}

pub type FuncGen = Gen;

impl FuncData {
    pub fn new(args: &[Var], is_main: bool) -> FuncData {
        let mut block_gen = BlockGen::new();
        let mut blocks = map!();
        let mut defs = map!();
        let mut vals = map!();
        let mut val_gen = ValGen::new();
        let mut incomplete_phis = map!();

        let root = block_gen.gen();
        let root_data = BlockData::new();
        blocks.insert(root, root_data);
        defs.insert(root, map!());
        incomplete_phis.insert(root, map!());

        let mut arg_vals = vec![];
        for (position, &arg) in args.iter().enumerate() {
            let val = val_gen.gen();
            vals.insert(val, Expr::LoadParam { position });
            defs.get_mut(&root)
                .unwrap()
                .insert(arg, val);
            arg_vals.push(val);
        }

        FuncData {
            main: is_main,
            root: root,
            args: arg_vals,
            block_gen: block_gen,
            blocks: blocks,
            sealed_blocks: set!(root),
            defs: defs,
            vals: vals,
            val_gen: val_gen,
            incomplete_phis: incomplete_phis,
        }
    }
}

pub struct Builder<'flat_func_map, 'func_data> {
    pub flat_func_map: &'flat_func_map HashMap<FlatFunc, Func>,
    pub func_data: &'func_data mut FuncData,
    pub curr: Block,
}

impl<'flat_func_map, 'func_data> Builder<'flat_func_map, 'func_data> {
    pub fn new(flat_func_map: &'flat_func_map HashMap<FlatFunc, Func>, func_data: &'func_data mut FuncData) -> Self
    {
        let curr = func_data.root;
        Builder { flat_func_map, func_data, curr }
    }

    /// If true, we can stop
    pub fn visit_stmt(&mut self, flat_stmt: &FlatStmt) -> bool {
        use self::FlatStmt::*;
        match *flat_stmt {
            Def(var, ref flat_expr) => {
                let val = self.def_expr(flat_expr);
                self.write_curr(var, val);
            }
            Discard(ref flat_expr) => {
                let _ = self.def_expr(flat_expr);
                // don't need to write definition, like in above Def case,
                // because there a var def to write.
            }
            Return(ref var) => {
                let curr = self.curr;
                let ret = var.as_ref().map(|&var| Rval::Val(self.read_curr(var)));
                self.term_block(curr, Term::Ret { ret });
                return true;
            }
            While(cond, ref header, ref body) => {
                let while_entry = self.curr;
                self.seal_block(while_entry);

                let header_start = self.create_block();
                self.term_block(while_entry, Term::Goto { block: header_start });
                self.enter(header_start);

                let header_end = self.fill_curr(header);

                let cond = Rval::Val(self.read_curr(cond));
                if header_end != header_start {
                    self.seal_block(header_end);
                }

                let body_start = self.create_block();
                let after_while = self.create_block();
                self.term_block(header_end, Term::Switch { cond, then: body_start, else_: after_while });

                self.enter(body_start);
                let body_end = self.fill_curr(body);
                self.seal_block(body_start);
                self.term_block(body_end, Term::Goto { block: header_start });
                if body_end != body_start {
                    self.seal_block(body_end);
                }

                self.enter(header_start);
                self.seal_block(header_start);

                self.enter(after_while);
            }
            If(cond, ref then, ref else_) => {
                let if_entry = self.curr;
                let cond = Rval::Val(self.read_curr(cond));
                self.seal_block(if_entry);

                let then_start = self.create_block();
                self.enter(then_start);
                let then_end = self.fill_curr(then);
                self.seal_block(then_start);
                if then_start != then_end {
                    self.seal_block(then_end);
                }

                let else_start = self.create_block();
                self.enter(else_start);
                let else_end = self.fill_curr(else_);
                self.seal_block(else_start);
                if else_start != else_end {
                    self.seal_block(else_end);
                }

                self.term_block(if_entry, Term::Switch { cond, then: then_start, else_: else_start });
                let if_exit = self.create_block();
                self.term_block(then_end, Term::Goto { block: if_exit });
                self.term_block(else_end, Term::Goto { block: if_exit });
                self.enter(if_exit);
            }
        }

        false
    }

    pub fn fill_curr(&mut self, flat_stmts: &[FlatStmt]) -> Block {
        for flat_stmt in flat_stmts {
            if self.visit_stmt(flat_stmt) {
                break
            }
        }
        let curr = self.curr;
        curr
    }

    pub fn enter(&mut self, block: Block) -> Block {
        ::std::mem::replace(&mut self.curr, block)
    }

    pub fn term_block(&mut self, block: Block, term: Term) {
        assert!(self.func_data.blocks.get(&block).unwrap().term.is_none());
        self.func_data.blocks.get_mut(&block).unwrap().term = Some(term);
        for succ in self.func_data.blocks[&block].successors() {
            self.func_data.blocks.get_mut(&succ).unwrap().preds.insert(block);
        }
    }

    pub fn def_expr(&mut self, flat_expr: &FlatExpr) -> Val {
        use self::FlatExpr::*;
        match *flat_expr {
            UnaryOp(op, var) => {
                let opcode = op.into();
                let val = self.read_curr(var);
                let arg = Rval::Val(val);
                self.push_def(Expr::Unary { opcode, arg })
            }
            BinOp(op, l, r) => {
                let opcode = op.into();
                let left = Rval::Val(self.read_curr(l));
                let right = Rval::Val(self.read_curr(r));
                self.push_def(Expr::Binary { opcode, left, right })
            }
            CallFunc(target, ref args) => {
                let target = self.read_curr(target);
                let args = args.iter().map(|&arg| Rval::Val(self.read_curr(arg))).collect();
                self.push_def(Expr::CallIndirect { target, args })
            }
            RuntimeFunc(ref name, ref args) => {
                let func = name.clone();
                let args = args.iter().map(|&arg| Rval::Val(self.read_curr(arg))).collect();
                self.push_def(Expr::Call { func, args })
            }
            GetTag(var) => {
                let val = self.read_curr(var);
                self.push_def(Expr::Binary {
                    opcode: Binary::And,
                    left: Rval::Val(val),
                    right: Rval::Imm(ex::MASK),
                })
            }
            ProjectTo(var, ty) => {
                let arg = Rval::Val(self.read_curr(var));
                let expr = match ty {
                    ex::Ty::Int | ex::Ty::Bool => Expr::Binary {
                        opcode: Binary::Shr,
                        left: arg,
                        right: Rval::Imm(ex::SHIFT),
                    },
                    ex::Ty::Big => Expr::Binary {
                        opcode: Binary::And,
                        left: arg,
                        right: Rval::Imm(!ex::MASK),
                    },
                    _ => panic!("Cannot project {} to {}", var, ty)
                };
                self.push_def(expr)
            }
            InjectFrom(var, ty) => {
                let arg = Rval::Val(self.read_curr(var));
                let expr = match ty {
                    ex::Ty::Int => {
                        Expr::ShiftLeftThenOr {
                            arg: arg,
                            shift: ex::SHIFT,
                            or: ex::INT_TAG,
                        }
                    }
                    ex::Ty::Bool => {
                        Expr::ShiftLeftThenOr {
                            arg: arg,
                            shift: ex::SHIFT,
                            or: ex::BOOL_TAG,
                        }
                    }
                    ex::Ty::Big => {
                        Expr::Binary {
                            opcode: Binary::Or,
                            left: arg,
                            right: Rval::Imm(ex::BIG_TAG),
                        }
                    }
                    _ => panic!("Cannot inject {} from {}", var, ty),
                };
                self.push_def(expr)
            }
            Const(i) => {
                self.push_def(Expr::Unary { opcode: Unary::Mov, arg: Rval::Imm(i) })
            }
            LoadFunctionPointer(flat_func) => {
                let func = match self.flat_func_map.get(&flat_func) {
                    Some(&func) => func,
                    None => panic!("no flat func map entry for {}", flat_func)
                };
                self.push_def(Expr::MovFuncLabel { func })
            }
            Copy(var) => {
                let val = Rval::Val(self.read_curr(var));
                self.push_def(Expr::Unary { opcode: Unary::Mov, arg: val })
            }
        }
    }

    pub fn create_block(&mut self) -> Block {
        let block = self.func_data.block_gen.gen();
        let block_data = BlockData::new();
        self.func_data.blocks.insert(block, block_data);
        self.func_data.defs.insert(block, map!());

        block
    }

    pub fn push_def(&mut self, expr: Expr) -> Val {
        let val = self.gen_val();
        assert!(self.func_data.vals.insert(val, expr).is_none());
        let block = self.curr;
        self.func_data.blocks.get_mut(&block).unwrap().body.push(val);
        val
    }

    pub fn read_curr(&mut self, var: Var) -> Val {
        let block = self.curr;
        self.read(block, var)
    }

    pub fn write_curr(&mut self, var: Var, val: Val) {
        let block = self.curr;
        self.write(block, var, val);
    }

    pub fn write(&mut self, block: Block, var: Var, val: Val) {
        self.func_data.defs.get_mut(&block).unwrap().insert(var, val);
    }

    pub fn read(&mut self, block: Block, var: Var) -> Val {
        if self.func_data.defs[&block].contains_key(&var) {
            return self.func_data.defs[&block][&var]
        }
        self.read_recursive(block, var)
    }

    pub fn read_recursive(&mut self, block: Block, var: Var) -> Val {
        let val;
        if !self.func_data.sealed_blocks.contains(&block) {
            println!("creating phi for {}", block);
            val = self.new_phi(block);
            self.func_data.incomplete_phis.entry(block).or_insert(map!()).insert(var, val);
        } else if self.func_data.blocks[&block].preds.len() == 1 {
            let pred = self.func_data.blocks[&block].preds_iter().nth(0).unwrap();
            val = self.read(pred, var);
        } else {
            println!("creating phi for {}", block);
            let phi = self.new_phi(block);
            self.write(block, var, phi);
            val = self.add_phi_operands(var, phi);
        }
        self.write(block, var, val);
        val
    }

    pub fn add_phi_operands(&mut self, var: Var, phi: Val) -> Val {
        let block = match self.func_data.vals.get_mut(&phi) {
            Some(&mut Expr::Phi { block, .. }) => block,
            _ => panic!("expected phi expr")
        };
        for pred in self.func_data.blocks[&block].preds_iter() {
            let val = self.read(pred, var);
            self.append_phi_operand(phi, val);
        }
        // XXX Implement this optimization!
        //self.try_remove_trivial_phi(phi)
        phi
    }

    pub fn append_phi_operand(&mut self, phi: Val, operand: Val) {
        match self.func_data.vals.get_mut(&phi) {
            Some(&mut Expr::Phi { ref mut vals, .. }) => {
                vals.push(operand);
            }
            _ => panic!("expected phi expr")
        }
    }

    pub fn get_phi_operands(&self, phi: Val) -> impl Iterator<Item=Val> {
        match self.func_data.vals.get(&phi) {
            Some(&Expr::Phi { ref vals, .. }) => vals.clone().into_iter(),
            _ => panic!("expected phi expr"),
        }
    }

    pub fn try_remove_trivial_phi(&mut self, phi: Val) -> Val {
        let mut same = None;
        for op in self.get_phi_operands(phi) {
            if let Some(same) = same {
                if same == op {
                    continue
                }
            }
            if op == phi {
                continue
            }

            if same != None {
                return phi
            }

            same = Some(op);
        }

        let same = match same {
            Some(same) => same,
            None => {
                let val = self.gen_val();
                self.func_data.vals.insert(val, Expr::Undef);
                val
            }
        };
        unimplemented!()
    }

    pub fn gen_val(&mut self) -> Val {
        self.func_data.val_gen.gen()
    }

    pub fn new_phi(&mut self, block: Block) -> Val {
        let val = self.gen_val();
        assert!(self.func_data.vals.insert(val, Expr::Phi { block, vals: vec![] }).is_none());
        self.func_data.blocks.get_mut(&block).unwrap().body.insert(0, val);
        val
    }

    pub fn seal_block(&mut self, block: Block) {
        for (var, val) in self.func_data.incomplete_phis.entry(block).or_insert(map!()).clone() {
            self.add_phi_operands(var, val);
        }
        if self.func_data.sealed_blocks.insert(block) {
            println!("WARNING {} already sealed", block)
        }
    }

    pub fn complete(mut self) {
    }
}
