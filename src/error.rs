use ast::ParseError;
use std::process::ExitStatus;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
        ReprParser(::parser::Error, ::parser::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        Parse(ParseError);
    }

    errors {
        LinkRuntime(e: ExitStatus) {
            description("Error linking program with runtime"),
            display("Error linking program with runtime (exit status {})", e),
        }

        Assembler(e: ExitStatus) {
            description("Error assembling program"),
            display("Error assembling program (exit status {})", e),
        }

        MissingRuntime {
            description("--runtime argument missing"),
            display("--runtime argument missing, cannot create binary"),
        }

        MissingInput {
            description("INPUT argument missing"),
            display("INPUT argument missing, nothing to compile"),
        }
    }
}
