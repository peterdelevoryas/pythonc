use ssa::Val;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Rval {
    Val(Val),
    Imm(i32),
}
