use std::collections::HashMap;
use std::fmt;
use cfg;
use vm::VarEnv;
use vm::Func;
use vm::FuncData;
use vm::func::Builder as FuncBuilder;
use explicate::VarData;
use raise;

pub struct Module {
    pub vars: VarEnv,
    pub funcs: HashMap<Func, FuncData>,
}

impl Module {
    pub fn new(m: cfg::Module) -> Self {
        let func_map = m.functions.iter()
            .map(|(&f, function)| {
                let func = Func::new(f.inner(), function.name.clone());
                (f, func)
            })
            .collect();
        let mut b = Builder::new(&m.var_data, func_map);
        for (f, function) in m.functions {
            b.visit_function(f, function, f == m.main);
        }
        b.build()
    }
}

struct Builder<'var_data> {
    var_data: &'var_data VarData,
    vars: VarEnv,
    funcs: HashMap<Func, FuncData>,
    func_map: HashMap<raise::Func, Func>,
}

impl<'var_data> Builder<'var_data> {
    fn new(var_data: &'var_data VarData, func_map: HashMap<raise::Func, Func>) -> Self {
        let vars = VarEnv::from(var_data);
        let funcs = HashMap::new();
        Builder {
            var_data,
            vars,
            funcs,
            func_map,
        }
    }

    fn visit_function(&mut self, f: raise::Func, function: cfg::Function, _is_main: bool) {
        let b = FuncBuilder::new(&mut self.vars, self.var_data, self.func_map.clone());
        let func_data = b.build(f, function);
        self.funcs.insert(func_data.name.clone(), func_data);
    }

    fn build(self) -> Module {
        Module {
            vars: self.vars,
            funcs: self.funcs,
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_, func) in &self.funcs {
            write!(f, "{}", func)?;
        }
        Ok(())
    }
}
