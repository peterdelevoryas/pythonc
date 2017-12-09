use std::collections::HashSet;
use vm::Module;
use vm::Func;
use vm::FuncData;
use vm::Block;
use vm::BlockData;
use vm::Inst;
use vm::Term;

pub trait Visit {
    fn traverse_module(&mut self, module: &Module) {
        for (_, func) in &module.funcs {
            self.visit_func(func);
        }
    }

    fn traverse_func(&mut self, func: &FuncData) {
        struct Dfs<'func_data, F>
        where
            F: FnMut(&BlockData)
        {
            func_data: &'func_data FuncData,
            visited: HashSet<Block>,
            callback: F,
        }

        impl<'func_data, F> Dfs<'func_data, F>
        where
            F: FnMut(&BlockData),
        {
            fn dfs(&mut self, block: &BlockData) {
                if !self.visited(&block.name) {
                    self.visit(block);
                    for s in self.func_data.block(&block.name).successors() {
                        self.dfs(self.func_data.block(&s));
                    }
                }
            }

            fn visited(&self, block: &Block) -> bool {
                self.visited.contains(block)
            }

            fn visit(&mut self, block: &BlockData) {
                (self.callback)(block);
                self.visited.insert(block.name.clone());
            }
        }

        let mut dfs = Dfs {
            func_data: func,
            visited: HashSet::new(),
            callback: move |block| {
                self.visit_block(block);
                self.traverse_block(block);
            }
        };

        dfs.dfs(func.root());
    }

    fn traverse_block(&mut self, block: &BlockData) {
        for inst in &block.body {
            self.visit_inst(inst);
        }
        self.visit_term(&block.term);
    }

    fn visit(&mut self, module: &Module) {
        self.visit_module(module);
        self.traverse_module(module);
    }
    fn visit_module(&mut self, module: &Module) {}
    fn visit_func(&mut self, _func: &FuncData) {}
    fn visit_block(&mut self, _block: &BlockData) {}
    fn visit_inst(&mut self, _inst: &Inst) {}
    fn visit_term(&mut self, _term: &Term) {}
}
