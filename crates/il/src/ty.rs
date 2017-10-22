use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Int,
    Bool,
    List,
    Dict,
    Any,
    Unit,
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ty = match *self {
            Ty::Int => "int",
            Ty::Bool => "bool",
            Ty::List => "list",
            Ty::Dict => "dict",
            Ty::Any => "any",
            Ty::Unit => "()",
        };
        write!(f, "{}", ty)
    }
}
