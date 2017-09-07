use x86::reg::Reg;
use x86::reg::Reg32;
use x86::imm::Imm;
use x86::Bits;
use x86::Bits32;

/// Intel:
///
///     segreg:[base+index*scale+disp]
///
///
/// AT&T:
///
///     %segreg:disp(base, index, scale)
///
#[derive(Debug)]
pub struct Mem<B, R, I>
    where B: Bits,
          R: Reg<Size=B>,
          I: Imm<Size=B>,
{
    pub base: R,
    //pub index: Option<R>,
    //pub scale: Option<I>,
    pub disp: I,
}

impl Mem<Bits32, Reg32, i32> {
    pub fn to_string(&self) -> String {
        format!("{}(%{})", self.disp, self.base.name())
    }
}
