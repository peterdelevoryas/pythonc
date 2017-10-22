use val::Val;
use val::Const;
use ty::Ty;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Inst {
    Binop(Binop, Arg, Arg),
    Unop(Unop, Arg),
    Call {
        func: Arg,
        args: Vec<Arg>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Binop {
    Add,
    And,
    Or,
    Eq,
    NotEq,
    Is,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Unop {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Arg {
    Loc(Val),
    Const(Const),
}

impl From<Val> for Arg {
    fn from(v: Val) -> Arg {
        Arg::Loc(v)
    }
}

impl From<Const> for Arg {
    fn from(c: Const) -> Arg {
        Arg::Const(c)
    }
}

impl Inst {
    pub fn ret_ty(&self) -> Ty {
        unimplemented!()
    }
}
