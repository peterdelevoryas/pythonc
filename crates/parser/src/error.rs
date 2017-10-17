error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }

    errors {
        // types here could be better (ie, not just a String)
        Lowering(from: String, to: String) {
            description("Could not lower repr ast node to pythonc ast node"),
            display("Could not lower {} to {}", from, to),
        }

        NonInputCallExpr(name: String) {
            description("Calls to functions other than input() are invalid"),
            display("Call to {:?} is invalid in P0/P1", name),
        }

        NonNameCallTarget(node: String) {
            description("Call target is not a Name expression"),
            display("Call with target {:?} is invalid in P0/P1", node),
        }

        UnexpectedCompareOp(op: String) {
            description("Unexpected operator in compare node"),
            display("Unexpected operator {:?} in compare node", op),
        }
    }
}
