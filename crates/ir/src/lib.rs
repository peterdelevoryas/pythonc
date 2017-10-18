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
#![feature(slice_concat_ext)]

extern crate lazy_static;
extern crate regex;
extern crate python_ast as ast;

pub mod tmp;
pub mod value;
pub mod expression;
pub mod statement;
pub mod program;
pub mod builder;

pub use tmp::Tmp;
pub use tmp::Allocator as TmpAllocator;
pub use value::Val;
pub use expression::Expr;
pub use statement::Stmt;
pub use program::Program;
pub use builder::Builder;

// here lies ye old forgotten test code
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
