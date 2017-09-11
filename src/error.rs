use ast::ParseError;
use std::process::ExitStatus;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error);
        Parse(ParseError);
    }

    errors {
        Link(e: ExitStatus) {
            description("link error"),
            display("link error (exit status {})", e),
        }
    }
}
