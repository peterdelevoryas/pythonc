use x86::reg::Reg;

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
pub struct Mem {
    base: usize,
    index: usize,
    scale: usize,
    disp: usize,
}
