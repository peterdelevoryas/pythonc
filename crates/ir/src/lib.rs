//!
//! Below is a running example of how I think p0 should
//! translate to the IR in this module:
//!
//! ```text, no_run
//!     x = 1 + 2
//!     y = 3 + 4
//!     print x + y
//!     z = -x + 1 + y
//!     print z
//!
//!
//!
//!
//!
//! In general, here are the mappings between `ast::Statement`s and `ir::Statement`s
//!
//!     Expression::DecimalI32(value) -> {
//!         tmpN = value
//!     }
//!
//!     Assign(Name, Expr) -> {
//!         tmpN = ir(expr)
//!     }
//!

extern crate lazy_static;
extern crate regex;
extern crate python_ast as ast;

use std::collections::HashMap;
use std::fmt;

pub fn debug_print<'ir, I: Iterator<Item = &'ir Stmt>>(stmts: I) {
    for (k, stmt) in stmts.enumerate() {
        println!("{:<3} {}", k, stmt);
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Stmt::*;
        match *self {
            Print(ref val) => write!(f, "print {}", val),
            Def(tmp, ref expr) => write!(f, "{} := {}", tmp, expr),
            // type assert
            TypeAssert(_, _) => unimplemented!()
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Val::*;
        match *self {
            Const(i, b) => write!(f, "Const({}, {})", i, b),
            PyObj(tmp) => write!(f, "{}", tmp),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self {
            UnaryNeg(ref val) => write!(f, "-{}", val),
            Add(ref l, ref r) => write!(f, "{} + {}", l, r),
            _ => unimplemented!("Expr Display is bad")
        }
    }
}

impl fmt::Display for Tmp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "t{}", self.index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tmp {
    pub index: usize,
}

// Tmp(index) -> index of Tmp in stack
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Val {
    Const(i32, bool),
    PyObj(Tmp)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    UnaryNeg(Val),
    Add(Val, Val),
    PolyEqv(Val, Val),
    Not(Val),
    Eq(Val, Val),
    PolyUnEqv(Val, Val),
    And(Val, Val),
    Or(Val,Val),
    If(Val,Val,Val),
    FunCall(String, Vec<Val>),
    Subscript(Val, Val),
    Inject(Val),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    Print(Val),
    Def(Tmp, Expr),
}

#[derive(Debug)]
pub struct Builder<'alloc> {
    stack: Vec<Stmt>,
    names: HashMap<String, Val>,
    tmp: &'alloc mut TmpAllocator,
}

impl Tmp {
    pub fn new(index: usize) -> Tmp {
        Tmp { index }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Debug)]
pub struct TmpAllocator {
    next: usize,
}

impl TmpAllocator {
    pub fn new() -> TmpAllocator {
        TmpAllocator { next: 0 }
    }

    pub fn alloc(&mut self) -> Option<Tmp> {
        let tmp = Tmp { index: self.next };
        self.next = match self.next.checked_add(1) {
            Some(sum) => sum,
            None => return None,
        };
        Some(tmp)
    }
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
            },
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
                    Val::PyObj(self.def(Expr::FunCall(String::from("get_subscript"), vec![tgt, tmp])))
                } else {
                    panic!("Attempted subscript of illegal val {:?}", tgt)
                }
            },
            ast::Expression::DecimalI32(i) => Val::Const(i, false),
            ast::Expression::Boolean(b) => Val::Const(if b { 1 } else { 0 }, true),
            ast::Expression::Input => {
                Val::PyObj(self.def(Expr::FunCall(String::from("input"), vec![])))
            },
            ast::Expression::UnaryNeg(ref expr) => {
                match self.flatten_expression(expr) {
                    Val::Const(i, _) => {
                        Val::Const(-i, false)
                    },
                    v @ Val::PyObj(_) => {
                        Val::PyObj(self.def(Expr::UnaryNeg(v)))
                    }
                    _ => unimplemented!()
                }
            },
            ast::Expression::Add(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);

                match (left,right) {
                    (Val::Const(li, _), Val::Const(ri, _)) => {
                        Val::Const(li + ri, false)
                    }
                    (l @ Val::PyObj(_), r @ Val::Const(_, _)) => {
                        let injected = self.inject(r);
                        Val::PyObj(self.def(Expr::Add(l, injected)))
                    }
                    (l @ Val::Const(_, _), r @ Val::PyObj(_)) => {
                        let injected = self.inject(l);
                        Val::PyObj(self.def(Expr::Add(injected, r)))
                    }
                    (l @ Val::PyObj(_), r @ Val::PyObj(_)) => {
                        Val::PyObj(self.def(Expr::Add(l, r)))
                    }
                }
            },
            ast::Expression::LogicalNot(ref expr) => {
                let val = self.flatten_expression(expr);
                match val {
                    Val::Const(i, _) => Val::Const(if i != 0 { 1 } else { 0 }, true),
                    v @ Val::PyObj(_) => Val::PyObj(self.def(Expr::Not(v)))
                }
            },
            ast::Expression::LogicalAnd(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);

                match (left,right) {
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
                    (l @ Val::PyObj(_), r @ Val::PyObj(_)) => {
                        Val::PyObj(self.def(Expr::And(l, r)))
                    }
                }
            },
            ast::Expression::LogicalOr(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);

                match (left,right) {
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
                    (l @ Val::PyObj(_), r @ Val::PyObj(_)) => {
                        Val::PyObj(self.def(Expr::Or(l, r)))
                    }
                }
            },
            ast::Expression::LogicalEq(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);

                match (left,right) {
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
            },
            ast::Expression::LogicalNotEq(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);

                match (left,right) {
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
            },
            ast::Expression::List(ref l) => {
                let tmp = self.def(Expr::FunCall(String::from("create_list"), vec![Val::Const(l.len() as i32, false)]));
                let tmp2 = self.def(Expr::FunCall(String::from("inject_big"), vec![Val::PyObj(tmp)]));

                for (i,x) in l.into_iter().enumerate() {
                    let tmp_v = self.flatten_expression(&x);
                    self.def(Expr::FunCall(String::from("set_subscript"), vec![Val::PyObj(tmp2), Val::Const(i as i32, false), tmp_v]));
                }

                Val::PyObj(tmp2)
            },
            ast::Expression::Dict(ref kvl) => {
                let tmp = self.def(Expr::FunCall(String::from("create_dict"), vec![]));
                let tmp2 = self.def(Expr::FunCall(String::from("inject_big"), vec![Val::PyObj(tmp)]));

                for (_, x) in kvl.into_iter().enumerate() {
                    let (ref tk, ref tv) = *x;
                    let tk_v = self.flatten_expression(&tk);
                    let tv_v = self.flatten_expression(&tv);

                    self.def(Expr::FunCall(String::from("set_subscript"), vec![Val::PyObj(tmp2), tk_v, tv_v]));
                }

                Val::PyObj(tmp2)
            },
            ast::Expression::Is(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);

                match (left,right) {
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
                    (l @ Val::PyObj(_), r @ Val::PyObj(_)) => {
                        Val::PyObj(self.def(Expr::Eq(l, r)))
                    }
                }
            },
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
                    },
                    ast::Target::Subscript(ref t, ref k) => {
                        if let Val::PyObj(_) = val {
                            let vt = self.flatten_expression(t);
                            let vk = self.flatten_expression(k);
                            self.def(Expr::FunCall(String::from("set_subscript"), vec![val, vk, vt]));
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

    pub fn inject(&mut self, v : Val) -> Val {
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

/*impl FromStr for Stmt {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref DEF: Regex = Regex::new(r"(t\d+)\s+:=\s+(.+)").unwrap();
            static ref PRINT: Regex = Regex::new(r"print\s+([[:alnum:]-]+)").unwrap();
            static ref VAL: Regex = Regex::new(r"(t\d+)|(-?\d+)").unwrap();
            static ref TMP: Regex = Regex::new(r"t(\d+)").unwrap();
            static ref EXPR: Regex = Regex::new(r"(t\d+|-?\d+)\s+\+\s+(t\d+|-?\d+)|(-t\d+|-?\d+)|(input\(\))|(\([^\)]+\))").unwrap();
        }
        fn parse_def(s: &str) -> Result<(Tmp, Expr), ()> {
            let captures = DEF.captures(s).ok_or(())?;
            let tmp = parse_tmp(captures.get(1).ok_or(())?.as_str())?;
            let expr = parse_expr(captures.get(2).ok_or(())?.as_str())?;
            Ok((tmp, expr))
        }
        fn parse_tmp(s: &str) -> Result<Tmp, ()> {
            let captures = TMP.captures(s).ok_or(())?;
            let index: usize = captures.get(1).ok_or(())?.as_str().parse().map_err(|_| ())?;
            let tmp = Tmp { index };
            Ok(tmp)
        }
        fn parse_val(s: &str) -> Result<Val, ()> {
            let captures = VAL.captures(s).ok_or(())?;
            captures
                .get(1)
                .ok_or(())
                .and_then(|m| {
                    let s = m.as_str();
                    parse_tmp(s).map(Val::Int)
                })
                .or(captures.get(2).ok_or(()).and_then(|m| {
                    let s = m.as_str();
                    s.parse::<i32>().map_err(|_| ()).map(|x| Val::Const(x, false))
                }))
        }
        fn parse_expr(s: &str) -> Result<Expr, ()> {
            //println!("s = {:?}", s);
            let captures = EXPR.captures(s).ok_or(())?;
            if let (Some(l), Some(r)) = (captures.get(1), captures.get(2)) {
                let l = parse_val(l.as_str())?;
                let r = parse_val(r.as_str())?;
                Ok(Expr::Add(l, r))
            } else if let Some(m) = captures.get(3) {
                let s = m.as_str();
                parse_val(s).map(Expr::UnaryNeg)
            } else if let Some(_) = captures.get(4) {
<<<<<<< HEAD
                Ok(Expr::FunCall("input".into()))
=======
                Ok(Expr::FunCall(String::from("input"), vec![]))
>>>>>>> 525c2681899d1326d1052dd98b0a6085e4843f0b
            } else if let Some(m) = captures.get(5) {
                let s = m.as_str();
                parse_expr(&s[1..s.len() - 1])
            } else {
                Err(())
            }
        }
        fn parse_print(s: &str) -> Result<Val, ()> {
            let captures = PRINT.captures(s).ok_or(())?;
            let val_str = captures.get(1).ok_or(())?.as_str();
            parse_val(val_str)
        }
        match parse_def(s) {
            Ok((tmp, expr)) => return Ok(Stmt::Def(tmp, expr)),
            _ => {}
        }
        match parse_print(s) {
            Ok(val) => return Ok(Stmt::Print(val)),
            _ => {}
        }
        Err(())
    }
}

impl FromStr for Program {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stmts = vec![];
        for line in s.lines() {
            //println!("line: {:?}", line);
            if line.is_empty() {
                continue;
            }
            stmts.push(line.parse::<Stmt>().map_err(
                |_| format!("parse error on {:?}", line),
            )?);
        }
        Ok(Program { stmts })
    }
}

#[cfg(test)]
mod test {
    extern crate python_token as token;
    use ast;

    macro_rules! test {
        ($p0:expr => $ir:expr) => ({
            let tok_stream = token::Stream::new($p0);
            let program = ast::parse_program(tok_stream).expect("ast::parse_program");
            let ir: $crate::Program = program.into();
            let expected = $ir.parse::<$crate::Program>().expect("ir program");
            assert_eq!(ir, expected, "generated ir {:#?} does not equal expected {:#?}", ir, expected);
        })
    }

    #[test]
    fn print_val() {
        test!("print -2\nx = 11\nprint x" => "print -2\nprint 11");
    }

    #[test]
    fn assign_int() {
        test!("rust_python = 33\n" => "");
    }

    #[test]
    fn assign_name() {
        test!("x = 1\ny = x" => "");
    }

    #[test]
    fn assign_input() {
        test!("x = input()" => "t0 := input()");
    }

    #[test]
    fn assign_add() {
        test!("x = 1 + 2" => "t0 := 1 + 2");
    }

    #[test]
    fn assign_unary_neg() {
        test!("x = --1\nx = -x" => "t0 := --1\nt1 := -t0");
    }

    #[test]
    fn assign_compound() {
        test!("x = 1 + 2 + 3" => "t0 := 1 + 2\nt1 := t0 + 3");
        test!("x = 1 + input()" => "t0 := input()\nt1 := 1 + t0");
        test!("x = input() + 1 + 2" => "t0 := input()\nt1 := t0 + 1\nt2 := t1 + 2");
        test!("x = 1 + 2\ny = x + 1" => "t0 := 1 + 2\nt1 := t0 + 1");
        test!("x = 2\ny = x + -2 + 1 + input()" => "\
            t0 := 2 + -2 \n\
            t1 := t0 + 1 \n\
            t2 := input() \n\
            t3 := t1 + t2");
    }

    #[test]
    fn print_add() {
        test!("print 1 + 2" => "t0 := 1 + 2\nprint t0");
        test!("x = 2\ny = x\nprint x + y + 1" => "t0 := 2 + 2\nt1 := t0 + 1\nprint t1");
    }

    #[test]
    fn print_unary_neg() {
        test!("print --1" => "t0 := --1\nprint t0");
        test!("x = 33\nprint -x" => "t0 := -(33)\nprint t0");
        test!("y = -33\nprint -(y + 22 + input())" => "\
               t0 := -33 + 22 \n\
               t1 := input() \n\
               t2 := t0 + t1 \n\
               t3 := -t2 \n\
               print t3");
    }

    #[test]
    fn print_input() {
        test!("print input()" => "t0 := input()\nprint t0");
    }
}*/
