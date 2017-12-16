use error::*;
use flatten::Flattener;
use std::fmt;
use ssa::Func;
use ssa::FuncData;
use ssa::FuncGen;
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
    func_map: HashMap<FlatFunc, Func>,
    completed_funcs: HashMap<Func, FuncData>,
}

impl Builder {
    fn new(units: &HashMap<FlatFunc, FlatFunction>) -> Builder {
        let mut trans = HashMap::new();
        let mut g = FuncGen::new();
        for (&f, function) in units {
            let func = g.gen();
            trans.insert(f, func);
        }

        Builder {
            func_map: trans,
            completed_funcs: HashMap::new(),
        }
    }

    fn complete_func(&mut self, func: FlatFunc, function: FlatFunction, is_main: bool) {
        unimplemented!()
    }

    fn complete(self) -> Program {
        Program {
            funcs: self.completed_funcs,
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "ssa {{}}")?;
        Ok(())
    }
}
