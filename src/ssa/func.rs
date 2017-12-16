use ssa::Block;
use ssa::BlockData;
use ssa::Val;
use ssa::ValGen;
use std::collections::HashMap;
use explicate::Var;
use raise::Func as FlatFunc;
use flatten::Function as FlatFunction;

impl_ref!(Func, "fn");

pub struct FuncData {
    pub main: bool,
    pub args: Vec<Val>,
    pub blocks: HashMap<Block, BlockData>,
}

pub type FuncGen = Gen;

pub struct Builder<'a> {
    func_map: &'a HashMap<FlatFunc, Func>,
    is_main: bool,
    args: Vec<Val>,
    val_gen: ValGen,
    var_map: HashMap<Block, HashMap<Var, Val>>,
    blocks: HashMap<Block, BlockData>,
}

impl<'a> Builder<'a> {
    pub fn new(func_map: &'a HashMap<FlatFunc, Func>, is_main: bool) -> Self {
        Builder {
            func_map,
            is_main,
            args: vec![],
            val_gen: ValGen::new(),
            var_map: HashMap::new(),
            blocks: HashMap::new(),
        }
    }

    pub fn args(&mut self, args: &[Var]) {
        unimplemented!()
    }

    pub fn complete(self) -> FuncData {
        FuncData {
            main: self.is_main,
            args: self.args,
            blocks: self.blocks,
        }
    }
}
