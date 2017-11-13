use std::collections::HashMap;
use error::*;
use explicate::*;
use raise::TransformAst;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Builder<'var_data> {
    var_data: &'var_data mut var::Slab<var::Data>,
}

impl<'var_data> Builder<'var_data> {
    pub fn new(var_data: &'var_data mut var::Slab<var::Data>) -> Builder<'var_data> {
        Builder {
            var_data,
        }
    }

    pub fn heapify_module(&mut self, module: Module) -> Module {
        unimplemented!()
    }

    fn new_temp(&mut self) -> Var {
        self.var_data.insert(var::Data::Temp)
    }

    fn new_fvs_list(&mut self) -> Var {
        let fvs = self.var_data.insert(var::Data::Temp);
        fvs
    }
}
