use slab_crate;
use std::marker::PhantomData;

pub struct Slab<T, I> {
    slab: slab_crate::Slab<T>,
    marker: PhantomData<I>,
}

impl<T, I> Slab<T, I>
where I: From<usize> + Into<usize>,
{
    
}
