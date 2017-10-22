use ty::Ty;
use func::ConstFunc;
use inst::Unop;
use std::fmt;
use slab;

impl_index_type!(Val);

pub type Slab = slab::Slab<Val, Data>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Const {
    Int(i32),
    Bool(bool),
    List(Vec<Const>),
    Dict(Vec<(Const, Const)>),
    Func(&'static ConstFunc),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Data {
    ty: Ty,
    src: Option<String>,
}

impl Data {
    pub fn unnamed(ty: Ty) -> Data {
        Data {
            ty,
            src: None,
        }
    }

    pub fn ty(&self) -> Ty {
        self.ty
    }
}

impl From<i32> for Const {
    fn from(i: i32) -> Const {
        Const::Int(i)
    }
}

impl From<bool> for Const {
    fn from(b: bool) -> Const {
        Const::Bool(b)
    }
}

impl From<Vec<Const>> for Const {
    fn from(cs: Vec<Const>) -> Const {
        Const::List(cs)
    }
}

impl From<Vec<(Const, Const)>> for Const {
    fn from(cs: Vec<(Const, Const)>) -> Const {
        Const::Dict(cs)
    }
}

impl From<&'static ConstFunc> for Const {
    fn from(f: &'static ConstFunc) -> Const {
        Const::Func(f)
    }
}

impl Const {
    pub fn unop(&self, unop: Unop) -> Const {
        match (unop, self) {
            (Unop::Neg, &Const::Int(i)) => Const::Int(-i),
            (Unop::Not, &Const::Int(i)) => Const::Bool(i == 0),
            (Unop::Neg, &Const::Bool(b)) => Const::Int(if b { -1 } else { 0 }),
            (Unop::Not, &Const::Bool(b)) => Const::Bool(!b),
            _ => panic!("Invalid unop: {:?} {:?}", unop, self),
        }
    }

    pub fn ty(&self) -> Ty {
        match *self {
            Const::Int(_) => Ty::Int,
            Const::Bool(_) => Ty::Bool,
            Const::List(_) => Ty::List,
            Const::Dict(_) => Ty::Dict,
            Const::Func(f) => f.ret,
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Const::Int(ref i) => write!(f, "{}", i),
            Const::Bool(ref b) => write!(f, "{}", b),
            Const::List(ref list) => {
                write!(f, "[")?;
                if !list.is_empty() {
                    write!(f, "{}", list[0])?;
                    for elem in &list[1..] {
                        write!(f, ", {}", elem)?;
                    }
                }
                write!(f, "]")
            }
            Const::Dict(ref dict) => {
                write!(f, "{{")?;
                if !dict.is_empty() {
                    write!(f, "{}: {}", dict[0].0, dict[0].1)?;
                    for tuple in &dict[1..] {
                        write!(f, ", {}: {}", tuple.0, tuple.1)?;
                    }
                }
                write!(f, "}}")
            }
            Const::Func(ref func) => {
                write!(f, "func {}(", func.name)?;
                if !func.args.is_empty() {
                    write!(f, "{}", func.args[0])?;
                    for ty in &func.args[1..] {
                        write!(f, ", {}", ty)?;
                    }
                }
                write!(f, ") -> {}", func.ret)
            }
        }
    }
}
