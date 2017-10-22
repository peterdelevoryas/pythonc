use ty::Ty;
use func::ConstFunc;
use inst::Unop;
use std::fmt;
use slab;

impl_index_type!(Val);

pub type Slab = slab::Slab<Val, Data>;

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

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}
