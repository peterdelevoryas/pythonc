use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Register32 {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
}

impl fmt::Display for Register32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Register32::*;
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
        write!(f, "%{}", s)
    }
}

#[cfg(test)]
mod test {}
