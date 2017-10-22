use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Int,
    Bool,
    List,
    Dict,
    PointerPyObj,
    PyObj,
    Unit,
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ty = match *self {
            Ty::Int => "int",
            Ty::Bool => "bool",
            Ty::List => "list",
            Ty::Dict => "dict",
            Ty::PointerPyObj => "*big_pyobj",
            Ty::PyObj => "pyobj",
            Ty::Unit => "()",
        };
        write!(f, "{}", ty)
    }
}
