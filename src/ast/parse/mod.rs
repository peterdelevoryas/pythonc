pub mod p0;

use tok;
use lalrpop_util::ParseError;

pub type Error = ParseError<usize, tok::Tok, tok::Error>;
