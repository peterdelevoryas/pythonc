error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Parse(::ast::parse::Error);
    }

    errors {
    }
}
