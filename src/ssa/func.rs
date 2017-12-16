use ssa::Block;
use ssa::BlockData;
use ssa::BlockGen;
use ssa::Val;
use ssa::ValGen;
use ssa::Expr;
use std::collections::HashMap;
use std::collections::HashSet;
use explicate::Var;
use raise::Func as FlatFunc;
use flatten::Function as FlatFunction;
use flatten::Stmt as FlatStmt;

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
    flat_func_map: &'flat_func_map HashMap<FlatFunc, Func>,
    func_data: &'func_data mut FuncData,
    curr: Block,
}

impl<'flat_func_map, 'func_data> Builder<'flat_func_map, 'func_data> {
    pub fn new(flat_func_map: &'flat_func_map HashMap<FlatFunc, Func>, func_data: &'func_data mut FuncData) -> Self
    {
        let curr = func_data.root;
        Builder { flat_func_map, func_data, curr }
    }

    pub fn visit_stmt(&mut self, flat_stmt: &FlatStmt) {
        use self::FlatStmt::*;
        match *flat_stmt {
            _ => unimplemented!()
        }
    }

    pub fn create_block(&mut self) -> Block {
        let block = self.func_data.block_gen.gen();
        let block_data = BlockData::new();
        self.func_data.blocks.insert(block, block_data);
        self.func_data.defs.insert(block, map!());

        block
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
            val = self.new_phi(block);
            self.func_data.incomplete_phis.get_mut(&block).unwrap().insert(var, val);
        } else if self.func_data.blocks[&block].preds.len() == 1 {
            let pred = self.func_data.blocks[&block].preds_iter().nth(0).unwrap();
            val = self.read(pred, var);
        } else {
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
        self.func_data.vals.insert(val, Expr::Phi { block, vals: vec![] });
        val
    }

    pub fn seal_block(&mut self, block: Block) {
        for (var, val) in self.func_data.incomplete_phis[&block].clone() {
            self.add_phi_operands(var, val);
        }
        self.func_data.sealed_blocks.insert(block);
    }

    pub fn complete(self) {}
}
