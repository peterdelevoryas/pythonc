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
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $ident(usize);

        pub type Slab<T> = $crate::slab::Slab<T, $ident>;

        #[derive(Debug, Clone)]
        pub struct Gen {
            next: usize,
        }

        impl From<usize> for $ident {
            fn from(u: usize) -> $ident {
                $ident::new(u)
            }
        }

        impl Into<usize> for $ident {
            fn into(self) -> usize {
                self.0
            }
        }

        impl $ident {
            fn new(i: usize) -> $ident {
                $ident(i)
            }

            pub fn inner(&self) -> usize {
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

        impl Iterator for Gen {
            type Item = $ident;
            fn next(&mut self) -> Option<Self::Item> {
                let next = self.next;
                self.next += 1;
                Some($ident::new(next))
            }
        }

        impl ::std::fmt::Display for $ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}{}", $display_prefix, self.0)
            }
        }
    }
}
