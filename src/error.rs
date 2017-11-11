error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
        Ast(::ast::error::Error, ::ast::error::ErrorKind);
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Clap(::clap::Error);
    }

    errors {
        LinkRuntime(e: ::std::process::ExitStatus) {
            description("Error linking program with runtime"),
            display("Error linking program with runtime (exit status {})", e),
        }
    }
}
