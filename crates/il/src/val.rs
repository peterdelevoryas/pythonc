use ty::Ty;
use func::ConstFunc;
use inst::Unop;

impl_index_type!(Val);

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
}
