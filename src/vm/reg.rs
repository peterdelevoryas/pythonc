use std::fmt;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Reg {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
}

pub fn caller_save() -> HashSet<Reg> {
    hash_set!(Reg::EAX, Reg::ECX, Reg::EDX)
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Reg::*;
        let s = match *self {
            EAX => "eax",
            EBX => "ebx",
            ECX => "ecx",
            EDX => "edx",
            ESI => "esi",
            EDI => "edi",
            ESP => "esp",
            EBP => "ebp",
        };
        write!(f, "{}", s)
    }
}
