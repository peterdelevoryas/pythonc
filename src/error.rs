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
    }

    errors {
        ParseIntegerLiteral(e: ParseIntError) {
            description("invalid integer literal"),
            display("invalid integer literal: {}", e),
        }
        UnexpectedEof(source: String) {
            description("unexpected end of file"),
            display("unexpected end of file: {:?}", source),
        }
        UnexpectedChar(c: char, source: String) {
            description("unexpected character"),
            display("unexpected character: {:?} in {:?}", c, source),
        }
    }
}
