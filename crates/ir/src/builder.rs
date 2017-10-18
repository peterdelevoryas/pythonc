use std::collections::HashMap;
use tmp::Allocator as TmpAllocator;
use tmp::Tmp;
use value::Val;
use statement::Stmt;
use expression::Expr;
use program::Program;
use ast;

#[derive(Debug)]
pub struct Builder<'alloc> {
    stack: Vec<Stmt>,
    names: HashMap<String, Val>,
    tmp: &'alloc mut TmpAllocator,
}

impl<'tmp_allocator> Builder<'tmp_allocator> {
    pub fn build(ast: ast::Program, tmp: &mut TmpAllocator) -> Program {
        let mut builder = Builder::new(tmp);
        for statement in &ast.module.statements {
            //println!("builder: {:#?}", builder);
            builder.flatten_statement(statement);
        }
        Program { stmts: builder.stack }
    }

    pub fn new(tmp: &'tmp_allocator mut TmpAllocator) -> Builder<'tmp_allocator> {
        Builder {
            stack: vec![],
            names: HashMap::new(),
            tmp,
        }
    }

    /// Flattens expression into a series of `tN := expr` `Stmt`s,
    /// pushing each onto the stack. The val of the final `Expr`
    /// (either a constant or the reference to the `Stmt` that
    /// gives the result) is returned. In other words, allocation
    /// of `Tmp`s only occurs for expressions that are not just values
    /// (because if the expression is just a `Val`, then the `Val` is
    /// simply returned)
    pub fn flatten_expression(&mut self, expression: &ast::Expression) -> Val {
        match *expression {
            ast::Expression::If(_, _, _) => unimplemented!(),
            ast::Expression::Target(ast::Target::Name(ref name)) => {
                match self.names.get(name) {
                    Some(&val) => val,
                    None => panic!("reference to undefined name {:?}", name),
                }
            }
            ast::Expression::Target(ast::Target::Subscript(ref t, ref i)) => {
                let tgt = self.flatten_expression(t);
                let index = self.flatten_expression(i);

                if let Val::PyObj(_) = tgt {
                    let tmp;
                    if let Val::PyObj(_) = index {
                        tmp = index;
                    } else {
                        tmp = self.inject(index);
                    }
                    Val::PyObj(self.def(Expr::FunCall(
                        String::from("get_subscript"),
                        vec![tgt, tmp],
                    )))
                } else {
                    panic!("Attempted subscript of illegal val {:?}", tgt)
                }
            }
            ast::Expression::DecimalI32(i) => Val::Const(i, false),
            ast::Expression::Boolean(b) => Val::Const(if b { 1 } else { 0 }, true),
            ast::Expression::Input => {
                Val::PyObj(self.def(Expr::FunCall(String::from("input"), vec![])))
            }
            ast::Expression::UnaryNeg(ref expr) => {
                match self.flatten_expression(expr) {
                    Val::Const(i, _) => Val::Const(-i, false),
                    v @ Val::PyObj(_) => Val::PyObj(self.def(Expr::UnaryNeg(v))),
                    _ => unimplemented!(),
                }
            }
            ast::Expression::Add(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);

                match (left, right) {
                    (Val::Const(li, _), Val::Const(ri, _)) => Val::Const(li + ri, false),
                    (l @ Val::PyObj(_), r @ Val::Const(_, _)) => {
                        let injected = self.inject(r);
                        Val::PyObj(self.def(Expr::Add(l, injected)))
                    }
                    (l @ Val::Const(_, _), r @ Val::PyObj(_)) => {
                        let injected = self.inject(l);
                        Val::PyObj(self.def(Expr::Add(injected, r)))
                    }
                    (l @ Val::PyObj(_), r @ Val::PyObj(_)) => Val::PyObj(self.def(Expr::Add(l, r))),
                }
            }
            ast::Expression::LogicalNot(ref expr) => {
                let val = self.flatten_expression(expr);
                match val {
                    Val::Const(i, _) => Val::Const(if i != 0 { 1 } else { 0 }, true),
                    v @ Val::PyObj(_) => Val::PyObj(self.def(Expr::Not(v))),
                }
            }
            ast::Expression::LogicalAnd(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);

                match (left, right) {
                    (Val::Const(li, _), Val::Const(ri, _)) => {
                        Val::Const(if (li != 0) && (ri != 0) { 1 } else { 0 }, true)
                    }
                    (l @ Val::PyObj(_), r @ Val::Const(_, _)) => {
                        let injected = self.inject(r);
                        Val::PyObj(self.def(Expr::And(l, injected)))
                    }
                    (l @ Val::Const(_, _), r @ Val::PyObj(_)) => {
                        let injected = self.inject(l);
                        Val::PyObj(self.def(Expr::And(injected, r)))
                    }
                    (l @ Val::PyObj(_), r @ Val::PyObj(_)) => Val::PyObj(self.def(Expr::And(l, r))),
                }
            }
            ast::Expression::LogicalOr(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);

                match (left, right) {
                    (Val::Const(li, _), Val::Const(ri, _)) => {
                        Val::Const(if (li != 0) || (ri != 0) { 1 } else { 0 }, true)
                    }
                    (l @ Val::PyObj(_), r @ Val::Const(_, _)) => {
                        let injected = self.inject(r);
                        Val::PyObj(self.def(Expr::Or(l, injected)))
                    }
                    (l @ Val::Const(_, _), r @ Val::PyObj(_)) => {
                        let injected = self.inject(l);
                        Val::PyObj(self.def(Expr::Or(injected, r)))
                    }
                    (l @ Val::PyObj(_), r @ Val::PyObj(_)) => Val::PyObj(self.def(Expr::Or(l, r))),
                }
            }
            ast::Expression::LogicalEq(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);

