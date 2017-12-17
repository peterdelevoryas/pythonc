use ssa::Value;
use ssa::Block;
use std::fmt;

pub struct Ret {
    pub value: Option<Value>,
}

pub struct Jmp {
    pub destination: Block,
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

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Branch::*;
        match *self {
            Ret(ref ret) => {
                write!(f, "ret")?;
                if let Some(value) = ret.value {
                    write!(f, " {}", value)?;
                }
                Ok(())
            }
            Jmp(ref jmp) => write!(f, "jmp {}", jmp.destination),
            Jnz(ref jnz) => write!(f, "jnz {}, {}, {}", jnz.cond, jnz.jnz, jnz.jmp),
        }
    }
}
