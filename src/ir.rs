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
            builder.flatten_statement(statement);
        }
        Program {
            stmts: builder.stack,
        }
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
    Copy(Val),
    UnaryNeg(Val),
    Add(Val, Val),
    Input,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    Print(Val),
    Def(Tmp, Expr),
}

#[derive(Debug)]
pub struct Builder {
    stack: Vec<Stmt>,
    names: HashMap<ast::Name, Tmp>,
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

    pub fn flatten_expression(&mut self, expression: &ast::Expression) -> Val {
        match *expression {
            ast::Expression::DecimalI32(ast::DecimalI32(i)) => Val::Int(i),
            ast::Expression::Name(ref name) => {
                let tmp = match self.names.get(name) {
                    Some(&tmp) => tmp,
                    None => panic!("reference to undefined name {:?}", name),
                };
                Val::Ref(tmp)
            }
            ast::Expression::Input(_) => {
                let tmp = self.def(Expr::Input);
                Val::Ref(tmp)
            }
            ast::Expression::UnaryNeg(ref expr) => {
                let val = self.flatten_expression(expr);
                let tmp = self.def(Expr::UnaryNeg(val));
                Val::Ref(tmp)
            }
            ast::Expression::Add(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);
                let tmp = self.def(Expr::Add(left, right));
                Val::Ref(tmp)
            }
        }
    }

    pub fn flatten_statement(&mut self, statement: &ast::Statement) {
        match *statement {
            ast::Statement::Print(ref expression) => {
                let val = self.flatten_expression(expression);
                self.print(val);
            }
            ast::Statement::Assign(ref name, ref expression) => {
                let val = self.flatten_expression(expression);
                let copy = Expr::Copy(val);
                let tmp = self.def(copy);
                self.names.insert(name.clone(), tmp);
            }
            ast::Statement::Expression(ref expression) => {
                let _ = self.flatten_expression(expression);
            }
            ast::Statement::Newline => {}
        }
    }

    pub fn push(&mut self, s: Stmt) {
        self.stack.push(s);
    }

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
            static ref EXPR: Regex = Regex::new(r"(t\d+|-?\d+)\s+\+\s+(t\d+|-?\d+)|(t\d+|-?\d+)|-(t\d+|-?\d+)").unwrap();
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
            captures.get(1).ok_or(()).and_then(|m| {
                let s = m.as_str();
                parse_tmp(s).map(Val::Ref)
            }).or(captures.get(2).ok_or(()).and_then(|m| {
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
                parse_val(s).map(Expr::Copy)
            } else if let Some(m) = captures.get(4) {
                let s = m.as_str();
                parse_val(s).map(Expr::UnaryNeg)
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
            if line.is_empty() { continue }
            stmts.push(line.parse::<Stmt>()?);
        }
        Ok(Program { stmts })
    }
}

#[cfg(test)]
mod test {
    use ir;
    use ast::Parse;

    // ir syntax:
    //
    // ```text, no_run
    //      t0 = 1;
    //      print t0
    //
    //
    //

    #[test]
    fn print_int() {
        let program = "print 1 + 2\n".parse_program().unwrap();
        let ir: ir::Program = program.into();

        let program = "
t0 := 0
t1 := t0 + -1
print t1
t2 := -t1
print t2
";
        println!("program: {:#?}", program.parse::<ir::Program>());
    }
}
