use std::collections::HashMap;
use std::collections::HashSet;
use ssa::ProgramBuilder;
use ssa::Value;
use ssa::ValueMap;
use ssa::Expr;
use ssa::BlockMap;
use ssa::BlockData;
use ssa::Block;
use ssa::Phi;
use ssa::Branch;
use ssa::Unary;
use ssa::Binary;
use ssa::CallTarget;
use explicate::Var;
use std::mem;
use flatten::Expr as FlatExpr;
use explicate as ex;

impl_ref!(Function, "f");
pub type FunctionGen = Gen;
pub type FunctionMap<T> = Slab<T>;

pub struct FunctionData {
    pub is_main: bool,
    pub params: Vec<Value>,
    pub values: ValueMap<Expr>,
    pub defs: HashMap<Block, HashMap<Var, Value>>,
    pub blocks: BlockMap<BlockData>,
}

pub struct Builder<'a> {
    program: &'a mut ProgramBuilder,
    values: ValueMap<Expr>,
    params: Vec<Value>,
    is_main: bool,
    defs: HashMap<Block, HashMap<Var, Value>>,
    blocks: BlockMap<BlockData>,
    sealed: HashSet<Block>,
    incomplete_phis: HashSet<Value>,
}

impl<'a> Builder<'a> {
    pub fn new(program: &'a mut ProgramBuilder) -> Self {
        Builder {
            program: program,
            is_main: false,
            params: vec![],
            values: ValueMap::new(),
            defs: HashMap::new(),
            blocks: BlockMap::new(),
            sealed: HashSet::new(),
            incomplete_phis: HashSet::new(),
        }
    }

    pub fn is_main(&mut self, is_main: bool) {
        self.is_main = is_main;
    }

    pub fn block(&self, block: Block) -> &BlockData {
        &self.blocks[block]
    }

    pub fn block_mut(&mut self, block: Block) -> &mut BlockData {
        &mut self.blocks[block]
    }

    pub fn create_block(&mut self) -> Block {
        let block = BlockData::new();
        let block = self.blocks.insert(block);
        self.defs.insert(block, HashMap::new());
        block
    }

    /// All predecessors of `block` are known
    pub fn seal_block(&mut self, block: Block) {
        unimplemented!()
    }

    pub fn create_value(&mut self, block: Block, expr: Expr) -> Value {
        let value = self.values.insert(expr);
        self.block_mut(block).body.push(value);
        value
    }

    pub fn value(&self, value: Value) -> &Expr {
        &self.values[value]
    }

    pub fn value_mut(&mut self, value: Value) -> &mut Expr {
        &mut self.values[value]
    }

    pub fn end_block<B>(&mut self, block: Block, branch: B)
    where
        B: Into<Branch>
    {
        let branch = branch.into();
        assert!(
            mem::replace(&mut self.block_mut(block).end,
            Some(branch)).is_none()
        );
    }

