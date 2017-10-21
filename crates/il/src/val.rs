use ty::Ty;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Val(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Data {
    ty: Ty,
    src: Option<String>,
}
