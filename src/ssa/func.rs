use ssa::Block;
use ssa::BlockData;
use ssa::BlockGen;
use ssa::Val;
use ssa::ValGen;
use ssa::Expr;
use std::collections::HashMap;
use explicate::Var;
use raise::Func as FlatFunc;
use flatten::Function as FlatFunction;

impl_ref!(Func, "fn");

pub struct FuncData {
    pub main: bool,
    pub args: Vec<Val>,
    pub block_gen: BlockGen,
    pub blocks: HashMap<Block, BlockData>,
    pub defs: HashMap<Block, HashMap<Var, Expr>>,
    pub val_gen: ValGen,
}

pub type FuncGen = Gen;

impl FuncData {
    pub fn new(args: &[Var], is_main: bool) -> FuncData {
        let mut block_gen = BlockGen::new();
        let mut blocks = map!();
        let mut defs = map!();
        let mut val_gen = ValGen::new();

        let root = block_gen.gen();
        let root_data = BlockData {
            body: vec![],
        };
        blocks.insert(root, root_data);
        defs.insert(root, HashMap::new());

        let mut arg_vals = vec![];
        for (position, &arg) in args.iter().enumerate() {
            let val = val_gen.gen();
            defs.get_mut(&root)
                .unwrap()
                .insert(arg, Expr::LoadParam { position });
            arg_vals.push(val);
        }

        FuncData {
            main: is_main,
            args: arg_vals,
            block_gen: block_gen,
            blocks: blocks,
            defs: defs,
            val_gen: val_gen,
        }
    }
}

pub struct Builder<'flat_func_map, 'func_data> {
    flat_func_map: &'flat_func_map HashMap<FlatFunc, Func>,
    func_data: &'func_data mut FuncData,
}

impl<'flat_func_map, 'func_data> Builder<'flat_func_map, 'func_data> {
    pub fn new(flat_func_map: &'flat_func_map HashMap<FlatFunc, Func>, func_data: &'func_data mut FuncData) -> Self
    {
        Builder { flat_func_map, func_data }
    }

    pub fn complete(self) {}
}
