use std::collections::HashMap;
use error::*;
use explicate::*;
use raise::TransformAst;
use std::collections::HashSet;

pub struct Builder<'var_data> {
    free_vars: HashSet<Var>,
    var_data: &'var_data mut var::Slab<var::Data>,
}

impl<'var_data> Builder<'var_data> {
    pub fn build(var_data: &'var_data mut var::Slab<var::Data>, m: Module) -> Module {
        let mut builder = Builder {
            var_data: var_data,
            //free_vars: m.free_vars(),
            free_vars: unimplemented!(),
        };
        let stmts = m.stmts.into_iter().map(|stmt| {
            builder.stmt(stmt)
        }).collect();
        Module { stmts }
    }

    fn needs_heapify(&self, var: Var) -> bool {
        self.free_vars.contains(&var)
    }

    fn new_temp(&mut self) -> Var {
        self.var_data.insert(var::Data::Temp)
    }

    fn new_fvs_list(&mut self) -> Var {
        let fvs = self.var_data.insert(var::Data::Temp);
        fvs
    }

    fn heap_assign(&mut self) -> Assign {
        let tmp = self.new_temp();
        Assign {
            target: tmp.into(),
            expr: List {
                exprs: vec![
                    Const::Int(0).into()
                ],
            }.into(),
        }
    }
}

impl<'var_data> TransformAst for Builder<'var_data> {
    fn var(&mut self, var: Var) -> Expr {
        if self.needs_heapify(var) {
            Subscript {
                base: var.into(),
                elem: Const::Int(0).into(),
            }.into()
        } else {
            var.into()
        }
    }

    fn target_var(&mut self, var: Var) -> Target {
        if self.needs_heapify(var) {
            Subscript {
                base: var.into(),
                elem: Const::Int(0).into(),
            }.into()
        } else {
            var.into()
        }
    }

    fn closure(&mut self, closure: Closure) -> Expr {
        unimplemented!()
        /*
        // first compute free vars for lower level
        self.free_vars.extend(closure.free_vars());
        // closure code gets heapified first, to add lower level free vars
        let closure_code: Vec<Stmt> = closure.code.into_iter().map(|stmt| {
            self.stmt(stmt)
        }).collect();
        // now, after heapification of lower level, compute free vars again
        self.free_vars.extend(closure_code.free_vars());

        let fvs = self.new_fvs_list();
        let args = {
            let mut args = vec![];
            args.push(fvs);
            args.extend(closure.args);
            args
        };
        let body = {
            let mut body = vec![];
            body.extend(closure_code);
            body
        };
        let mut closure: Expr = Closure {
            args: args,
            code: body,
        }.into();

        for &free_var in &self.free_vars {
            closure = let_(free_var, List { exprs: vec![Const::Int(0).into()] }, closure).into();
        }
        closure
        */
    }
}
