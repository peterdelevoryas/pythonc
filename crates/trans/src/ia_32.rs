use Register;
use Memory;
use Immediate;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Source {
    Register(Register),
    Memory(Memory),
    Immediate(Immediate),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Destination {
    Register(Register),
    Memory(Memory),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Register(Register),
    Immediate(Immediate),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Load {
    pub memory: Memory,
    pub register: Register,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Store {
    pub value: Value,
    pub memory: Memory,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Mov {
    pub value: Value,
    pub register: Register,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Add {
    pub value: Value,
    pub register: Register,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Push {
    pub value: Value
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Neg {
    pub register: Register,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Call {
    pub label: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ret;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    Load(Load),
    Store(Store),
    Mov(Mov),
    Add(Add),
    Push(Push),
    Neg(Neg),
    Call(Call),
    Ret(Ret),
}
