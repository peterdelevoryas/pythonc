use x86::Bits;
use x86::Bits8;
use x86::Bits16;
use x86::Bits32;

pub use self::Reg8::*;
pub use self::Reg16::*;
pub use self::Reg32::*;

pub trait Reg {
    type Size: Bits;
    fn name(&self) -> &str;
}

#[derive(Debug)]
pub enum Reg8 {
    AH,
    AL,
    BH,
    BL,
    CH,
    CL,
    DH,
    DL,
}

#[derive(Debug)]
pub enum Reg16 {
    AX,
    BX,
    CX,
    DX,
}

#[derive(Debug)]
pub enum Reg32 {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
}

impl Reg for Reg8 {
    type Size = Bits8;
    fn name(&self) -> &str {
        match *self {
            AH => "ah",
            AL => "al",
            BH => "bh",
            BL => "bl",
            CH => "ch",
            CL => "cl",
            DH => "dh",
            DL => "dl",
        }
    }
}

impl Reg for Reg16 {
    type Size = Bits16;
    fn name(&self) -> &str {
        match *self {
            AX => "ax",
            BX => "bx",
            CX => "cx",
            DX => "dx",
        }
    }
}

impl Reg for Reg32 {
    type Size = Bits32;
    fn name(&self) -> &str {
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
mod test {
    use x86::reg::Set;

    #[test]
    fn split_borrow() {
        let mut set = Set::new();
        let _ah = &mut set.eax.ax.ah;
        let _al = &mut set.eax.ax.al;
        // must compile
    }
}
