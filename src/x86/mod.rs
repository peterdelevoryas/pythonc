pub mod reg;
pub mod imm;
pub mod ia32;
pub mod mem;

mod sealed {
    pub trait Sealed {}
}

use self::sealed::Sealed;

pub trait Bits: Sealed {}

#[derive(Debug)]
pub enum Bits8 {}
impl Sealed for Bits8 {}
impl Bits for Bits8 {}

#[derive(Debug)]
pub enum Bits16 {}
impl Sealed for Bits16 {}
impl Bits for Bits16 {}

#[derive(Debug)]
pub enum Bits32 {}
impl Sealed for Bits32 {}
impl Bits for Bits32 {}
