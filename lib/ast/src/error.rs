error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
    }

    errors {
        InvalidIntegerLiteral(lit: String) {
            description("Invalid integer literal")
            display("Invalid integer literal: {:?}", lit)
        }
    }
}
