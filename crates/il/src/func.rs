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
            Subscript(ref base, ref index) => {
                let base = self.flatten_expression(base);
                let index = self.flatten_expression(index);
                unimplemented!()
            }
            _ => unimplemented!()
        }
    }

    fn print(&mut self, arg: inst::Arg) {
        let _val = self.call_direct(PRINT_ANY, vec![arg]);
    }

    fn inject(&mut self, arg: inst::Arg) -> Val {
        use inst::Arg::*;
        let val = match arg {
            c @ Const(_) => self.copy(c),
            Loc(val) => val,
        };
        let inject_fn = match self.vals[val].ty() {
            Ty::Int => INJECT_INT,
            Ty::Bool => INJECT_BOOL,
            Ty::PointerPyObj => INJECT_BIG,
            // early return, inject is unnecessary!
            Ty::PyObj => return val,
            ty => panic!("Cannot inject val of type \"{}\"", ty),
        };
        self.call_direct(inject_fn, vec![inst::Arg::Loc(val)])
    }

    fn copy(&mut self, arg: inst::Arg) -> Val {
        self.def(inst::Inst::Unop(inst::Unop::Copy, arg))
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

    fn call_direct(&mut self, func: &'static ConstFunc, args: Vec<inst::Arg>) -> Val {
        assert_eq!(func.args.len(), args.len(), "call_direct args len does not match function signature");

        let args = args.into_iter()
            .zip(func.args.iter())
            .map(|(arg, &ty)| {
                self.cast(arg, ty)
            })
            .collect();

        self.def(inst::Inst::Call {
            func: Const::from(func).into(),
            args,
        })
    }

    fn cast(&mut self, arg: inst::Arg, ty: Ty) -> inst::Arg {
        use inst::Arg::*;
        use self::Ty::*;
        let arg_ty = arg.ty(&self.vals);
        match (arg_ty, ty) {
            (a, b) if a == b => return arg,
            (_, PyObj) => {
                let v = self.inject(arg);
                inst::Arg::Loc(v)
            }
            (PyObj, ty) => {
                if let inst::Arg::Loc(v) = arg {
                    let v = self.project(v, ty);
                    inst::Arg::Loc(v)
                } else {
                    panic!("Const with type PyObj? {}", arg)
                }
            }
            (a, b) => panic!("Cannot cast {} to {}", arg, ty),
        }
    }

    fn project(&mut self, val: Val, ty: Ty) -> Val {
        assert!(self.vals[val].ty() == Ty::PyObj);
        let project_fn = match ty {
            Ty::Int => PROJECT_INT,
            Ty::Bool => PROJECT_BOOL,
            Ty::PointerPyObj => PROJECT_BIG,
            _ => panic!("Cannot project {} to {}", val, ty),
        };
        self.call_direct(project_fn, vec![inst::Arg::Loc(val)])
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

macro_rules! runtime_functions {
    (
        $(
            const $name:ident = func $func_name:ident($($tt:tt),*) -> $ret:tt;
        )*
    ) => {
        $(
            const $name: &'static ConstFunc = &ConstFunc {
                name: stringify!($func_name),
                args: &[
                    $(
                        ty_ident_to_expr!($tt)
                    ),*
                ],
                ret: ty_ident_to_expr!($ret),
            };
        )*
    }
}

macro_rules! ty_ident_to_expr {
    (int) => (Ty::Int);
    (bool) => (Ty::Bool);
    (pyobj) => (Ty::PyObj);
    (big_pyobj) => (Ty::PointerPyObj);
    (()) => (Ty::Unit)
}

runtime_functions! {
    const INPUT         = func input() -> int;
    const PRINT_INT_NL  = func print_int_nl(int) -> ();
    const TAG           = func tag(pyobj) -> int;
    const IS_INT        = func is_int(pyobj) -> int;
    const IS_BOOL       = func is_bool(pyobj) -> int;
    const IS_BIG        = func is_big(pyobj) -> int;
    const INJECT_INT    = func inject_int(int) -> pyobj;
    const INJECT_BOOL   = func inject_bool(int) -> pyobj;
    const INJECT_BIG    = func inject_big(big_pyobj) -> pyobj;
    const PROJECT_INT   = func project_int(pyobj) -> int;
    const PROJECT_BOOL  = func project_bool(pyobj) -> int;
    const PROJECT_BIG   = func project_big(pyobj) -> big_pyobj;
    const IS_TRUE       = func is_true(pyobj) -> int;
    const PRINT_ANY     = func print_any(pyobj) -> ();
    const INPUT_INT     = func input_int() -> pyobj;
    const CREATE_LIST   = func create_list(pyobj) -> big_pyobj;
    const CREATE_DICT   = func create_dict() -> big_pyobj;
    const SET_SUBSCRIPT = func set_subscript(pyobj, pyobj, pyobj) -> pyobj;
    const GET_SUBSCRIPT = func get_subscript(pyobj, pyobj) -> pyobj;
    const ADD           = func add(big_pyobj, big_pyobj) -> big_pyobj;
    const EQUAL         = func equal(big_pyobj, big_pyobj) -> int;
    const NOT_EQUAL     = func not_equal(big_pyobj, big_pyobj) -> int;
}

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
