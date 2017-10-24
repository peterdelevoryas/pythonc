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
    Call(String, Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Target {
    Name(String),
    Subscript(Box<Expression>, Box<Expression>),

    // just for explication and flattening
    Tmp(Tmp),
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
            Assign(Tmp(_), _) => panic!("There shouldn't be any Tmp's before explication!"),
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
                        box InjectFrom(Bool, box Is(box GetTag(box Target(Tmp(l_tmp))), box DecimalI32(BIG_TAG))),
                        box InjectFrom(Big, box Call("add".into(), vec![ProjectTo(Big, box Target(Tmp(l_tmp))),
                                                                        ProjectTo(Big, box Target(Tmp(r_tmp)))])),
                        box InjectFrom(Int, box Add(box ProjectTo(Int, box Target(Tmp(l_tmp))),
                                                    box ProjectTo(Int, box Target(Tmp(r_tmp)))))
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
                        box Target(Tmp(l_tmp)),
                        box r,
                        box Target(Tmp(l_tmp)),
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
                        box Target(Tmp(l_tmp)),
                        box Target(Tmp(l_tmp)),
                        box r,
                    )
                )
            }
            Target(Subscript(box base, box elem)) => {
                let base = base.explicate(tmp_gen);
                let elem = elem.explicate(tmp_gen);
                Call("get_subscript".into(), vec![base, elem])
            }
            other => other,
        }
    }
}

pub struct Flattener<'g, G>
where
    G: 'g + Iterator<Item=Tmp>,
{
    tmp_gen: &'g mut G,
    flat: Vec<Statement>,
}

impl<'g, G> Flattener<'g, G>
where
    G: 'g + Iterator<Item=Tmp>,
{
    pub fn new(tmp_gen: &'g mut G) -> Self {
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
        use self::Target::*;
        match st {
            Print(e) => {
                let print = Print(self.expression(e));
                self.flat.push(print);
            }
            Assign(Subscript(_, _), _) => panic!("Encountered un-explicated assign subscript"),
            Assign(target @ Name(_), e) | Assign(target @ Tmp(_), e) => {
                let v = self.expression(e);
                self.flat.push(Assign(target, v));
            }
            Expression(e) => {
                let v = self.expression(e);
                self.flat.push(Expression(v));
            }
            If(box e, then, els) => {
                let v = self.expression(e);
                let then = {
                    let mut b = Self::new(self.tmp_gen);
                    b.block(then);
                    b.complete()
                };
                let els = {
                    let mut b = Self::new(self.tmp_gen);
                    b.block(els);
                    b.complete()
                };
                self.flat.push(If(box v, then, els));
            }
            Newline => {}
        }
    }

    // Should only return Expression::Target(Target::Name)
    // or Expression::Tmp
    pub fn expression(&mut self, e: Expression) -> Expression {
        use self::Statement::*;
        use self::Expression::*;
        use self::Target::*;
        match e {
            target @ Target(Name(_)) | target @ Target(Tmp(_)) => target,
            c @ DecimalI32(_) | c @ Boolean(_) => self.tmp(c),
            UnaryNeg(box e) => {
                let tmp = self.expression(e);
                self.tmp(UnaryNeg(box tmp))
            }
            Add(box l, box r) => {
                let l = self.expression(l);
                let r = self.expression(r);
                self.tmp(Add(box l, box r))
            }
            LogicalNot(box e) => {
                let e = self.expression(e);
                self.tmp(LogicalNot(box e))
            }
            LogicalEq(box l, box r) => {
                let l = self.expression(l);
                let r = self.expression(r);
                self.tmp(LogicalEq(box l, box r))
            }
            LogicalNotEq(box l, box r) => {
                let l = self.expression(l);
                let r = self.expression(r);
                self.tmp(LogicalNotEq(box l, box r))
            }
            Is(box l, box r) => {
                let l = self.expression(l);
                let r = self.expression(r);
                self.tmp(Is(box l, box r))
            }
            // Should really move list and dict into
            // explicate, but unfortunately it's not setup
            // correctly right now for that (need to be able
            // to explicate Expression into Vec<Statement>)
            List(elems) => {
                let len = self.tmp(DecimalI32(elems.len() as i32));
                let len_pyobj = self.tmp(InjectFrom(Type::Int, box len));
                let list = self.tmp(Call("create_list".into(), vec![len_pyobj]));
                let list_pyobj = self.tmp(InjectFrom(Type
                unimplemented!()
            }
            Dict(pairs) => unimplemented!(),
            ::Expression::If(box cond, box then, box els) => {
                let cond = self.expression(cond);
                let res = self.tmp_gen.next().unwrap();
                let then = {
                    let mut builder = Self::new(self.tmp_gen);
                    let then = builder.expression(then);
                    builder.flat.push(Assign(Tmp(res), then));
                    builder.complete()
                };
                let els = {
                    let mut builder = Self::new(self.tmp_gen);
                    let els = builder.expression(els);
                    builder.flat.push(Assign(Tmp(res), els));
                    builder.complete()
                };
                Target(Tmp(res))
            }
            GetTag(box e) => {
                let e = self.expression(e);
                self.tmp(GetTag(box e))
            }
            InjectFrom(ty, box e) => {
                let e = self.expression(e);
                self.tmp(InjectFrom(ty, box e))
            }
            ProjectTo(ty, box e) => {
                let e = self.expression(e);
                self.tmp(ProjectTo(ty, box e))
            }
            Let(decls, box e) => {
                for (tmp, e) in decls.decls {
                    self.flat.push(Assign(Tmp(tmp), e));
                }
                self.tmp(e)
            }
            Call(name, args) => {
                let args: Vec<_> = args.into_iter()
                    .map(|arg| self.expression(arg))
                    .collect();
                Call(name, args)
            }
            e @ Input
                | e @ Target(Subscript(_, _))
                | e @ LogicalAnd(_, _)
                | e @ LogicalOr(_, _) => panic!("{:?} should have been explicated", e),
        }
    }

    pub fn tmp(&mut self, e: Expression) -> Expression {
        use self::Statement::*;
        use self::Expression::*;
        use self::Target::*;
        let tmp = self.tmp_gen.next().unwrap();
        self.flat.push(Assign(Tmp(tmp), e));
        Target(Tmp(tmp))
    }

    pub fn complete(&mut self) -> Block {
        Block {
            statements: ::std::mem::replace(&mut self.flat, vec![]),
        }
    }
}
