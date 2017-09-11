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
