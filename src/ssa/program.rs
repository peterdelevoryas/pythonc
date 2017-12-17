use std::fmt;
use std::collections::HashMap;
use ssa::Function;
use ssa::FunctionData;
use ssa::FunctionMap;
use raise::Func as RaiseFunc;
use flatten::Function as FlatFunction;
use flatten::Flattener;
use itertools;

pub struct Program {
    pub functions: FunctionMap<FunctionData>,
}

pub struct Builder {
    functions: FunctionMap<Option<FunctionData>>,
    flat_function_map: HashMap<RaiseFunc, Function>,
}

impl Builder {
    pub fn new(units: &HashMap<RaiseFunc, FlatFunction>) -> Builder {
        let mut functions = FunctionMap::new();
        let flat_function_map = {
            let mut map = map!();
            for &raise_func in units.keys() {
                let function = functions.insert(None);
                map.insert(raise_func, function);
            }
            map
        };

        Builder { functions, flat_function_map }
    }

    pub fn function(&self, raise_func: RaiseFunc) -> Function {
        self.flat_function_map[&raise_func]
    }

    pub fn add_function(&mut self, raise_func: RaiseFunc, function_data: FunctionData) -> Function {
        let function = self.function(raise_func);
        self.functions[function] = Some(function_data);
        function
    }

    pub fn build(self) -> Program {
        let mut functions = FunctionMap::new();
        for (function, function_data) in self.functions {
            let function_data = match function_data {
                Some(data) => data,
                None => panic!("function is none"),
            };
            let f = functions.insert(function_data);
            assert_eq!(f, function);
        }
        Program { functions }
    }
}

use ssa::LiveSets;

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (function, function_data) in &self.functions {
            let args = itertools::join(&function_data.params, ", ");
            if function_data.is_main {
                writeln!(f, "function main({}) {{", args)?;
            } else {
                writeln!(f, "function {}({})", function, args)?;
            }
            writeln!(f, "reverse order: {}",
                     itertools::join(&function_data.reverse_order(), ", "))?;
            let livesets = LiveSets::new(&function_data);
            for b in function_data.reverse_order().into_iter().rev() {
                let in_ = &livesets.in_[&b];
                let out = &livesets.out[&b];
                let gens = &livesets.gens[&b];
                let kills = &livesets.kills[&b];
                writeln!(f, "{}:", b)?;
                writeln!(f, "   in    ({})", itertools::join(in_, ", "))?;
                writeln!(f, "   out   ({})", itertools::join(out, ", "))?;
                writeln!(f, "   gens  ({})", itertools::join(gens, ", "))?;
                writeln!(f, "   kills ({})", itertools::join(kills, ", "))?;
                writeln!(f, "   preds ({})", itertools::join(&function_data.block(b).predecessors, ", "))?;
            }
            writeln!(f)?;

            for (block, block_data) in &function_data.blocks {
                writeln!(f, "{}:", block)?;
                for &value in &block_data.body {
                    writeln!(f, "    {} = {}", value, function_data.values[value])?;
                }
                writeln!(f, "    {}", block_data.end.as_ref().unwrap())?;
            }

            writeln!(f, "}}")?;
        }
        Ok(())
    }
}
