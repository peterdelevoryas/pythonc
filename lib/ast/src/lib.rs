#![feature(box_syntax, box_patterns)]
extern crate lalrpop_util;
extern crate regex;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate util;

use error::*;
use lalrpop_util::ParseError;
use std::str;

pub mod parser;
pub mod error;

pub type LalrpopParseError<'input> = ParseError<usize, (usize, &'input str), Error>;

impl_wrapper_enum! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Expr {
        boxed: [
            Add, UnarySub,
            CallFunc, Compare,
            Or, And, Not,
            List, Dict,
            Subscript,
            Lambda,
            IfExp
        ];
        simple: [
            Const, Name
        ];
    }
}

impl_wrapper_enum! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Stmt {
        boxed: [];
        simple: [
            Function,
            Class,
            If,
            While,
            Printnl,
            Assign,
            Expr,
            Return
        ];
    }
}

impl_wrapper_enum! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Target {
        boxed: [];
        simple: [
            Name, Subscript
        ];
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Const(pub i32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Add {
    pub left: Expr,
    pub right: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnarySub {
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallFunc {
    // specified in source as "node", but should
    // probably not be like a Module or Stmt right,
    // really should just be an expression.
    pub expr: Expr,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Printnl {
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign {
    pub target: Target,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompareOp {
    Eq,
    NotEq,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compare {
    pub op: CompareOp,
    pub left: Expr,
    pub right: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Or {
    pub left: Expr,
    pub right: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct And {
    pub left: Expr,
    pub right: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Not {
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    pub exprs: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dict {
    pub tuples: Vec<(Expr, Expr)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subscript {
    pub base: Expr,
    pub elem: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub name: String,
    pub bases: Vec<Name>,
    pub code: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct While {
    pub test: Expr,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If {
    pub cond: Expr,
    pub then: Vec<Stmt>,
    pub else_: Option<Vec<Stmt>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfExp {
    pub cond: Expr,
    pub then: Expr,
    pub else_: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub code: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lambda {
    pub args: Vec<String>,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Return {
    pub expr: Option<Expr>,
}

pub struct Parser {
    _private: (),
}

impl Parser {
    pub fn new() -> Parser {
        Parser { _private: () }
    }

    pub fn parse(&self, source: &str) -> Result<Module> {
        use std::process::Command;
        use std::process::Stdio;
        use std::io::Write;

        let mut parser = Command::new("python")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .chain_err(|| "Could not spawn python")?;
        {
            let stdin = parser.stdin.as_mut().ok_or(
                "failed to capture python stdin",
            )?;
            let program = print_parsed(source);
            stdin.write_all(program.as_bytes()).chain_err(
                || "Unable to write to python stdin",
            )?;
        }
        let output = parser.wait_with_output().chain_err(
            || "Unable to capture python stdout",
        )?;
        let output = String::from_utf8(output.stdout).chain_err(
            || "python stdout is not utf-8",
        )?;
        use error::ResultExt;
        let module = parser::parse_module(&output).map_err(|e| {
            Error::from(ErrorKind::Msg(lalrpop_parse_error_to_str(source, e).into()))
        }).chain_err(|| {
            format!("Python repr: {}", output)
        })?;

        Ok(module)
    }
}

/// Returns a Python program that will
/// print the parsed repr of the source
/// str.
fn print_parsed(source: &str) -> String {
    format!(
        "\
import compiler
import sys

source = {:?}
parsed = compiler.parse(source)
print parsed
",
        source
    )
}

fn lalrpop_parse_error_to_str<'input>(source: &str, e: LalrpopParseError<'input>) -> String {
    use lalrpop_util::ParseError::*;
    match e {
        InvalidToken { location } => {
            format!("Invalid token: {:?}", {
                str::from_utf8(&source.as_bytes()[location..location + 16]).unwrap()
            })
        }
        UnrecognizedToken { token, expected } => {
            let token = token.map(|(_, t, _)| t);
            format!(
                "Unrecognized token {:?}, expected one of {:?}",
                token,
                expected
            )
        }
        ExtraToken { token } => {
            let (_, token, _) = token;
            format!("Extra token: {:?}", token)
        }
        User { error } => format!("User error: {}", error),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! parse_tests {
        (
            $wrapper_parse_fn:ident tests {
                $(
                    test $parse_fn:ident {
                        source: $source:expr;
                        expect: $expect:expr;
                    }
                )*
            }
        ) => {
            mod $wrapper_parse_fn {
                use super::*;
                $(
                    #[test]
                    fn $parse_fn() {
                        let source = $source;
                        let expect = $expect;
                        let result = ::parser::$parse_fn(source).unwrap();
                        assert_eq!(result, expect);
                    }
                )*

                #[test]
                fn $wrapper_parse_fn() {
                    $(
                        let source = $source;
                        let expect = ($expect).into();
                        let result = ::parser::$wrapper_parse_fn(source).unwrap();
                        assert_eq!(result, expect);
                    )*
                }
            }
        }
    }

    parse_tests! {
        parse_expr tests {
            test parse_const {
                source: "Const(1)";
                expect: Const(1);
            }

            test parse_name {
                source: "Name('x')";
                expect: Name("x".into());
            }

            test parse_add {
                source: "Add((Const(2), Const(2)))";
                expect: Add {
                    left: Expr::Const(Const(2)),
                    right: Expr::Const(Const(2)),
                };
            }

            test parse_unary_sub {
                source: "UnarySub(Const(1))";
                expect: UnarySub {
                    expr: Expr::Const(Const(1)),
                };
            }

            test parse_call_func {
                source: "CallFunc(Name('add_args'), [Const(1), Const(2), Const(3)], None, None)";
                expect: CallFunc {
                    expr: Expr::Name(Name("add_args".into())),
                    args: vec![
                        Expr::Const(Const(1)),
                        Expr::Const(Const(2)),
                        Expr::Const(Const(3)),
                    ],
                };
            }

            test parse_discard {
                source: "Discard(Const(1))";
                expect: Expr::Const(Const(1));
            }

            test parse_compare {
                source: "Compare(Const(1), [('!=', Const(2))])";
                expect: Compare {
                    op: CompareOp::NotEq,
                    left: Expr::Const(Const(1)),
                    right: Expr::Const(Const(2)),
                };
            }

            test parse_or {
                source: "Or([Const(1), Const(2)])";
                expect: Or {
                    left: Expr::Const(Const(1)),
                    right: Expr::Const(Const(2)),
                };
            }

            test parse_and {
                source: "And([Const(1), Name('True')])";
                expect: And {
                    left: Expr::Const(Const(1)),
                    right: Expr::Name(Name("True".into())),
                };
            }

            test parse_not {
                source: "Not(Name('False'))";
                expect: Not {
                    expr: Expr::Name(Name("False".into())),
                };
            }

            test parse_list {
                source: "List([Const(1), Name('True')])";
                expect: List {
                    exprs: vec![
                        Expr::Const(Const(1)),
                        Expr::Name(Name("True".into())),
                    ],
                };
            }

            test parse_dict {
                source: "Dict([(Const(1), Const(2)), (Name('True'), Name('False'))])";
                expect: Dict {
                    tuples: vec![
                        (Expr::Const(Const(1)), Expr::Const(Const(2))),
                        (Expr::Name(Name("True".into())), Expr::Name(Name("False".into()))),
                    ],
                };
            }

            test parse_subscript {
                source: "Subscript(Name('x'), 'OP_APPLY', [Const(1)])";
                expect: Subscript {
                    base: Expr::Name(Name("x".into())),
                    elem: Expr::Const(Const(1)),
                };
            }

            test parse_if_exp {
                source: "IfExp(Name('True'), Const(1), Const(2))";
                expect: IfExp {
                    cond: Expr::Name(Name("True".into())),
                    then: Expr::Const(Const(1)),
                    else_: Expr::Const(Const(2)),
                };
            }

            test parse_lambda {
                source: "Lambda(['x'], [], 0, Add((Name('x'), Const(1))))";
                expect: Lambda {
                    args: vec!["x".into()],
                    expr: Add {
                        left: Name("x".into()).into(),
                        right: Const(1).into(),
                    }.into()
                };
            }
        }
    }

    parse_tests! {
        parse_stmt tests {
            test parse_return {
                source: "Return(Name('x'))";
                expect: Return {
                    expr: Some(Name("x".into()).into()),
                };
            }

            test parse_expr {
                source: "Name('x')";
                expect: Expr::Name(Name("x".into()));
            }

            test parse_printnl {
                source: "Printnl([Const(1)], None)";
                expect: Printnl {
                    expr: Expr::Const(Const(1)),
                };
            }

            test parse_assign {
                source: "Assign([AssName('x', 'OP_ASSIGN')], Const(1))";
                expect: Assign {
                    target: Target::Name(Name("x".into())),
                    expr: Expr::Const(Const(1)),
                };
            }

            test parse_function {
                source: "Function(None, 'f', ['x'], [], 0, None, Stmt([Printnl([Const(1)], None)]))";
                expect: Function {
                    name: "f".into(),
                    args: vec!["x".into()],
                    code: vec![
                        Stmt::Printnl(Printnl {
                            expr: Expr::Const(Const(1)),
                        }),
                    ],
                };
            }
        }
    }

    parse_tests! {
        parse_target tests {
            test parse_ass_name {
                source: "AssName('x', 'OP_ASSIGN')";
                expect: Name("x".into());
            }

            test parse_name {
                source: "Name('x')";
                expect: Name("x".into());
            }

            test parse_subscript {
                source: "Subscript(Name('x'), 'OP_APPLY', [Const(1)])";
                expect: Subscript {
                    base: Expr::Name(Name("x".into())),
                    elem: Expr::Const(Const(1)),
                };
            }
        }
    }

    #[test]
    fn parse_module() {
        let source = "Module(None, Stmt([Discard(Const(1)), Printnl([Const(1)], None)]))";
        let expect = Module {
            stmts: vec![
                Stmt::Expr(Expr::Const(Const(1))),
                Stmt::Printnl(Printnl { expr: Expr::Const(Const(1)) }),
            ],
        };
        let result = parser::parse_module(source).unwrap();
        assert_eq!(result, expect);
    }
}
