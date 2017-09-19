use Register32;
use Immediate32;

/// Intel:
///
///     segreg:[base+index*scale+disp]
///
///
/// AT&T:
///
///     %segreg:disp(base, index, scale)
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Memory32 {
    pub base: Register32,
    pub index: Option<Register32>,
    pub scale: Option<Immediate32>,
    pub disp: Immediate32,
}
