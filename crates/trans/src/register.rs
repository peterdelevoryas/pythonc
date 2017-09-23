#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Register {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
}

impl Register {
    pub fn as_str(&self) -> &'static str {
        use self::Register::*;
        match *self {
            EAX => "eax",
            EBX => "ebx",
            ECX => "ecx",
            EDX => "edx",
            ESI => "esi",
            EDI => "edi",
            ESP => "esp",
            EBP => "ebp",
        }
    }
}

#[cfg(test)]
mod test {}
