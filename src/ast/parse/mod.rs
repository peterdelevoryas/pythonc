pub mod tok;
pub mod p0;

use lalrpop_util::ParseError;

pub type Error = ParseError<usize, tok::Tok, tok::Error>;
