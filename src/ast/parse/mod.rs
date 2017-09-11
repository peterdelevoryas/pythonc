pub mod p0;

use ast::{
    DecimalI32,
    Name,
    Expression,
    Statement,
    Module,
    Program,
};
use tok;
use lalrpop_util::ParseError;
use std::iter::Iterator;

pub type Error = ParseError<usize, tok::Tok, tok::Error>;
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Parser<S> {
    stream: S,
}

impl<S> Parser<S>
    where S: Iterator<Item=tok::Spanned<tok::Tok>>
{
    pub fn new(stream: S) -> Parser<S> {
        Parser { stream }
    }

}
    
// TODO There has to be a better way to do this
// than just making a macro around it, or maybe
// it's just unnecessary to have this Parser<S>
// struct.
macro_rules! impl_parser_methods {
    ($($parse_fn:ident -> $data:ty;)+) => {
        impl<S> Parser<S>
            where S: Iterator<Item=tok::Spanned<tok::Tok>>
        {
            $(
                pub fn $parse_fn(&mut self) -> Result<$data> {
                    p0::$parse_fn(&mut self.stream)
                }
            )+
        }
    }
}

impl_parser_methods! {
    parse_decimal_i32 -> DecimalI32;
    parse_name -> Name;
    parse_expression -> Expression;
    parse_statement -> Statement;
    parse_module -> Module;
    parse_program -> Program;
}
