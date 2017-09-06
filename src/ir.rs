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

use ast;
use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    stmts: Vec<Stmt>,
}

impl<'a> From<&'a ast::Program> for Program {
    fn from(program: &'a ast::Program) -> Program {
        let mut builder = Builder::new();
        for statement in &program.module.statements {
            println!("builder: {:#?}", builder);
            builder.flatten_statement(statement);
        }
        Program { stmts: builder.stack }
    }
}

impl From<ast::Program> for Program {
    fn from(program: ast::Program) -> Program {
        let program = &program;
        program.into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Tmp {
    index: usize,
}

/// Tmp(index) -> index of Tmp in stack
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Val {
    Int(i32),
    Ref(Tmp),
}

///
///     tmp8 = tmp7     ; Copy(Tmp)
///     tmp0 = 1        ; Copy(Constant)
///     tmp1 = -1       ; UnaryNeg(Constant)
///     tmp2 = -tmp1    ; UnaryNeg(Tmp)
///     tmp3 = 1 + 2    ; Add(Constant, Constant)
///     tmp4 = 1 + tmp3 ; Add(Constant, Tmp)
///     tmp5 = tmp4 + 1     ; Add(Tmp, Constant)
///     tmp6 = tmp4 + tmp5  ; Add(Tmp, Tmp)
///     tmp7 = input()  ; Input
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Val(Val),
    UnaryNeg(Val),
    Add(Val, Val),
    Input,
}

/// Flattening all possible statements:
///
/// v = n | t
/// e = v | e + e | -e | input()
///
/// print n
/// print t
/// print v + v
/// print v + v + v
///
/// print -v
/// print -e
/// p
///

/// TODO: Determine distinction between `Compute` and `Copy`
///
/// ```text, no_run
///     p0:
///         x = 1
///         y = x 
///         z = y
///
/// ```text, no_run
///     ir:
///         t0 := 1
///         t1 := t0
///         t2 := t1
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    Print(Val),
    Def(Tmp, Expr),
}

#[derive(Debug)]
pub struct Builder {
    stack: Vec<Stmt>,
    names: HashMap<ast::Name, Val>,
    tmp: TmpAllocator,
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

impl Builder {
    pub fn new() -> Builder {
        Builder {
            stack: vec![],
            names: HashMap::new(),
            tmp: TmpAllocator::new(),
        }
    }

    /// Flattens expression into a series of `tN := expr` `Stmt`s,
    /// pushing each onto the stack. The val of the final `Expr`
    /// (either a constant or the reference to the `Stmt` that
    /// gives the result) is returned. In other words, allocation
    /// of `Tmp`s only occurs for expressions that are not just values
    /// (because if the expression is just a `Val`, then the `Val` is
    /// simply returned)
    pub fn flatten_expression(&mut self, expression: &ast::Expression) -> Expr {
        match *expression {
            ast::Expression::DecimalI32(ast::DecimalI32(i)) => Expr::Val(Val::Int(i)),
            ast::Expression::Name(ref name) => {
                match self.names.get(name) {
                    Some(&val) => Expr::Val(val),
                    None => panic!("reference to undefined name {:?}", name),
                }
            }
            ast::Expression::Input(_) => {
                Expr::Input
            }
            ast::Expression::UnaryNeg(ref expr) => {
                let expr = self.flatten_expression(expr);
                let val = self.push_def(expr);
                Expr::UnaryNeg(val)
            }
            ast::Expression::Add(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let left = self.push_def(left);
                let right = self.flatten_expression(right);
                let right = self.push_def(right);
                Expr::Add(left, right)
            }
        }
    }

    pub fn flatten_statement(&mut self, statement: &ast::Statement) {
        match *statement {
            ast::Statement::Print(ref expression) => {
                let expr = self.flatten_expression(expression);
                let val = self.push_def(expr);
                self.print(val);
            }
            ast::Statement::Assign(ref name, ref expression) => {
                let expr = self.flatten_expression(expression);
                let val = self.push_copy(expr);
                self.names.insert(name.clone(), val);
            }
            ast::Statement::Expression(ref expression) => {
                let expr = self.flatten_expression(expression);
                self.push_def(expr);
            }
            ast::Statement::Newline => {}
        }
    }

    pub fn push(&mut self, s: Stmt) {
        self.stack.push(s);
    }

    /// Pushes a def and returns the Tmp reference,
    /// if Expr::Val then just returns self
    /// TODO Refactor this, feels really bad!!!!!1!!1!
    pub fn push_def(&mut self, expr: Expr) -> Val {
        let expr = match expr {
            Expr::Val(val) => return val,
            e => e,
        };
        let tmp = self.tmp.alloc().expect("tmp allocator oom");
        let def = Stmt::Def(tmp, expr);
        self.push(def);
        Val::Ref(tmp)
    }

    pub fn push_copy(&mut self, expr: Expr) -> Val {
        let tmp = self.tmp.alloc().expect("tmp allocator oom");
        let def = Stmt::Def(tmp, expr);
        self.push(def);
        Val::Ref(tmp)
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
            static ref EXPR: Regex = Regex::new(r"(t\d+|-?\d+)\s+\+\s+(t\d+|-?\d+)|(t\d+|-?\d+)|-(t\d+|-?\d+)|(input\(\))").unwrap();
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
                    parse_tmp(s).map(Val::Ref)
                })
                .or(captures.get(2).ok_or(()).and_then(|m| {
                    let s = m.as_str();
                    s.parse::<i32>().map_err(|_| ()).map(Val::Int)
                }))
        }
        fn parse_expr(s: &str) -> Result<Expr, ()> {
            println!("s = {:?}", s);
            let captures = EXPR.captures(s).ok_or(())?;
            if let (Some(l), Some(r)) = (captures.get(1), captures.get(2)) {
                let l = parse_val(l.as_str())?;
                let r = parse_val(r.as_str())?;
                Ok(Expr::Add(l, r))
            } else if let Some(m) = captures.get(3) {
                let s = m.as_str();
                parse_val(s).map(Expr::Val)
            } else if let Some(m) = captures.get(4) {
                let s = m.as_str();
                parse_val(s).map(Expr::UnaryNeg)
            } else if let Some(_) = captures.get(5) {
                Ok(Expr::Input)
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stmts = vec![];
        for line in s.lines() {
            println!("line: {:?}", line);
            if line.is_empty() {
                continue;
            }
            stmts.push(line.parse::<Stmt>()?);
        }
        Ok(Program { stmts })
    }
}

#[cfg(test)]
mod test {
    use ir;

    // ir syntax:
    //
    // ```text, no_run
    //      t0 = 1;
    //      print t0

    macro_rules! test {
        ($p0:expr => $ir:expr) => ({
            use $crate::ast::Parse;

            let program = $p0.parse_program().unwrap();
            let ir: $crate::ir::Program = program.into();
            let expected = $ir.parse::<ir::Program>().unwrap();
            assert_eq!(ir, expected, "generated ir {:#?} does not equal expected {:#?}", ir, expected);
        })
    }

    #[test]
    fn print_int() {
        test!("print 1 + 2\n" => "t0 := 1 + 2\nprint t0");
    }

    #[test]
    fn assign_int() {
        test!("rust_python = 33\n" => "t0 := 33");
    }

    #[test]
    fn assign_name() {
        test!("x = 1\ny = x" => "t0 := 1\nt1 := t0");
    }

    #[test]
    fn assign_input() {
        test!("x = input()" => "t0 := input()");
    }

    #[test]
    fn assign_add() {
        test!("x = 1 + 2" => "t0 := 1 + 2");
    }
}
