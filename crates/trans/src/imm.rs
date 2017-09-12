use Bits;
use Bits32;

pub trait Imm {
    type Size: Bits;
}

impl Imm for i32 {
    type Size = Bits32;
}
