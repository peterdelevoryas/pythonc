use self::sealed::Sealed;
use std::fmt::Display;

mod sealed {
    pub trait Sealed {}
}

pub trait Unsigned: Sealed {}
pub trait Signed: Sealed {}
pub trait Imm8: Sealed {}
pub trait Imm16: Sealed {}
pub trait Imm32: Sealed {}
pub trait Imm: Sealed + Display {}

macro_rules! impl_imm {
    ($ty:ty: $($trait:ident),+) => {
        impl Sealed for $ty {}
        impl Imm for $ty {}

        $(
            impl $trait for $ty {}
        )+
    }
}

impl_imm!(i8:  Signed);
impl_imm!(i16: Signed);
impl_imm!(i32: Signed);

impl_imm!(u8:  Unsigned);
impl_imm!(u16: Unsigned);
impl_imm!(u32: Unsigned);
