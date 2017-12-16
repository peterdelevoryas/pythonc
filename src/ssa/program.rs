use error::*;
use flatten::Flattener;
use std::fmt;
use ssa::Func;
use ssa::FuncData;
use ssa::FuncGen;
use ssa::FuncBuilder;
use raise::Func as FlatFunc; // flatten uses raise::Func for ident
use flatten::Function as FlatFunction;
use std::collections::HashMap;

pub struct Program {
    pub funcs: HashMap<Func, FuncData>,
}

impl Program {
    pub fn from(flattener: Flattener) -> Program {
        let mut builder = Builder::new(&flattener.units);
        for (f, function) in flattener.units {
            builder.complete_func(f, function, f == flattener.main);
        }

        builder.complete()
    }
}

struct Builder {
    // translates FlatFunc's to Func's
    flat_func_map: HashMap<FlatFunc, Func>,
    completed_funcs: HashMap<Func, FuncData>,
}

impl Builder {
    fn new(units: &HashMap<FlatFunc, FlatFunction>) -> Builder {
        let mut flat_func_map = HashMap::new();
        let mut g = FuncGen::new();
        for (&flat_func, flat_function) in units {
            let func = g.gen();
            flat_func_map.insert(flat_func, func);
        }

        Builder {
            flat_func_map,
            completed_funcs: HashMap::new(),
        }
    }

    fn complete_func(&mut self, flat_func: FlatFunc, flat_function: FlatFunction, is_main: bool) {
        let mut func_data = FuncData::new(&flat_function.args, is_main);
        {
            let mut builder = FuncBuilder::new(&self.flat_func_map, &mut func_data);
            for stmt in &flat_function.body {
                if builder.visit_stmt(stmt) {
                    break
                }
            }
            let last = builder.curr;
            builder.term_block(last, ::ssa::Term::Ret { ret: None });
            builder.complete();
        }

        let func = self.translate_flat_func(flat_func);
        self.completed_funcs.insert(func, func_data);
    }

    fn translate_flat_func(&self, flat_func: FlatFunc) -> Func {
        match self.flat_func_map.get(&flat_func) {
            Some(&func) => func,
            None => panic!("no flat func map entry for {}", flat_func),
        }
    }

    fn complete(self) -> Program {
        Program {
            funcs: self.completed_funcs,
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use vm::util::fmt_indented;
        use itertools::join;
        use itertools::sorted;

        for (func, func_data) in &self.funcs {
            writeln!(f, "fn {}({}) {{", func, join(&func_data.args, ", "))?;

            let mut blocks: Vec<_> = func_data.blocks.clone().into_iter().collect();
            blocks.sort_by(|&(l, _), &(r, _)| {
                l.inner().cmp(&r.inner())
            });
            for (block, block_data) in blocks {
                writeln!(f, "block {}:", block)?;
                for val in &block_data.body {
                    writeln!(f, "    {} = {}", val, func_data.vals[val])?;
                }
                match block_data.term {
                    Some(ref term) => writeln!(f, "{}", fmt_indented(term))?,
                    None => writeln!(f, "!! no term")?,
                }
            }
            writeln!(f, "}}")?;
        }

        Ok(())
    }
}
