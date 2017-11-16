#[macro_use]
pub mod impl_wrapper_enum;
#[macro_use]
pub mod impl_ref;

pub mod fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn into<T>(self) -> T
    where
        T: From<L>,
        T: From<R>,
    {
        match self {
            Either::Left(l) => T::from(l),
            Either::Right(r) => T::from(r),
        }
    }
}
