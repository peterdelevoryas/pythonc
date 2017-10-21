use val::Val;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Inst {
    Binop(Binop, Val, Val),
    Unop(Unop, Val),
    Call {
        func: Val,
        args: Vec<Val>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Binop {
    Add,
    Not,
    And,
    Or,
    Eq,
    NotEq,
    Is,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Unop {
    Neg,
}
