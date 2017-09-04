//!
//! AST for p0
//!
//! Context-free grammar, slightly modified from Course Notes (2.2), Figure 5
//! to include newlines as tokens
//!
//! ```text, no_run
//!     program ::= module
//!     module ::= statements
//!     statements ::= statement*
//!     statement ::= "print" expression | name "=" expression | expression | "\n"
//!     expression ::= expression "+" term
//!                  | term
//!     term ::= name
//!            | decimal_i32
//!            | "-" term
//!            | "(" expression ")"
//!            | "input" "(" ")"
//!     decimal_i32 ::= "-"? nonzero_decimal_digit decimal_digit* | "-"? "0"
//!     name ::= name_first_char name_char*
//!     name_first_char ::= letter | "_"
//!     name_char ::= letter | "_" | decimal_digit
//!     letter ::= lowercase | uppercase
//!     lowercase ::= "a" | "b" | "c" | "d" | "e" | "f"
//!                 | "g" | "h" | "i" | "j" | "k" | "l"
//!                 | "m" | "n" | "o" | "p" | "q" | "r"
//!                 | "s" | "t" | "u" | "v" | "w" | "x"
//!                 | "y" | "z"
//!     uppercase ::= "A" | "B" | "C" | "D" | "E" | "F"
//!                 | "G" | "H" | "I" | "J" | "K" | "L"
//!                 | "M" | "N" | "O" | "P" | "Q" | "R"
//!                 | "S" | "T" | "U" | "V" | "W" | "X"
//!                 | "Y" | "Z"
//!     decimal_digit ::= "0" | "1" | "2" | "3" | "4"
//!                     | "5" | "6" | "7" | "8" | "9"
//!

use lalrpop_util;
use lexer;
use p0;
use std::str::FromStr;
use std::num::ParseIntError;

///     name ::= name_first_char name_char*
///     name_first_char ::= letter | "_"
///     name_char ::= letter | "_" | decimal_digit
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(Vec<u8>);

impl Name {
    pub fn new(bytes: &[u8]) -> Result<Name, ()> {
        let valid_name = bytes
            .split_first()
            .and_then(|(&first, rest)| match first {
                b'a'...b'z' | b'A'...b'Z' | b'_' => Some(rest),
                _ => None,
            })
            .map_or(false, |rest| {
                rest.iter().all(|&b| match b {
                    b'a'...b'z' | b'A'...b'Z' | b'_' | b'0'...b'9' => true,
                    _ => false,
                })
            });
        if valid_name {
            Ok(Name(bytes.into()))
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DecimalI32(pub i32);

impl FromStr for DecimalI32 {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i32::from_str_radix(s, 10).map(DecimalI32)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Input;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Name(Name),
    DecimalI32(DecimalI32),
    Input(Input),
    UnaryNeg(Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    Print(Expression),
    Assign(Name, Expression),
    Expression(Expression),
    Newline,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    pub module: Module,
}

impl From<Statement> for Program {
    fn from(statement: Statement) -> Program {
        Program {
            module: Module {
                statements: vec![statement],
            }
        }
    }
}

impl From<Expression> for Program {
    fn from(expression: Expression) -> Program {
        Program {
            module: Module {
                statements: vec![Statement::Expression(expression)],
            }
        }
    }
}

pub type ParseError<'input> = lalrpop_util::ParseError<usize, lexer::Tok<'input>, lexer::Error>;

pub trait Parse {
    fn parse<'input>(&'input self) -> Result<Program, ParseError<'input>>;
}

impl Parse for str {
    fn parse<'input>(&'input self) -> Result<Program, ParseError<'input>> {
        let lexer = lexer::Lexer::new(self);
        p0::parse_program(self, lexer)
    }
}
