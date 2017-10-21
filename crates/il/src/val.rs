use ty::Ty;

impl_index_type!(Val);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Data {
    ty: Ty,
    src: Option<String>,
}
