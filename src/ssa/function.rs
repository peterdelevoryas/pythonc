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
use ssa::CallTarget;
use std::collections::HashMap;
use std::collections::HashSet;
use explicate::Var;
use raise::Func as FlatFunc;
use flatten::Function as FlatFunction;
use flatten::Stmt as FlatStmt;
use flatten::Expr as FlatExpr;
use explicate as ex;

impl_ref!(Func, "f");

pub struct FuncData {
    /// Is this the main function?
    pub main: bool,
    /// Root block identifier (no predecessors, start of program)
    pub root: Option<Block>,
    /// List of Val's that are arguments to the function.
    /// `vals` will contain <Val> => LoadParam { position } for each of these.
    pub args: Vec<Val>,
    pub block_gen: BlockGen,
    /// Set of function blocks
    pub blocks: HashMap<Block, BlockData>,
    /// Blocks who will not have any further predecessors added to it
    pub sealed_blocks: HashSet<Block>,
    /// Records of Var definitions (per block)
    pub defs: HashMap<Block, HashMap<Var, Rval>>,
    /// Non-constant expr's, mapped to a SSA Val
    pub vals: HashMap<Val, Expr>,
    pub val_gen: ValGen,
    /// The set of phi calls that need to be added to
    /// a block. XXX Can probably be merged into Defs
    pub incomplete_phis: HashMap<Block, HashMap<Var, Val>>,
}

pub type FuncGen = Gen;

impl FuncData {
    pub fn new() -> FuncData {
        FuncData {
            main: false,
            root: None,
            args: vec![],
            block_gen: BlockGen::new(),
            blocks: map!(),
            sealed_blocks: set!(),
            defs: map!(),
            vals: map!(),
            val_gen: ValGen::new(),
            incomplete_phis: map!(),
        }
    }
    /*
        let root = block_gen.gen();
        let root_data = BlockData::new();
        blocks.insert(root, root_data);
        defs.insert(root, map!());
        incomplete_phis.insert(root, map!());

        let arg_vals = if !is_main {
            let mut arg_vals = vec![];
            for (position, &arg) in args.iter().enumerate() {
                let val = val_gen.gen();
                vals.insert(val, Expr::LoadParam { position });
                defs.get_mut(&root)
                    .unwrap()
                    .insert(arg, Rval::Val(val));
                arg_vals.push(val);
                blocks.get_mut(&root).unwrap()
                    .body.push(val);
            }
            arg_vals
        } else {
            assert!(args.len() == 1);
            vec![]
        };


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
    */
}

/// XXX This is a terrible design for a builder. I wanted to try something
/// a little different, and I thought constructing an initial FuncData and
/// then modifying it iteratively would reduce code duplication, but it
/// just increases it or makes it more difficult to do things properly,
/// and to reason about the state.
pub struct Builder<'flat_func_map, 'func_data> {
    pub flat_func_map: &'flat_func_map HashMap<FlatFunc, Func>,
    pub func_data: &'func_data mut FuncData,
    pub curr: Option<Block>,
}

