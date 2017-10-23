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

    // Just for explication
    GetTag(Box<Expression>),
    InjectFrom(Type, Box<Expression>),
    ProjectTo(Type, Box<Expression>),
    Let(Decls, Box<Expression>),
    Tmp(Tmp),
    Call(String, Vec<Expression>),
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
    If(Box<Expression>, Block, Block),
    Newline,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Decls {
    pub decls: Vec<(Tmp, Expression)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    pub module: Module,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Bool,
    Big,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Tmp(usize);

pub trait Explicate {
    fn explicate<G>(self, tmp_gen: &mut G) -> Self
    where
        G: Iterator<Item=Tmp>;
}

impl Explicate for Block {
    fn explicate<G>(self, tmp_gen: &mut G) -> Block
    where
        G: Iterator<Item=Tmp>,
    {
        Block {
            statements: self.statements.into_iter()
                .map(|st| st.explicate(tmp_gen))
                .collect()
        }
    }
}

impl Explicate for Statement {
    fn explicate<G>(self, tmp_gen: &mut G) -> Statement
    where
        G: Iterator<Item=Tmp>,
    {
        use self::Statement::*;
        use self::Expression;
        use self::Target::*;
        match self {
            Print(e) => {
                let e = e.explicate(tmp_gen);
                Statement::Expression(Expression::Call("print_any".into(), vec![e]))
            }
            Assign(Name(name), e) => {
                let e = e.explicate(tmp_gen);
                Statement::Assign(Name(name), e)
            }
            Assign(Subscript(box base, box elem), e) => {
                let base = base.explicate(tmp_gen);
                let elem = elem.explicate(tmp_gen);
                let e = e.explicate(tmp_gen);
                Statement::Expression(Expression::Call("set_subscript".into(), vec![base, elem, e]))
            }
            Expression(e) => Expression(e.explicate(tmp_gen)),
            If(box e, then, els) => {
                let e = e.explicate(tmp_gen);
                let then = then.explicate(tmp_gen);
                let els = els.explicate(tmp_gen);
                If(box e, then, els)
            }
            Newline => Newline,
        }
    }
}

const BIG_TAG: i32 = 3;

impl Explicate for Expression {
    fn explicate<G>(self, tmp_gen: &mut G) -> Expression
    where
        G: Iterator<Item=Tmp>
    {
        use self::Expression::*;
        use self::Target::*;
        use self::Type::*;
        match self {
            Input => Call("input_int".into(), vec![]),
            Add(box l, box r) => {
                let l = l.explicate(tmp_gen);
                let r = r.explicate(tmp_gen);
                let l_tmp = tmp_gen.next().unwrap();
                let r_tmp = tmp_gen.next().unwrap();
                let decls = Decls {
                    decls: vec![
                        (l_tmp, l),
                        (r_tmp, r),
                    ],
                };
                Let(decls, 
                    box If(
                        box InjectFrom(Bool, box Is(box GetTag(box Tmp(l_tmp)), box DecimalI32(BIG_TAG))),
                        box InjectFrom(Big, box Call("add".into(), vec![ProjectTo(Big, box Tmp(l_tmp)),
                                                                        ProjectTo(Big, box Tmp(r_tmp))])),
                        box InjectFrom(Int, box Add(box ProjectTo(Int, box Tmp(l_tmp)),
                                                    box ProjectTo(Int, box Tmp(r_tmp))))
                    )
                )
            }
            LogicalAnd(box l, box r) => {
                let l = l.explicate(tmp_gen);
                let l_tmp = tmp_gen.next().unwrap();
                let decls = Decls {
                    decls: vec![
                        (l_tmp, l),
                    ],
                };
                Let(decls,
                    box If(
                        box Tmp(l_tmp),
                        box r,
                        box Tmp(l_tmp),
                    )
                )
            }
            LogicalOr(box l, box r) => {
                let l = l.explicate(tmp_gen);
                let l_tmp = tmp_gen.next().unwrap();
                let decls = Decls {
                    decls: vec![
                        (l_tmp, l),
                    ],
                };
                Let(decls,
                    box If(
                        box Tmp(l_tmp),
                        box Tmp(l_tmp),
                        box r,
                    )
                )
            }
            other => other,
        }
    }
}

pub struct Flattener<G>
where
    G: Iterator<Item=Tmp>,
{
    tmp_gen: &mut G,
    flat: Vec<Statement>,
}

impl<G> Flattener<G>
where
    G: Iterator<Item=Tmp>,
{
    pub fn new(tmp_gen: &mut G) -> Self {
        Flattener {
            tmp_gen,
            flat: vec![]
        }
    }

    pub fn block(&mut self, block: Block) {
        for st in block.statements {
            self.statement(st);
        }
    }

    pub fn statement(&mut self, st: Statement) {
        use self::Statement::*;
        unimplemented!()
    }

    // Should only return Expression::Target(Target::Name)
    // or Expression::Tmp
    pub fn expression(&mut self, e: Expression) -> Expression {
        unimplemented!()
    }

    pub fn complete(&mut self) -> Block {
        Block {
            statements: ::std::mem::replace(&mut self.flat, vec![]),
        }
    }
}
