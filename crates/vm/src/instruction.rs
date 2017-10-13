#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Instruction {
    Load(Load),
    /*
    Store(Store),
    Mov(Mov),
    Add(Add),
    Push(Push),
    Neg(Neg),
    Call(Call),
    */
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Load {
}