                match (left, right) {
                    (Val::Const(li, _), Val::Const(ri, _)) => {
                        Val::Const(if (li != 0) && (ri != 0) { 1 } else { 0 }, true)
                    }
                    (l @ Val::PyObj(_), r @ Val::Const(_, _)) => {
                        let injected = self.inject(r);
                        Val::PyObj(self.def(Expr::PolyEqv(l, injected)))
                    }
                    (l @ Val::Const(_, _), r @ Val::PyObj(_)) => {
                        let injected = self.inject(l);
                        Val::PyObj(self.def(Expr::PolyEqv(injected, r)))
                    }
                    (l @ Val::PyObj(_), r @ Val::PyObj(_)) => {
                        Val::PyObj(self.def(Expr::PolyEqv(l, r)))
                    }
                }
            }
            ast::Expression::LogicalNotEq(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);

                match (left, right) {
                    (Val::Const(li, _), Val::Const(ri, _)) => {
                        Val::Const(if (li != 0) && (ri != 0) { 1 } else { 0 }, true)
                    }
                    (l @ Val::PyObj(_), r @ Val::Const(_, _)) => {
                        let injected = self.inject(r);
                        Val::PyObj(self.def(Expr::PolyUnEqv(l, injected)))
                    }
                    (l @ Val::Const(_, _), r @ Val::PyObj(_)) => {
                        let injected = self.inject(l);
                        Val::PyObj(self.def(Expr::PolyUnEqv(injected, r)))
                    }
                    (l @ Val::PyObj(_), r @ Val::PyObj(_)) => {
                        Val::PyObj(self.def(Expr::PolyUnEqv(l, r)))
                    }
                }
            }
            ast::Expression::List(ref l) => {
                let tmp = self.def(Expr::FunCall(
                    String::from("create_list"),
                    vec![Val::Const(l.len() as i32, false)],
                ));
                let tmp2 = self.def(Expr::FunCall(
                    String::from("inject_big"),
                    vec![Val::PyObj(tmp)],
                ));

                for (i, x) in l.into_iter().enumerate() {
                    let tmp_v = self.flatten_expression(&x);
                    self.def(Expr::FunCall(
                        String::from("set_subscript"),
                        vec![Val::PyObj(tmp2), Val::Const(i as i32, false), tmp_v],
                    ));
                }

                Val::PyObj(tmp2)
            }
            ast::Expression::Dict(ref kvl) => {
                let tmp = self.def(Expr::FunCall(String::from("create_dict"), vec![]));
                let tmp2 = self.def(Expr::FunCall(
                    String::from("inject_big"),
                    vec![Val::PyObj(tmp)],
                ));

                for (_, x) in kvl.into_iter().enumerate() {
                    let (ref tk, ref tv) = *x;
                    let tk_v = self.flatten_expression(&tk);
                    let tv_v = self.flatten_expression(&tv);

                    self.def(Expr::FunCall(
                        String::from("set_subscript"),
                        vec![Val::PyObj(tmp2), tk_v, tv_v],
                    ));
                }

                Val::PyObj(tmp2)
            }
            ast::Expression::Is(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);

                match (left, right) {
                    (Val::Const(li, _), Val::Const(ri, _)) => {
                        Val::Const(if (li != 0) && (ri != 0) { 1 } else { 0 }, true)
                    }
                    (l @ Val::PyObj(_), r @ Val::Const(_, _)) => {
                        let injected = self.inject(r);
                        Val::PyObj(self.def(Expr::Eq(l, injected)))
                    }
                    (l @ Val::Const(_, _), r @ Val::PyObj(_)) => {
                        let injected = self.inject(l);
                        Val::PyObj(self.def(Expr::Eq(injected, r)))
                    }
                    (l @ Val::PyObj(_), r @ Val::PyObj(_)) => Val::PyObj(self.def(Expr::Eq(l, r))),
                }
            }
            ast::Expression::If(ref c, ref t, ref e) => {
                let cv = self.flatten_expression(c);

                if let Val::Const(i, _) = cv {
                    if i == 0 {
                        self.flatten_expression(e)
                    } else {
                        self.flatten_expression(t)
                    }
                } else {
                    let tv = self.flatten_expression(t);
                    let ev = self.flatten_expression(e);
                    Val::PyObj(self.def(Expr::If(cv, tv, ev)))
                }
            }
        }
    }

    pub fn flatten_statement(&mut self, statement: &ast::Statement) {
        match *statement {
            ast::Statement::Print(ref expression) => {
                let val = self.flatten_expression(expression);
                self.print(val);
            }
            ast::Statement::Assign(ref target, ref expression) => {
                let val = self.flatten_expression(&expression);
                match *target {
                    ast::Target::Name(ref s) => {
                        self.names.insert(s.clone(), val);
                    }
                    ast::Target::Subscript(ref t, ref k) => {
                        if let Val::PyObj(_) = val {
                            let vt = self.flatten_expression(t);
                            let vk = self.flatten_expression(k);
                            self.def(Expr::FunCall(
                                String::from("set_subscript"),
                                vec![val, vk, vt],
                            ));
                        } else {
                            panic!("Attempted to subscript a Const Integer: {:?}.", val);
                        }
                    }
                }

            }
            ast::Statement::Expression(ref expression) => {
                let _ = self.flatten_expression(expression);
            }
            ast::Statement::Newline => {}
            ast::Statement::Assign(ast::Target::Subscript(_, _), _) => unimplemented!(),
        }
    }

    pub fn inject(&mut self, v: Val) -> Val {
        if let Val::Const(_, _) = v {
            Val::PyObj(self.def(Expr::Inject(v)))
        } else {
            unreachable!("Attempted to inject a known PyObj {:?}", v)
        }
    }

    pub fn push(&mut self, s: Stmt) {
        self.stack.push(s);
    }

    /// Pushes a def and returns the Tmp reference,
    /// if Expr::Val then just returns self
    /// TODO Refactor this, feels really bad!!!!!1!!1!
    pub fn def(&mut self, expr: Expr) -> Tmp {
        let tmp = self.tmp.alloc().expect("tmp allocator oom");
        let def = Stmt::Def(tmp, expr);
        self.push(def);
        tmp
    }

    pub fn print(&mut self, val: Val) {
        let print = Stmt::Print(val);
        self.push(print);
    }

    pub fn stack(&self) -> &[Stmt] {
        &self.stack
    }
}


