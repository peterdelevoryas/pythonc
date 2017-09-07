use x86::imm::Imm;
use x86::reg::Reg;
use x86::mem::Mem;
use x86::Bits;
use x86::Bits32;

pub trait Instr {
    fn trans(&self) -> String;
}

/// Uses AT&T syntax 
///     RegReg => movB %reg, %reg
///     RegMem => movB %reg, mem
#[derive(Debug)]
pub enum Mov<B, R, I>
where
    B: Bits,
    R: Reg<Size=B>,
    I: Imm<Size=B>,
{
    RegReg(R, R),
    RegMem(R, Mem<B, R, I>),
    MemReg(Mem<B, R, I>, R),
    ImmReg(I, R),
    ImmMem(I, Mem<B, R, I>),
}

impl<B, R, I> Instr for Mov<B, R, I>
where
    B: Bits,
    R: Reg<Size=B>,
    I: Imm<Size=B>,
{
    fn trans(&self) -> String {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum Add<B, R, I>
    where B: Bits,
          R: Reg<Size=B>,
          I: Imm<Size=B>,
{
    RegReg(R, R),
    MemReg(Mem<B, R, I>, R),
    RegMem(R, Mem<B, R, I>),
    ImmReg(I, R),
    ImmMem(I, Mem<B, R, I>),
}

impl<B, R, I> Instr for Add<B, R, I>
    where B: Bits,
          R: Reg<Size=B>,
          I: Imm<Size=B>,
{
    fn trans(&self) -> String {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum Push<R, I>
    where R: Reg<Size=Bits32>,
          I: Imm<Size=Bits32>,
{
    Reg(R),
    Mem(Mem<Bits32, R, I>),
    Imm(I),
}

impl<R, I> Instr for Push<R, I>
where
    R: Reg<Size=Bits32>,
    I: Imm<Size=Bits32>,
{
    fn trans(&self) -> String {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum Neg<B, R, I>
    where B: Bits,
          R: Reg<Size=B>,
          I: Imm<Size=B>,
{
    Reg(R),
    Mem(Mem<B, R, I>),
}

#[derive(Debug)]
pub struct Call {
    pub label: String,
}

impl Instr for Call {
    fn trans(&self) -> String {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Ret;
