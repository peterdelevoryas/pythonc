use ssa::Value;
use ssa::Block;

pub struct Ret {
    pub value: Option<Value>,
}

pub struct Jmp {
    pub block: Block,
}

pub struct Jnz {
    pub cond: Value,
    pub jnz: Block,
    pub jmp: Block,
}

pub enum Branch {
    Ret(Ret),
    Jmp(Jmp),
    Jnz(Jnz),
}
