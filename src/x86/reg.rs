use self::sealed::Sealed;

pub trait Reg8: Sealed {}
pub trait Reg16: Sealed {}
pub trait Reg32: Sealed {}

pub trait Reg: Sealed {
    const NAME: &'static str;
}

mod sealed {
    pub trait Sealed {}
}

macro_rules! register_set {
    (
        $Set:ident {
        $(
            $reg32:ident: $Reg32:ident {
            $(
                $reg16:ident: $Reg16:ident {
                $(
                    $reg8: ident: $Reg8:ident
                ),*
                }
            ),*
            }
        ),*
        }
    ) => {
        $(
        register! {
            $Reg32 {
            $(
                $reg16: $Reg16 {
                $(
                    $reg8: $Reg8
                ),*
                }
            ),*
            }
        }
        )*

        #[derive(Debug)]
        pub struct $Set {
        $(
            $reg32: $Reg32
        ),*
        }

        impl $Set {
            pub fn new() -> $Set {
                $Set {
                $(
                    $reg32: $Reg32::new()
                ),*
                }
            }
        }
    }
}

macro_rules! register {
    (
        $Reg32:ident {
        $(
            $reg16:ident: $Reg16:ident {
            $(
                $reg8: ident: $Reg8:ident
            ),*
            }
        ),*
        }
    ) => {
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub struct $Reg32 {
            private: (),
        $(
            pub $reg16: $Reg16,
        ),*
        }

        impl $Reg32 {
            fn new() -> $Reg32 {
                $Reg32 {
                    private: (),
                $(
                    $reg16: $Reg16::new()
                ),*
                }
            }
        }

        impl Sealed for $Reg32 {}
        impl Reg32 for $Reg32 {}
        impl Reg for $Reg32 {
            const NAME: &'static str = stringify!($reg32);
        }

        $(
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub struct $Reg16 {
            private: (),
        $(
            pub $reg8: $Reg8
        ),*
        }

        impl $Reg16 {
            fn new() -> $Reg16 {
                $Reg16 {
                    private: (),
                $(
                    $reg8: $Reg8::new()
                ),*
                }
            }
        }

        impl Sealed for $Reg16 {}
        impl Reg16 for $Reg16 {}
        impl Reg for $Reg16 {
            const NAME: &'static str = stringify!($reg16);
        }
        )*

        $(
        $(
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub struct $Reg8 {
            private: (),
        }

        impl $Reg8 {
            fn new() -> $Reg8 {
                $Reg8 { private: () }
            }
        }

        impl Sealed for $Reg8 {}
        impl Reg8 for $Reg8 {}
        impl Reg for $Reg8 {
            const NAME: &'static str = stringify!($reg8);
        }
        )*
        )*

    }
}

register_set! {
    X86 {
        eax: EAX {
            ax: AX {
                ah: AH,
                al: AL
            }
        },
        ecx: ECX {
            cx: CX {
                ch: CH,
                cl: CL
            }
        },
        edx: EDX {
            dx: DX {
                dh: DH,
                dl: DL
            }
        },
        ebx: EBX {
            bx: BX {
                bh: BH,
                bl: BL
            }
        },
        esp: ESP {
            sp: SP {}
        },
        ebp: EBP {
            bp: BP {}
        },
        esi: ESI {
            si: SI {}
        },
        edi: EDI {
            di: DI {}
        }
    }
}

#[cfg(test)]
mod test {
    use x86::reg::X86;

    #[test]
    fn split_borrow() {
        let mut x86 = X86::new();
        let _ah = &mut x86.eax.ax.ah;
        let _al = &mut x86.eax.ax.al;
        // must compile
    }
}
