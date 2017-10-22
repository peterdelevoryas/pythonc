use bb::{self, BasicBlock};
use val::{self, Val, Const};
use inst::{self, Inst};
use ty::Ty;
use slab::Slab;
use std::collections::HashMap;
use ast;
use std::fmt;

#[derive(Debug)]
pub struct Func {
    name: String,
    bbs: Slab<BasicBlock, bb::Data>,
    vals: Slab<Val, val::Data>,
    ret: Ty,
}

impl Func {
    pub fn build(m: &ast::Module) -> Func {
        let b = Builder::new();
        b.build(m)
    }
}

pub struct Builder {
    curr: bb::Partial,
    bbs: Slab<BasicBlock, bb::Data>,
    vals: Slab<Val, val::Data>,
    names: HashMap<String, inst::Arg>,
}

impl Builder {
    fn new() -> Builder {
        Builder {
            curr: bb::Partial::new(),
            bbs: Slab::new(),
            vals: Slab::new(),
            names: HashMap::new(),
        }
    }

    fn build(mut self, m: &ast::Module) -> Func {
        for st in &m.statements {
            self.flatten_statement(st);
        }
        let ret_bb = self.curr.ret();
        let _bb = self.bbs.insert(ret_bb);
        let name = "__main__".into();

        Func {
            name,
            bbs: self.bbs,
            vals: self.vals,
            ret: Ty::Int,
        }
    }

    fn flatten_statement(&mut self, st: &ast::Statement) {
        use ast::Statement::*;
        use ast::Target::*;
        match *st {
            Newline => {}
            Print(ref e) => {
                let arg = self.flatten_expression(e);
                self.print(arg);
            }
            Assign(Name(ref name), ref e) => {
                // flatten expression, creating either a const or val
                let operand = self.flatten_expression(e);
                // replace current mapping for "name"
                // with "operand"
                self.assign(name, operand);
            }
            _ => unimplemented!()
        }
    }

    fn flatten_expression(&mut self, e: &ast::Expression) -> inst::Arg {
        use ast::Expression::*;
        match *e {
            Target(ref t) => self.flatten_rvalue_target(t),
            DecimalI32(i) => Const::Int(i).into(),
            Boolean(b) => Const::Bool(b).into(),
            Input => Const::Func(INPUT).into(),
            UnaryNeg(ref e) => {
                let e = self.flatten_expression(e);
                self.unop(inst::Unop::Neg, e)
            }
            _ => unimplemented!()
        }
    }

    fn flatten_rvalue_target(&mut self, t: &ast::Target) -> inst::Arg {
        use ast::Target::*;
        match *t {
            Name(ref name) => {
                // lookup name in mapping of names to vals
                self.names.get(name)
                    .ok_or(format!("Reference to undefined name {:?}", name))
                    .unwrap()
                    .clone()
            }
            _ => unimplemented!()
        }
    }

    fn print(&mut self, arg: inst::Arg) {
        let _val = self.def(inst::Inst::Call {
            func: Const::from(PRINT).into(),
            args: vec![arg],
        });
    }

    fn assign(&mut self, name: &str, arg: inst::Arg) {
        // throwaway previous value
        let _ = self.names.insert(name.to_owned(), arg);
    }

    fn binop(&mut self, binop: inst::Binop, l: inst::Arg, r: inst::Arg) -> inst::Arg {
        unimplemented!()
    }

    fn unop(&mut self, unop: inst::Unop, arg: inst::Arg) -> inst::Arg {
        match arg {
            inst::Arg::Const(c) => c.unop(unop).into(),
            inst::Arg::Loc(val) => self.def(inst::Inst::Unop(unop, arg)).into(),
        }
    }

    fn call(&mut self, func: Val, args: Vec<Val>) -> Val {
        unimplemented!()
    }

    fn def(&mut self, inst: inst::Inst) -> Val {
        let ret_ty = inst.ret_ty(&self.vals);
        let data = val::Data::unnamed(ret_ty);
        let val = self.vals.insert(data);
        self.curr.push(val, inst);
        val
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstFunc {
    pub name: &'static str,
    pub args: &'static [Ty],
    pub ret: Ty,
}

const INPUT: &'static ConstFunc = &ConstFunc {
    name: "input",
    args: &[],
    ret: Ty::Int,
};

const PRINT: &'static ConstFunc = &ConstFunc {
    name: "print_int_nl",
    args: &[Ty::Int],
    ret: Ty::Unit,
};

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "func {}() -> {} {{", self.name, self.ret)?;

        for (val, data) in self.vals.iter() {
            writeln!(f, "    val {}: {};", val, data.ty())?;
        }

        for (bb, data) in self.bbs.iter() {
            writeln!(f, "{}:", bb)?;
            for def in data.defs() {
                writeln!(f, "    {} = {};", def.0, def.1)?;
            }
            writeln!(f, "    {}", data.term())?;
        }

        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use func;
    use ast::Expression::*;
    use ast::Statement::*;
    use ast::Target::*;
    use ast::Module;

    #[test]
    fn flatten_target() {
        //unimplemented!()
    }

    #[test]
    fn flatten_statement() {
        //unimplemented!()
    }

    #[test]
    fn flatten_expression() {
        //unimplemented!()
    }

    #[test]
    fn const_flattening() {
        let module = Module {
            statements: vec![
                Assign(Name("x".into()), DecimalI32(64)),
                Print(Target(Name("x".into()))),
            ]
        };
        let f = func::Func::build(&module);
        println!("{}", f);
    }
}
