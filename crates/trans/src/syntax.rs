use std::fmt;
use ia_32;

pub trait Syntax<T> {
    fn syntax(&self, asm: &T, f: &mut fmt::Formatter) -> fmt::Result;
}
