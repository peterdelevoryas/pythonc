use Register;
use Immediate;

/// Intel:
///
///     segreg:[base+index*scale+disp]
///
///
/// AT&T:
///
///     %segreg:disp(base, index, scale)
///
#[derive(Debug, Copy, Clone)]
pub struct Memory {
    pub base: Register,
    pub index: Option<Register>,
    pub scale: Option<Immediate>,
    pub disp: Immediate,
}
