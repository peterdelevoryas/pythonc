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
use explicate::Var;

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

    pub fn create_value(&mut self, expr: Expr) -> Value {
        self.values.insert(expr)
    }

    pub fn def_var(&mut self, block: Block, var: Var, value: Value) {
        self.defs.get_mut(&block).unwrap().insert(var, value);
        self.block_mut(block).body.push(value);
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
            let phi = self.create_value(Expr::Phi(Phi::new(block)));
            self.incomplete_phis.insert(phi);
            phi
        } else if self.predecessors(block).len() == 1 {
            let &pred = self.predecessors(block).iter().nth(0).unwrap();
            self.use_var(pred, var)
        } else {
            let phi = self.create_value(Expr::Phi(Phi::new(block)));
            self.def_var(block, var, phi);
            self.add_phi_operands(var, phi)
        };
        self.def_var(block, var, value);
        value
    }

    pub fn add_phi_operands(&mut self, var: Var, phi: Value) -> Value {
        unimplemented!()
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
