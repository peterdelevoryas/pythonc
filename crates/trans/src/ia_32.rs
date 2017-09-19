use Register32;
use Memory32;
use Immediate32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Source32 {
    Register32(Register32),
    Memory32(Memory32),
    Immediate32(Immediate32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Destination32 {
    Register32(Register32),
    Memory32(Memory32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Value32 {
    Register32(Register32),
    Immediate32(Immediate32),
}

/// Uses AT&T syntax
///     RegReg => movB %reg, %reg
///     RegMem => movB %reg, mem
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Load32 {
    pub source: Memory32,
    pub register: Register32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Store32 {
    pub value: Value32,
    pub memory: Memory32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Mov32 {
    pub value: Value32,
    pub register: Register32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Add32 {
    pub value: Value32,
    pub register: Register32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Push32 {
    pub value: Value32
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Neg32 {
    pub register: Register32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Call {
    pub label: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ret;
