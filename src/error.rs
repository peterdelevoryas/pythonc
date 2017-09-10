use lexer;
use lalrpop_util::ParseError;

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
