use slab_crate;
use std::marker::PhantomData;

#[macro_export]
macro_rules! impl_index_type {
    ($name:ident) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $name(usize);

        impl From<usize> for $name {
            fn from(u: usize) -> $name {
                $name(u)
            }
        }

        impl From<$name> for usize {
            fn from(i: $name) -> usize {
                i.0
            }
        }
    }
}

#[derive(Debug)]
pub struct Slab<I, T>
where
    I: From<usize> + Into<usize> + Copy
{
    slab: slab_crate::Slab<T>,
    marker: PhantomData<I>,
}

impl<I, T> Slab<I, T>
where I: From<usize> + Into<usize> + Copy,
{
    pub fn new() -> Self {
        Self {
            slab: slab_crate::Slab::new(),
            marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.slab.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slab.is_empty()
    }

    pub fn iter<'slab>(&'slab self) 
        -> impl 'slab + Iterator<Item=(I, &'slab T)>
    {
        self.slab.iter().map(|(i, val)| (I::from(i), val))
    }

    pub fn iter_mut<'slab>(&'slab mut self)
        -> impl 'slab + Iterator<Item=(I, &'slab mut T)>
    {
        self.slab.iter_mut().map(|(i, val)| (I::from(i), val))
    }

    pub fn insert(&mut self, val: T) -> I {
        let i = self.slab.insert(val);
        I::from(i)
    }

    pub fn remove(&mut self, key: I) -> T {
        self.slab.remove(key.into())
    }

    pub fn contains(&self, key: I) -> bool {
        self.slab.contains(key.into())
    }
}

use std::ops::Index;
use std::ops::IndexMut;

impl<I, T> Index<I> for Slab<I, T>
where I: From<usize> + Into<usize> + Copy
{
    type Output = T;
    fn index(&self, key: I) -> &T {
        self.slab.index(key.into())
    }
}

impl<I, T> IndexMut<I> for Slab<I, T>
where I: From<usize> + Into<usize> + Copy
{
    fn index_mut(&mut self, key: I) -> &mut T {
        self.slab.index_mut(key.into())
    }
}
