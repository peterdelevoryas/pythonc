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
#![feature(box_syntax, box_patterns, conservative_impl_trait)]
extern crate python_token as token;
extern crate lalrpop_util;

//pub mod parse;
//pub use parse::parse_program;

pub mod explicated;

pub type ParseError = lalrpop_util::ParseError<usize, token::Token, token::Error>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Target(Target),
    DecimalI32(i32),
    Boolean(bool),
    Input,
    UnaryNeg(Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    LogicalNot(Box<Expression>),
    LogicalAnd(Box<Expression>, Box<Expression>),
    LogicalOr(Box<Expression>, Box<Expression>),
    LogicalEq(Box<Expression>, Box<Expression>),
    LogicalNotEq(Box<Expression>, Box<Expression>),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    List(Vec<Expression>),
    Dict(Vec<(Expression, Expression)>),
    Is(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Target {
    Name(String),
    Subscript(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    Print(Expression),
    Assign(Target, Expression),
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
