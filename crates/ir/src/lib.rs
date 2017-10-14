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

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate python_ast as ast;

use std::str::FromStr;
use std::collections::HashMap;
use std::fmt;
use regex::Regex;

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
            ConstInt(i) => write!(f, "{}", i),
            ConstBool(b) => write!(f, "{}", b),
            Any(tmp) => write!(f, "{}:any", tmp),
            Int(tmp) => write!(f, "{}:int", tmp),
            Bool(tmp) => write!(f, "{}:bool", tmp),
            List(tmp) => write!(f, "{}:list", tmp),
            Dict(tmp) => write!(f, "{}:dict", tmp),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self {
            UnaryNeg(ref val) => write!(f, "-{}", val),
            Add(ref l, ref r) => write!(f, "{} + {}", l, r),
            FunCall(ref label) => write!(f, "{label}()", label=label),
            _ => unimplemented!(),
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


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Bool,
    List,
    Dict
}

impl Type {
    fn is_big(self) -> bool {
        match self {
            Type::Int => false,
            Type::Bool => false,
            Type::List => true,
            Type::Dict => true,
        }
    }
}

/// Tmp(index) -> index of Tmp in stack
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Val {
    ConstInt(i32),
    ConstBool(bool),
    Any(Tmp),
    Int(Tmp),
    Bool(Tmp),
    List(Tmp),
    Dict(Tmp),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    UnaryNeg(Val),
    Add(Val, Val),
    PolyEqv(Val, Val),
    Not(Val),
    Eq(Val, Val),
    NotEq(Val, Val),
    And(Val, Val),
    Or(Val,Val),
    If(Val,Val,Val),
    FunCall(String),
    Subscript(Val, Val),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    Print(Val),
    Def(Tmp, Expr),
    TypeAssert(Tmp, Type),
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
            ast::Expression::Target(ast::Target::Subscript(_, _)) => {
                unimplemented!("Target Subscript!")
            },
            ast::Expression::DecimalI32(i) => Val::ConstInt(i),
            ast::Expression::Boolean(b) => Val::ConstBool(b),
            ast::Expression::Input => {
                let tmp = self.def(Expr::FunCall(String::from("input")));
                Val::Int(tmp)
            },
            ast::Expression::UnaryNeg(ref expr) => {
                match self.flatten_expression(expr) {
                    v @ Val::Int(_) => {
                        Val::Int(self.def(Expr::UnaryNeg(v)))
                    },
                    Val::ConstInt(i) => {
                        Val::ConstInt(-i)
                    },
                    v @ Val::Any(_) => {
                        let int = self.typed(v, Type::Int);
                        Val::Int(self.def(Expr::UnaryNeg(int)))
                    }
                    _ => unimplemented!()
                }
            },
            ast::Expression::Add(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);
                let tmp = self.def(Expr::Add(left, right));
                Val::Int(tmp)
            },
            ast::Expression::LogicalNot(ref expr) => {
                let val = self.flatten_expression(expr);
                let tmp = self.def(Expr::Not(val));
                Val::Int(tmp)
            },
            ast::Expression::LogicalAnd(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);
                let tmp = self.def(Expr::And(left, right));
                Val::Int(tmp)
            },
            ast::Expression::LogicalOr(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);
                let tmp = self.def(Expr::Or(left, right));
                Val::Int(tmp)
            },
            ast::Expression::LogicalEq(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);
                let tmp = self.def(Expr::PolyEqv(left, right));
                Val::Int(tmp)
            },
            ast::Expression::LogicalNotEq(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);
                let tmp = self.def(Expr::And(left, right));
                Val::Int(tmp)
            },
            ast::Expression::List(ref l) => {
                unimplemented!()
            },
            ast::Expression::Dict(ref kvl) => {
                unimplemented!()
            },
            ast::Expression::Is(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);
                let tmp = self.def(Expr::Eq(left, right));
                Val::Int(tmp)
            },
        }
    }

    pub fn flatten_statement(&mut self, statement: &ast::Statement) {
        match *statement {
            ast::Statement::Print(ref expression) => {
                let val = self.flatten_expression(expression);
                self.print(val);
            }
            ast::Statement::Assign(ast::Target::Name(ref name), ref expression) => {
                let val = self.flatten_expression(expression);
                self.names.insert(name.clone(), val);
            }
            ast::Statement::Expression(ref expression) => {
                let _ = self.flatten_expression(expression);
            }
            ast::Statement::Newline => {}
            ast::Statement::Assign(ast::Target::Subscript(_, _), _) => unimplemented!(),
        }
    }

    pub fn typed(&mut self, v : Val, ty : Type) -> Val {
        match v {
            Val::Any(tmp) => {
                self.push(Stmt::TypeAssert(tmp, ty));
                match ty {
                    Type::Int => Val::Int(tmp),
                    Type::Bool => Val::Bool(tmp),
                    Type::Dict => Val::Dict(tmp),
                    Type::List => Val::List(tmp)
                }
            },
            _ => unreachable!("typed() called with non-any, only reachable from any match")
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

impl FromStr for Stmt {
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
                    s.parse::<i32>().map_err(|_| ()).map(Val::ConstInt)
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
                Ok(Expr::FunCall("input".into()))
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
}
