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
    pub displacement: Displacement,
}

/// Can only be 1, 2, 4, or 8
/// Only provides `fn value(&self) -> u8`, rather
/// than `pub value: u8`, because we want to prevent
/// `ScaleFactor` from ever being anything other
/// than 1, 2, 4, or 8.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ScaleFactor(u8);

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

    pub fn value(&self) -> u8 {
        self.0
    }
}

/// This wrapper struct exists because
///     1. Are there any restrictions on displacement value? (I don't think so)
///     2. AT&T syntax doesn't format displacements the same as immediates.
/// Otherwise, this would just be replaced with using `Immediate`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Displacement(pub i32);

impl Displacement {
    pub fn new(value: i32) -> Displacement {
        Displacement(value)
    }
}
