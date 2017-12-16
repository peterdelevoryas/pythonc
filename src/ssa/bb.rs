use ssa::Inst;

impl_ref!(Block, "bb");

pub struct BlockData {
    pub body: Vec<Inst>,
}

pub type BlockGen = Gen;
