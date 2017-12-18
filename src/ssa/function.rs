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
use ssa::Jmp;
use ssa::Ret;
use ssa::Jnz;
use ssa::Unary;
use ssa::Binary;
use ssa::CallTarget;
use explicate::Var;
use std::mem;
use flatten::Expr as FlatExpr;
use flatten::Stmt as FlatStmt;
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
    pub root: Block,
}

impl FunctionData {
    /// Dead code pass!
    pub fn remove_unused_values(&mut self) {
        let mut change_made;
        loop {
            change_made = false;

            let mut visited = HashSet::new();
            let mut used_values = HashSet::new();
            for block in self.exit_blocks() {
                let c = self.remove_unused_values_block(block, &mut visited, &mut used_values);
                change_made |= c;
            }

            if !change_made {
                break;
            }
        }
    }

    /// Returns "changes made"
    fn remove_unused_values_block(&mut self,
                                  block: Block,
                                  visited: &mut HashSet<Block>,
                                  used_values: &mut HashSet<Value>) -> bool
    {
        if visited.contains(&block) {
            return false;
        }
        match *self.block(block).end.as_ref().unwrap() {
            Branch::Ret(ref ret) => {
                if let Some(value) = ret.value {
                    used_values.insert(value);
                }
            }
            Branch::Jmp(ref jmp) => {}
            Branch::Jnz(ref jnz) => {
                used_values.insert(jnz.cond);
            }
        }

        let mut trimmed = vec![];
        let body = mem::replace(&mut self.block_mut(block).body, vec![]);
        for &value in body.iter().rev() {
            if used_values.contains(&value) || self.values[value].has_side_effects() {
                trimmed.push(value);
            }
            let used = self.values[value].used_values();
            used_values.extend(used);
        }

        trimmed.reverse();
        let mut changes_made = trimmed.len() < body.len();
        self.block_mut(block).body = trimmed;

        for pred in self.block(block).predecessors.clone() {
            let c = self.remove_unused_values_block(pred, visited, used_values);
            changes_made |= c;
        }

        changes_made
    }

    /// Set of blocks that have return branches (ends of control flow graph)
    pub fn exit_blocks(&self) -> HashSet<Block> {
        let mut blocks = HashSet::new();
        for (block, block_data) in &self.blocks {
            if let Branch::Ret(_) = *block_data.end.as_ref().unwrap() {
                blocks.insert(block);
            }
        }
        blocks
    }

    pub fn reverse_order(&self) -> Vec<Block> {
        let mut blocks = Vec::new();
        let mut visited = set!();
        for block in self.exit_blocks() {
            self.reverse_order_blocks(block, &mut visited, &mut blocks);
        }
        blocks
    }

    fn reverse_order_blocks(&self,
                            block: Block,
                            visited: &mut HashSet<Block>,
                            blocks: &mut Vec<Block>)
    {
        if visited.contains(&block) {
            return;
        }
        blocks.push(block);
        visited.insert(block);
        for pred in self.block(block).predecessors.clone() {
            self.reverse_order_blocks(pred, visited, blocks);
        }
    }

    pub fn block(&self, block: Block) -> &BlockData {
        &self.blocks[block]
    }

    pub fn block_mut(&mut self, block: Block) -> &mut BlockData {
        &mut self.blocks[block]
    }

    pub fn convert_out_of_ssa(&mut self) {
        // must place assignments of v = k after k for all v for each k
        let mut copies: HashMap<Value, HashSet<Value>> = HashMap::new();
        for (block, block_data) in &mut self.blocks {
            let mut converted = vec![];
            for &value in &block_data.body {
                match self.values[value] {
                    Expr::Phi(ref phi) => {
                        for &arg in &phi.args {
                            copies.entry(arg)
                                .or_insert(HashSet::new())
                                .insert(value);
                        }
                    }
                    _ => converted.push(value),
                }
            }
            block_data.body = converted;
        }
        for (block, block_data) in &mut self.blocks {
            for (&copy, dsts) in &copies {
                if block_data.body.contains(&copy) {
                    for &dst in dsts {
                        block_data.body.push(dst);
                        if let Expr::JoinMov { ref mut value } = self.values[dst] {
                            value.insert(block, copy);
                            continue
                        }
                        self.values[dst] = Expr::JoinMov { value: map!(block => copy) };
                    }
                }
            }
        }
    }

