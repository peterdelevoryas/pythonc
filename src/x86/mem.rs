use x86::reg::Reg;
use x86::imm::Imm;
use x86::Bits;

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
