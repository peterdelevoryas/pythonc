/// Implements trivial From<variant_type> impls for enum
#[macro_export]
macro_rules! impl_wrapper_enum {
    (
        $(#[$meta:meta])*
        pub enum $ty:ident {
            boxed: [$($box_variant:ident),*];
            simple: [$($simple_variant:ident),*];
        }
    ) => {
        $(#[$meta])*
        pub enum $ty {
            $(
                $box_variant(Box<$box_variant>),
            )*
            $(
                $simple_variant($simple_variant),
            )*
        }

        $(
            impl From<$simple_variant> for $ty {
                fn from(val: $simple_variant) -> $ty {
                    $ty::$simple_variant(val)
                }
            }
        )*
        $(
            impl From<$box_variant> for $ty {
                fn from(val: $box_variant) -> $ty {
                    $ty::$box_variant(box val)
                }
            }
        )*
    }
}