    pub fn eval_flat_expr(&mut self,
                          block: Block,
                          expr: &FlatExpr) -> Value
    {
        use self::FlatExpr::*;
        let expr = match *expr {
            UnaryOp(op, var) => {
                let opcode = op.into();
                let value = self.use_var(block, var);
                self.unary(opcode, value)
            }
            BinOp(op, left, right) => {
                let opcode = op.into();
                let left = self.use_var(block, left);
                let right = self.use_var(block, right);
                self.binary(opcode, left, right)
            }
            CallFunc(var, ref args) => {
                let target = self.use_var(block, var);
                let target = match self.values[target] {
                    Expr::Function(function) => CallTarget::Direct(function),
                    ref expr => panic!("call to non-const target: {}", expr),
                };
                let args = args.iter()
                    .map(|&arg| self.use_var(block, arg))
                    .collect();
                self.call(target, args)
            }
            RuntimeFunc(ref name, ref args) => {
                let name: &'static str = match name.as_str() {
                    "is_true" => "is_true",
                    "print_any" => "print_any",
                    "input_int" => "input_int",
                    "create_list" => "create_list",
                    "create_dict" => "create_dict",
                    "set_subscript" => "set_subscript",
                    "get_subscript" => "get_subscript",
                    "add" => "add",
                    "equal" => "equal",
                    "not_equal" => "not_equal",
                    "create_closure" => "create_closure",
                    "get_fun_ptr" => "get_fun_ptr",
                    "get_free_vars" => "get_free_vars",
                    "set_free_vars" => "set_free_vars",
                    _ => panic!("unknown runtime function name: {}", name),
                };
                let target = CallTarget::Runtime(name);
                let args = args.iter()
                    .map(|&arg| self.use_var(block, arg))
                    .collect();
                self.call(target, args)
            }
            GetTag(var) => {
                let value = self.use_var(block, var);
                self.get_tag(block, value)
            }
            _ => unimplemented!()
        };
        self.create_value(block, expr)
    }

    pub fn unary(&mut self, opcode: Unary, value: Value) -> Expr {
        if let Expr::Const(i) = self.values[value] {
            match opcode {
                Unary::Mov => Expr::Const(i),
                Unary::Neg => Expr::Const(-i),
                Unary::Not => Expr::Const(!i),
            }
        } else {
            Expr::Unary {
                opcode: opcode,
                arg: value,
            }
        }
    }

    pub fn binary(&mut self, opcode: Binary,
                  left: Value, right: Value) -> Expr
    {
        match (&self.values[left], &self.values[right]) {
            (&Expr::Const(left), &Expr::Const(right)) => {
                match opcode {
                    Binary::Add => Expr::Const(left + right),
                    Binary::And => Expr::Const(left & right),
                    Binary::Or => Expr::Const(left | right),
                    Binary::Sete => Expr::Const(if left == right { 1 }
                                                else { 0 }),
                    Binary::Setne => Expr::Const(if left != right { 1 }
                                                 else { 0 }),
                    Binary::Shr => Expr::Const(left >> right),
                    Binary::Shl => Expr::Const(left << right),
                }
            }
            _ => {
                Expr::Binary {
                    opcode: opcode,
                    left: left,
                    right: right,
                }
            }
        }
    }

    pub fn call(&mut self, target: CallTarget, args: Vec<Value>) -> Expr {
        Expr::Call { target, args }
    }

    pub fn get_tag(&mut self, block: Block, value: Value) -> Expr {
        match self.values[value] {
            Expr::Const(i) => Expr::Const(i & ex::MASK),
            _ => {
                let mask = self.create_value(block, Expr::Const(ex::MASK));
                Expr::Binary {
                    opcode: Binary::And,
                    left: value,
                    right: mask,
                }
            },
        }
    }

    pub fn def_var(&mut self, block: Block, var: Var, value: Value) {
        self.defs.get_mut(&block).unwrap().insert(var, value);
    }

    pub fn use_var(&mut self, block: Block, var: Var) -> Value {
        if self.defs[&block].contains_key(&var) {
            return self.defs[&block][&var];
        }
        self.use_var_recursive(block, var)
    }

    fn use_var_recursive(&mut self, block: Block, var: Var) -> Value {
        // if block not sealed, then we do not try to read from it,
        // placing a phi function temporarily (and then later, when
        // we seal the block, we will fix-up the phi function with
        // the correct value).
        let value = if !self.is_sealed(block) {
            let phi = self.create_value(block, Expr::Phi(Phi::new(block)));
            self.incomplete_phis.insert(phi);
            phi
        } else if self.predecessors(block).len() == 1 {
            let &pred = self.predecessors(block).iter().nth(0).unwrap();
            self.use_var(pred, var)
        } else {
            let phi = self.create_value(block, Expr::Phi(Phi::new(block)));
            self.def_var(block, var, phi);
            self.add_phi_operands(var, phi)
        };
        self.def_var(block, var, value);
        value
    }

    fn phi(&self, phi: Value) -> &Phi {
        match *self.value(phi) {
            Expr::Phi(ref phi) => phi,
            ref expr => panic!("non-phi value: {}", expr),
        }
    }

    fn phi_mut(&mut self, phi: Value) -> &mut Phi {
        match *self.value_mut(phi) {
            Expr::Phi(ref mut phi) => phi,
            ref expr => panic!("non-phi value: {}", expr),
        }
    }

    pub fn add_phi_operands(&mut self, var: Var, phi: Value) -> Value {
        let phi_block = self.phi(phi).block;
        let predecessors = self.predecessors(phi_block).clone().into_iter();
        for pred in predecessors {
            let value = self.use_var(pred, var);
            self.phi_mut(phi).push(value);
        }

        self.try_remove_trivial_phi(phi)
    }

    pub fn try_remove_trivial_phi(&mut self, phi: Value) -> Value {
        let mut same = None;
        for &arg in &self.phi(phi).args {
            // if the arg is the same arg we saw before, or the phi itself,
            // we continue (ie, possibly trivial)
            if Some(arg) == same || arg == phi {
                continue
            }
            // if arg is not the same and not phi, then non-trivial, return
            if same != None {
                return phi
            }
            // save this arg to check for duplicates on next arg
            same = Some(arg);
        }
        // no args, or arg == phi
        let same = match same {
            Some(value) => value,
            None => panic!("phi value {} is undefined!", phi),
        };
        let users = self.replace_phi(phi, same);
        for user in users {
            if let Expr::Phi(_) = *self.value(user) {
                self.try_remove_trivial_phi(user);
            }
        }

        return same
    }

    /// Replaces all uses of `phi` with `same`
    /// Returns the values it was removed from
    fn replace_phi(&mut self, phi: Value, same: Value) -> HashSet<Value> {
        let mut users = HashSet::new();
        for (value, expr) in &mut self.values {
            if value == phi {
                continue
            }
            match *expr {
                Expr::Unary { ref mut arg, .. } if *arg == phi => {
                    *arg = same;
                }
                Expr::Binary { ref mut left, ref mut right, .. } if *left == phi || *right == phi => {
                    if *left == phi {
                        *left = same;
                    }
                    if *right == phi {
                        *right = same;
                    }
                }
                Expr::Call { ref mut args, .. } if args.contains(&phi) => {
                    for arg in args {
                        if *arg == phi {
                            *arg = same;
                        }
                    }
                }
                Expr::ShiftLeftThenOr { ref mut arg, .. } if *arg == phi => {
                    *arg = same;
                }
                Expr::Phi(ref mut other) if other.args.contains(&phi) => {
                    for arg in &mut other.args {
                        if *arg == phi {
                            *arg = same;
                        }
                    }
                }
                _ => continue,
            }
            users.insert(value);
        }

        users
    }

    pub fn is_sealed(&self, block: Block) -> bool {
        self.sealed.contains(&block)
    }

    pub fn predecessors(&self, block: Block) -> &HashSet<Block> {
        &self.block(block).predecessors
    }

    pub fn build(self) -> FunctionData {
        for (block, block_data) in &self.blocks {
            if block_data.end.is_none() {
                panic!("{} does not have a terminating branch intruction", block)
            }
        }
        FunctionData {
            is_main: self.is_main,
            params: self.params,
            values: self.values,
            defs: self.defs,
            blocks: self.blocks,
        }
    }
}
