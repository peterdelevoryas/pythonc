use Register;
use Immediate;

/// Intel:
///
///     segreg:[base+index*scale+displacement]
///
///
/// AT&T:
///
///     %segreg:displacement(base, index, scale)
///
/// We don't make displacement optional,
/// because the assembler will automatically
/// optimize the instruction selection if
/// displacement is 0. Similar reasoning
/// for allowing ScaleFactor of 1.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Memory {
    pub base: Register,
    pub index: Option<(Register, ScaleFactor)>,
    pub displacement: Immediate,
}

/// Can only be 1, 2, 4, or 8
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ScaleFactor(u32);

impl ScaleFactor {
    pub fn one() -> ScaleFactor {
        ScaleFactor(1)
    }

    pub fn two() -> ScaleFactor {
        ScaleFactor(2)
    }

    pub fn four() -> ScaleFactor {
        ScaleFactor(4)
    }

    pub fn eight() -> ScaleFactor {
        ScaleFactor(8)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}
