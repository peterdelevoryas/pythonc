use ssa::Inst;

impl_ref!(BB, "bb");

#[derive(Debug, Clone)]
pub struct Data {
    pub body: Vec<Inst>,
}
