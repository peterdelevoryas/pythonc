use lexer;
use lalrpop_util::ParseError;
use std::num::ParseIntError;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Parse(ParseError<usize, lexer::Tok, lexer::Error>);
    }

    errors {
    }
}
