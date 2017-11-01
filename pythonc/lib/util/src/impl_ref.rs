//!
//! Provides macro for implementing
//! a continuous, monotonic index type
//! (such as Name, Temp, Block, etc)
//!
//! Heavily inspired by EntityRef pattern in Cretonne.
//!

///
///     impl_ref!(Name, "n");
///
/// Results in:
/// 
///     #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
///     pub struct Name(usize);
///
///     pub struct Slab<T> { ... }
///
///     pub struct Gen { ... }
///
///     impl Iterator<Item=Name> for Gen { ... }
///
///     let n = Name(1);
///     println!("{}", n); // formats as "n1"
///     let mut g = Name::generator();
///     let a = g.next();
///     let b = g.next();
///     // etc
///     let mut slab: Slab<Data> = Name::slab();
///     let a = slab.insert(Data::new(...));
///     println!("{:?}", slab[a]); // Data { ... }
///
#[macro_export]
macro_rules! impl_ref {
    (
        $ident:ident,
        $display_prefix:expr
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $ident(usize);

        #[derive(Debug, Clone)]
        pub struct Slab<T> {
            inner: ::slab::Slab<T>,
        }

        pub struct Iter<'slab, T: 'slab> {
            inner: ::slab::Iter<'slab, T>,
        }

        #[derive(Debug, Clone)]
        pub struct Gen {
            next: usize,
        }

        impl $ident {
            fn new(i: usize) -> $ident {
                $ident(i)
            }

            fn inner(&self) -> usize {
                self.0
            }
        }

        impl Gen {
            pub fn new() -> Gen {
                Gen {
                    next: 0,
                }
            }
        }

        impl<T> Slab<T> {
            pub fn new() -> Self {
                Slab {
                    inner: ::slab::Slab::new(),
                }
            }

            pub fn len(&self) -> usize {
                self.inner.len()
            }

            pub fn is_empty(&self) -> bool {
                self.inner.is_empty()
            }

            pub fn insert(&mut self, data: T) -> $ident {
                let i = self.inner.insert(data);
                $ident::new(i)
            }

            pub fn iter(&self) -> Iter<T> {
                Iter {
                    inner: self.inner.iter(),
                }
            }

            pub fn get(&self, index: $ident) -> Option<&T> {
                self.inner.get(index.inner())
            }

            /// Panics if index not full
            pub fn remove(&mut self, index: $ident) -> T {
                self.inner.remove(index.inner())
            }
        }

        impl Iterator for Gen {
            type Item = $ident;
            fn next(&mut self) -> Option<Self::Item> {
                let next = self.next;
                self.next += 1;
                Some($ident::new(next))
            }
        }

        impl<T> ::std::ops::Index<$ident> for Slab<T> {
            type Output = T;
            fn index(&self, index: $ident) -> &T {
                self.inner.index(index.inner())
            }
        }

        impl<T> ::std::ops::IndexMut<$ident> for Slab<T> {
            fn index_mut(&mut self, index: $ident) -> &mut T {
                self.inner.index_mut(index.inner())
            }
        }

        impl<'slab, T> ::std::iter::Iterator for Iter<'slab, T> {
            type Item = ($ident, &'slab T);
            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next().map(|(i, data)| ($ident::new(i), data))
            }
        }

        impl<'slab, T> IntoIterator for &'slab Slab<T> {
            type Item = ($ident, &'slab T);
            type IntoIter = Iter<'slab, T>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
    }
}
