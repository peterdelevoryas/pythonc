use imm::Imm;
use reg::Reg;
use reg::Reg32;
use mem::Mem;
use Bits;
use Bits32;

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

impl Instr for Mov<Bits32, Reg32, i32> {
    fn trans(&self) -> String {
        match *self {
            Mov::MemReg(ref m, ref r) => {
                format!("movl {}, %{}", m.to_string(), r.name())
            }
            Mov::RegMem(ref r, ref m) => {
                format!("movl %{}, {}", r.name(), m.to_string())
            }
            Mov::ImmMem(i, ref m) => {
                format!("movl ${}, {}", i, m.to_string())
            }
            _ => unimplemented!()
        }
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

impl Instr for Add<Bits32, Reg32, i32> {
    fn trans(&self) -> String {
        match *self {
            Add::RegReg(ref r1, ref r2) => {
                format!("addl %{}, %{}", r1.name(), r2.name())
            }
            Add::MemReg(ref m, ref r) => {
                format!("addl {}, %{}", m.to_string(), r.name())
            }
            Add::RegMem(ref r, ref m) => {
                format!("addl %{}, {}", r.name(), m.to_string())
            }
            Add::ImmReg(ref i, ref r) => {
                format!("addl ${}, %{}", i, r.name())
            }
            Add::ImmMem(ref i, ref m) => {
                format!("addl ${}, {}", i, m.to_string())
            }
        }
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

impl Instr for Push<Reg32, i32> {
    fn trans(&self) -> String {
        match *self {
            Push::Reg(ref r) => {
                format!("pushl %{}", r.name())
            }
            Push::Mem(ref m) => {
                format!("pushl {}", m.to_string())
            }
            Push::Imm(i) => {
                format!("pushl ${}", i)
            }
        }
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

impl Instr for Neg<Bits32, Reg32, i32> {
    fn trans(&self) -> String {
        match *self {
            Neg::Reg(ref r) => format!("negl %{}", r.name()),
            Neg::Mem(ref m) => format!("negl {}", m.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Call {
    pub label: String,
}

impl Instr for Call {
    fn trans(&self) -> String {
        format!("call {}", self.label)
    }
}

#[derive(Debug)]
pub struct Ret;