impl<'flat_func_map, 'func_data> Builder<'flat_func_map, 'func_data> {
    pub fn new(flat_func_map: &'flat_func_map HashMap<FlatFunc, Func>,
               func_data: &'func_data mut FuncData) -> Self
    {
        Builder { flat_func_map, func_data, curr: None }
    }

    pub fn curr_block(&self) -> Block {
        self.curr.unwrap()
    }

    pub fn switch_to_block(&mut self, block: Block) {
        self.curr = Some(block);
    }

    pub fn block(&self, block: Block) -> &BlockData {
        &self.func_data.blocks[&block]
    }

    pub fn block_mut(&mut self, block: Block) -> &mut BlockData {
        self.func_data.blocks.get_mut(&block).unwrap()
    }

    pub fn root(&self) -> &BlockData {
        self.block(self.func_data.root.unwrap())
    }

    pub fn set_root(&mut self, root: Block) {
        assert!(self.is_sealed(root));
        assert!(self.predecessors(root).is_empty());
        assert!(self.func_data.root.is_none());
        self.func_data.root = Some(root);
    }

    // basically an alias for write(curr, var, val)
    pub fn def_var(&mut self, var: Var, val: Val) {
        let curr = self.curr_block();
        self.write(curr, var, val);
    }

    /// Returns true if block is terminated by a return (and we can
    /// skip remaining stmts in the block), returns false otherwise
    /// (and caller needs to terminated the block)
    pub fn visit_stmt(&mut self, flat_stmt: &FlatStmt) -> bool {
        use self::FlatStmt::*;
        match *flat_stmt {
            Def(var, ref flat_expr) => {
                let rval = self.def_expr(flat_expr);
                self.write_curr(var, rval);
            }
            Discard(ref flat_expr) => {
                let _ = self.def_expr(flat_expr);
                // don't need to write definition, like in above Def case,
                // because there a var def to write.
            }
            Return(ref var) => {
                let curr = self.curr_block();
                let ret = var.as_ref().map(|&var| self.read_curr(var));
                self.term_block(curr, Term::Ret { ret });
                return true;
            }
            While(cond, ref header, ref body) => {
                let while_entry = self.curr_block();
                self.seal_block(while_entry);

                let header_start = self.create_block();
                self.term_block(while_entry, Term::Goto { block: header_start });
                self.switch_to_block(header_start);

                let header_end = self.fill_curr(header);

                let cond = self.read_curr(cond);
                if header_end != header_start {
                    self.seal_block(header_end);
                }

                let body_start = self.create_block();
                let after_while = self.create_block();
                self.term_block(header_end, Term::Switch { cond, then: body_start, else_: after_while });

                self.switch_to_block(body_start);
                let body_end = self.fill_curr(body);
                self.seal_block(body_start);
                self.term_block(body_end, Term::Goto { block: header_start });
                if body_end != body_start {
                    self.seal_block(body_end);
                }

                self.switch_to_block(header_start);
                self.seal_block(header_start);

                self.switch_to_block(after_while);
            }
            If(cond, ref then, ref else_) => {
                let if_entry = self.curr_block();
                let cond = self.read_curr(cond);
                self.seal_block(if_entry);

                let then_start = self.create_block();
                let else_start = self.create_block();
                self.term_block(if_entry, Term::Switch { cond, then: then_start, else_: else_start });

                self.switch_to_block(then_start);
                let then_end = self.fill_curr(then);
                self.seal_block(then_start);
                if then_start != then_end {
                    self.seal_block(then_end);
                }

                self.switch_to_block(else_start);
                let else_end = self.fill_curr(else_);
                self.seal_block(else_start);
                if else_start != else_end {
                    self.seal_block(else_end);
                }

                let if_exit = self.create_block();
                self.term_block(then_end, Term::Goto { block: if_exit });
                self.term_block(else_end, Term::Goto { block: if_exit });
                self.switch_to_block(if_exit);
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
        let curr = self.curr_block();
        curr
    }

    pub fn term_block(&mut self, block: Block, term: Term) {
        assert!(self.func_data.blocks.get(&block).unwrap().term.is_none());
        self.func_data.blocks.get_mut(&block).unwrap().term = Some(term);
        for succ in self.func_data.blocks[&block].successors() {
            self.func_data.blocks.get_mut(&succ).unwrap().preds.insert(block);
        }
    }

    pub fn unary(&mut self, opcode: Unary, arg: Rval) -> Rval {
        match arg {
            Rval::Val(val) => Rval::Val(self.push_def(Expr::Unary { opcode, arg })),
            Rval::Imm(imm) => {
                match opcode {
                    Unary::Mov => Rval::Imm(imm),
                    Unary::Neg => Rval::Imm(-imm),
                    Unary::Not => Rval::Imm(!imm),
                }
            }
            Rval::Func(_) => panic!("Encounered Func in unary instruction: {} {}", opcode, arg)
        }
    }

    pub fn add_func_param(&mut self, param: Var) -> Val {
        let val = self.gen_val();
        self.func_data.args.push(val);
        val
    }

    pub fn load_param(&mut self, param: Val) -> Val {
        let position = match self.func_data.args.iter().position(|&a| a == param) {
            Some(position) => position,
            None => panic!("val is not a param: {}", param),
        };
        self.push_def(Expr::LoadParam { position })
    }

    pub fn binary(&mut self, opcode: Binary, left: Rval, right: Rval) -> Rval {
        use self::Rval::*;
        use self::Binary::*;
        match (left, right) {
            (Imm(left), Imm(right)) => {
                match opcode {
                    Add => Imm(left + right),
                    And => Imm(left & right),
                    Or => Imm(left | right),
                    Sete => Imm(if left == right { 1 } else { 0 }),
                    Setne => Imm(if left != right { 1 } else { 0 }),
                    Shr => Imm(left >> right),
                    Shl => Imm(left << right),
                }
            }
            (Func(_), _) |
            (_, Func(_)) => panic!("Encountered Func in binary instruction: {} {} {}", opcode, left, right),
            _ => {
                let val = self.push_def(Expr::Binary { opcode, left, right });
                Val(val)
            }
        }
    }

    pub fn call(&mut self, target: CallTarget, args: Vec<Rval>) -> Rval {
        let ret = self.push_def(Expr::Call { target, args });
        Rval::Val(ret)
    }

    pub fn get_tag(&mut self, rval: Rval) -> Rval {
        match rval {
            Rval::Imm(imm) => Rval::Imm(imm & ex::MASK),
            Rval::Val(val) => {
                let val = self.push_def(Expr::Binary { opcode: Binary::And, left: rval, right: Rval::Imm(ex::MASK) });
                Rval::Val(val)
            }
            Rval::Func(_) => panic!("Encountered Func in get_tag instruction: {}", rval),
        }
    }

    pub fn project_to(&mut self, rval: Rval, ty: ex::Ty) -> Rval {
        match rval {
            Rval::Imm(imm) => {
                match ty {
                    ex::Ty::Int | ex::Ty::Bool => Rval::Imm(imm >> ex::SHIFT),
                    ex::Ty::Big => Rval::Imm(imm & (!ex::MASK)),
                    _ => panic!("Cannot project {} to {}", rval, ty)
                }
            }
            Rval::Val(val) => {
                let expr = match ty {
                    ex::Ty::Int | ex::Ty::Bool => Expr::Binary {
                        opcode: Binary::Shr,
                        left: rval,
                        right: Rval::Imm(ex::SHIFT),
                    },
                    ex::Ty::Big => Expr::Binary {
                        opcode: Binary::And,
                        left: rval,
                        right: Rval::Imm(!ex::MASK),
                    },
                    _ => panic!("Cannot project {} to {}", rval, ty)
                };
                let val = self.push_def(expr);
                Rval::Val(val)
            }
            Rval::Func(_) => panic!("Encountered Func in ProjectTo instruction: {}", rval),
        }
    }

    pub fn inject_from(&mut self, rval: Rval, ty: ex::Ty) -> Rval {
        match rval {
            Rval::Imm(imm) => {
                match ty {
                    ex::Ty::Int => Rval::Imm((imm << ex::SHIFT) | ex::INT_TAG),
                    ex::Ty::Bool => Rval::Imm((imm << ex::SHIFT) | ex::BOOL_TAG),
                    ex::Ty::Big => Rval::Imm(imm | ex::BIG_TAG),
                    _ => panic!("Cannot inject {} from {}", rval, ty)
                }
            }
            arg @ Rval::Val(_) => {
                let expr = match ty {
                    ex::Ty::Int => Expr::ShiftLeftThenOr { arg, shift: ex::SHIFT, or: ex::INT_TAG },
                    ex::Ty::Bool => Expr::ShiftLeftThenOr { arg, shift: ex::SHIFT, or: ex::BOOL_TAG },
                    ex::Ty::Big => Expr::Binary { opcode: Binary::Or, left: arg, right: Rval::Imm(ex::BIG_TAG) },
                    _ => panic!("Cannot inject {} from {}", rval, ty)
                };
                let val = self.push_def(expr);
                Rval::Val(val)
            }
            Rval::Func(_) => panic!("Encountered Func in InjectFrom instruction: {}", rval),
        }
    }

    /// Takes a FlatExpr and returns the constant
    /// that it evalutes to, or (if non-constant) creates
    /// a new ssa Val, writes it to func_data.vals,
    /// 
    pub fn def_expr(&mut self, flat_expr: &FlatExpr) -> Rval {
        use self::FlatExpr::*;
        match *flat_expr {
            UnaryOp(op, var) => {
                let opcode = op.into();
                let arg = self.read_curr(var);
                self.unary(opcode, arg)
            }
            BinOp(op, left, right) => {
                let opcode = op.into();
                let left = self.read_curr(left);
                let right = self.read_curr(right);
                self.binary(opcode, left, right)
            }
            CallFunc(target, ref args) => {
                let rval = self.read_curr(target);
                let func = match rval {
                    Rval::Func(func) => func,
                    rval => panic!("Received indirect function call: {}", rval)
                };
                let target = CallTarget::Direct { func };
                let args = args.iter().map(|&arg| self.read_curr(arg)).collect();
                self.call(target, args)
            }
            LoadFunctionPointer(flat_func) => {
                let func = match self.flat_func_map.get(&flat_func) {
                    Some(&func) => func,
                    None => panic!("no flat func map entry for {}", flat_func)
                };
                Rval::Func(func)
            }
            RuntimeFunc(ref name, ref args) => {
                let func_name = match name.as_str() {
                    "is_true" => "is_true",
                    "input_int" => "input_int",
                    "add" => "add",
                    "equal" => "equal",
                    "not_equal" => "not_equal",
                    "set_subscript" => "set_subscript",
                    "print_any" => "print_any",
                    "get_subscript" => "get_subscript",
                    "create_list" => "create_list",
                    "create_dict" => "create_dict",
                    _ => panic!("Encountered unknown runtime function {}", name)
                };
                let target = CallTarget::Runtime { func_name };
                let args = args.iter().map(|&arg| self.read_curr(arg)).collect();
                self.call(target, args)
            }
            GetTag(var) => {
                let rval = self.read_curr(var);
                self.get_tag(rval)
            }
            ProjectTo(var, ty) => {
                let rval = self.read_curr(var);
                self.project_to(rval, ty)
            }
            InjectFrom(var, ty) => {
                let rval = self.read_curr(var);
                self.inject_from(rval, ty)
            }
            Const(i) => {
                Rval::Imm(i)
            }
            Copy(var) => {
                self.read_curr(var)
            }
        }
    }

    pub fn create_block(&mut self) -> Block {
        let block = self.func_data.block_gen.gen();
        let block_data = BlockData::new();
        self.func_data.blocks.insert(block, block_data);
        self.func_data.defs.insert(block, map!());
        self.func_data.incomplete_phis.insert(block, map!());
        block
    }

    pub fn push_def(&mut self, expr: Expr) -> Val {
        let val = self.gen_val();
        assert!(self.func_data.vals.insert(val, expr).is_none());
        let block = self.curr_block();
        self.block_mut(block).body.push(val);
        val
    }

    pub fn read_curr(&mut self, var: Var) -> Rval {
        let block = self.curr_block();
        self.read(block, var)
    }

    pub fn write_curr(&mut self, var: Var, rval: Rval) {
        let block = self.curr_block();
        self.write(block, var, rval);
    }

    pub fn write(&mut self, block: Block, var: Var, rval: Rval) {
        self.func_data.defs.get_mut(&block).unwrap().insert(var, rval);
    }

    pub fn read(&mut self, block: Block, var: Var) -> Rval {
        if self.func_data.defs[&block].contains_key(&var) {
            return self.func_data.defs[&block][&var]
        }
        self.read_recursive(block, var)
    }

    pub fn read_recursive(&mut self, block: Block, var: Var) -> Rval {
        let rval;
        if !self.func_data.sealed_blocks.contains(&block) {
            let phi = self.new_phi(block);
            rval = Rval::Val(phi);
            self.func_data.incomplete_phis.entry(block).or_insert(map!()).insert(var, phi);
        } else if self.func_data.blocks[&block].preds.len() == 1 {
            let pred = self.func_data.blocks[&block].preds_iter().nth(0).unwrap();
            rval = self.read(pred, var);
        } else {
            let phi = self.new_phi(block);
            self.write(block, var, Rval::Val(phi));
            rval = Rval::Val(self.add_phi_operands(var, phi));
        }
        self.write(block, var, rval);
        rval
    }

    pub fn add_phi_operands(&mut self, var: Var, phi: Val) -> Val {
        let block = match self.func_data.vals.get_mut(&phi) {
            Some(&mut Expr::Phi { block, .. }) => block,
            _ => panic!("expected phi expr")
        };
        for pred in self.func_data.blocks[&block].preds_iter() {
            let rval = self.read(pred, var);
            self.append_phi_operand(phi, rval);
        }
        // XXX Implement this optimization!
        //self.try_remove_trivial_phi(phi)
        phi
    }

    pub fn append_phi_operand(&mut self, phi: Val, operand: Rval) {
        match self.func_data.vals.get_mut(&phi) {
            Some(&mut Expr::Phi { ref mut args, .. }) => {
                args.push(operand);
            }
            _ => panic!("expected phi expr")
        }
    }

    pub fn get_phi_operands(&self, phi: Val) -> impl Iterator<Item=Rval> {
        match self.func_data.vals.get(&phi) {
            Some(&Expr::Phi { ref args, .. }) => args.clone().into_iter(),
            _ => panic!("expected phi expr"),
        }
    }

    pub fn try_remove_trivial_phi(&mut self, phi: Val) -> Rval {
        let mut same = None;
        for op in self.get_phi_operands(phi) {
            if let Some(same) = same {
                if same == op {
                    continue
                }
            }
            if op == Rval::Val(phi) {
                continue
            }

            if same != None {
                return Rval::Val(phi)
            }

            same = Some(op);
        }

        let same = match same {
            Some(same) => {
                self.func_data.vals.insert(phi, Expr::Unary { opcode: Unary::Mov, arg: same });
                same
            }
            None => {
                let val = self.gen_val();
                self.func_data.vals.insert(val, Expr::Undef);
                Rval::Val(val)
            }
        };
        same
    }

    pub fn gen_val(&mut self) -> Val {
        self.func_data.val_gen.gen()
    }

    pub fn new_phi(&mut self, block: Block) -> Val {
        let val = self.gen_val();
        assert!(self.func_data.vals.insert(val, Expr::Phi { block, args: vec![] }).is_none());
        self.func_data.blocks.get_mut(&block).unwrap().body.insert(0, val);
        val
    }

    pub fn is_sealed(&self, block: Block) -> bool {
        self.func_data.sealed_blocks.contains(&block)
    }

    pub fn seal_block(&mut self, block: Block) {
        assert!(
            !self.predecessors(block).is_empty(),
            "There should always be at least one predecessor for a block that's being sealed!"
        );
        for pred in self.func_data.blocks[&block].preds_iter() {
            assert!(
                self.is_sealed(pred),
                "Predecessors must be sealed before sealing successor!!"
            );
        }
        for (var, val) in self.func_data.incomplete_phis.entry(block).or_insert(map!()).clone() {
            self.add_phi_operands(var, val);
        }
        self.func_data.sealed_blocks.insert(block);
    }

    pub fn predecessors(&self, block: Block) -> &HashSet<Block> {
        &self.block(block).preds
    }

    pub fn complete(mut self) {
    }
}