    //pub fn spill(&mut self, value: Value, 
}

pub struct Builder<'a> {
    program: &'a mut ProgramBuilder,
    values: ValueMap<Expr>,
    params: Vec<Value>,
    is_main: bool,
    root: Option<Block>,
    defs: HashMap<Block, HashMap<Var, Value>>,
    blocks: BlockMap<BlockData>,
    sealed: HashSet<Block>,
    incomplete_phis: HashMap<Block, HashMap<Var, Value>>,
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
            incomplete_phis: HashMap::new(),
            root: None,
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
        self.incomplete_phis.insert(block, HashMap::new());
        block
    }

    pub fn set_root(&mut self, block: Block) {
        assert!(mem::replace(&mut self.root, Some(block)).is_none());
    }

    fn seal_root(&mut self, block: Block) {
        assert!(self.predecessors(block).is_empty());
        self.sealed.insert(block);
    }

    /// All predecessors of `block` are known
    pub fn seal_block(&mut self, block: Block) {
        if Some(block) == self.root {
            self.seal_root(block);
            return
        }
        assert!(self.predecessors(block).len() == 1 || self.predecessors(block).len() == 2,
            "wrong number of predecessors: {}: {}", block, self.predecessors(block).len());
        for (var, phi) in self.incomplete_phis[&block].clone() {
            self.add_phi_operands(var, phi);
        }
        self.sealed.insert(block);
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
        // Now get the successors and add block to predecessors
        for successor in self.successors(block) {
            self.block_mut(successor).predecessors.insert(block);
        }
    }

    pub fn eval_flat_stmts(&mut self, mut current_block: Block, stmts: &[FlatStmt]) -> Block {
        for stmt in stmts {
            use flatten::Stmt::*;
            match *stmt {
                Def(var, ref expr) => {
                    let value = self.eval_flat_expr(current_block, expr);
                    self.def_var(current_block, var, value);
                }
                Discard(ref expr) => {
                    let _ = self.eval_flat_expr(current_block, expr);
                }
                Return(ref var) => {
                    let value = if let Some(var) = *var {
                        Some(self.use_var(current_block, var))
                    } else { None };
                    self.end_block(current_block, ::ssa::Ret { value });
                    break;
                }
                While(var, ref header, ref body) => {
                    let before_while = current_block;
                    self.seal_block(before_while);
                    // previous blocks will not be returned to
                    let header_entry = self.create_block();
                    self.end_block(before_while, Jmp { destination: header_entry });
                    // cannot seal header_entry, because while loop will go back to it

                    let header_exit = self.eval_flat_stmts(header_entry, header);
                    if header_exit != header_entry {
                        self.seal_block(header_exit);
                    }

                    let body_entry = self.create_block();
                    let after_while = self.create_block();
                    let cond = self.use_var(header_exit, var);
                    self.end_block(header_exit, Jnz { cond, jnz: body_entry, jmp: after_while });
                    self.seal_block(body_entry);
                    self.seal_block(after_while);
                    let body_exit = self.eval_flat_stmts(body_entry, body);
                    if body_exit != body_entry {
                        self.seal_block(body_exit);
                    }
                    self.end_block(header_exit, Jmp { destination: header_entry });
                    self.seal_block(header_entry);
                    current_block = after_while;
                }
                If(cond, ref then, ref else_) => {
                    let before_if = current_block;
                    let cond = self.use_var(before_if, cond);

                    if let Expr::Const(i) = self.values[cond] {
                        let body = if i != 0 { then } else { else_ };
                        let body_entry = before_if;
                        let body_exit = self.eval_flat_stmts(body_entry, body);
                        current_block = body_exit;
                        continue
                    }

                    self.seal_block(before_if);
                    let then_entry = self.create_block();
                    let else_entry = self.create_block();
                    self.end_block(before_if, Jnz { cond, jnz: then_entry, jmp: else_entry });
                    self.seal_block(then_entry);
                    self.seal_block(else_entry);
                    let then_exit = self.eval_flat_stmts(then_entry, then);
                    self.seal_block(then_exit);
                    let else_exit = self.eval_flat_stmts(else_entry, else_);
                    self.seal_block(else_exit);
                    let after_if = self.create_block();
                    self.end_block(then_exit, Jmp { destination: after_if });
                    self.end_block(else_exit, Jmp { destination: after_if });
                    self.seal_block(after_if);
                    current_block = after_if;
                }
            }
        }

        current_block
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
            ProjectTo(var, ty) => {
                let value = self.use_var(block, var);
                self.project_to(block, value, ty)
            }
            InjectFrom(var, ty) => {
                let value = self.use_var(block, var);
                self.inject_from(block, value, ty)
            }
            Const(i) => {
                Expr::Const(i)
            }
            LoadFunctionPointer(raise_func) => {
                let function = self.program.function(raise_func);
                Expr::Function(function)
            }
            Copy(var) => {
                return self.use_var(block, var)
            }
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
        match target {
            CallTarget::Runtime(name) => {
                match name {
                    "is_true" => {
                        assert_eq!(args.len(), 1);
                        match self.values[args[0]] {
                            Expr::Const(i) => return Expr::Const(if i != 0 { 1 } else { 0 }),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
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

    pub fn project_to(&mut self, block: Block,
                      value: Value, ty: ex::Ty) -> Expr
    {
        use explicate::Ty::*;
        match self.values[value] {
            Expr::Const(i) => {
                match ty {
                    Int | Bool => Expr::Const(i >> ex::SHIFT),
                    Big => Expr::Const(i & (!ex::MASK)),
                    _ => panic!()
                }
            }
            _ => {
                match ty {
                    Int | Bool => Expr::Binary {
                        opcode: Binary::Shr,
                        left: value,
                        right: self.create_value(block,
                                                 Expr::Const(ex::SHIFT)),
                    },
                    Big => Expr::Binary {
                        opcode: Binary::And,
                        left: value,
                        right: self.create_value(block,
                                                 Expr::Const(!ex::MASK)),
                    },
                    _ => panic!()
                }
            }
        }
    }

    pub fn inject_from(&mut self, block: Block,
                       value: Value, ty: ex::Ty) -> Expr
    {
        use explicate::Ty::*;
        match self.values[value] {
            Expr::Const(i) => {
                match ty {
                    Int => Expr::Const((i << ex::SHIFT) | ex::INT_TAG),
                    Bool => Expr::Const((i << ex::SHIFT) | ex::BOOL_TAG),
                    Big => Expr::Const(i | ex::BIG_TAG),
                    _ => panic!()
                }
            }
            _ => {
                match ty {
                    Int => Expr::ShiftLeftThenOr {
                        arg: value,
                        shift: ex::SHIFT,
                        or: ex::INT_TAG,
                    },
                    Bool => Expr::ShiftLeftThenOr {
                        arg: value,
                        shift: ex::SHIFT,
                        or: ex::BOOL_TAG,
                    },
                    Big => Expr::Binary {
                        opcode: Binary::Or,
                        left: value,
                        right: self.create_value(block,
                                                 Expr::Const(ex::BIG_TAG)),
                    },
                    _ => panic!(),
                }
            }
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
            self.incomplete_phis.get_mut(&block).unwrap().insert(var, phi);
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

    pub fn successors(&self, block: Block) -> HashSet<Block> {
        match *self.block(block).end.as_ref().unwrap() {
            Branch::Ret(ref ret) => set!(),
            Branch::Jmp(ref jmp) => set!(jmp.destination),
            Branch::Jnz(ref jnz) => set!(jnz.jnz, jnz.jmp),
        }
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
            root: self.root.unwrap(),
        }
    }
}
