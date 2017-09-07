use self::sealed::Sealed;

pub trait Reg8: Sealed {}
pub trait Reg16: Sealed {}
pub trait Reg32: Sealed {}

mod sealed {
    pub trait Sealed {}
}

macro_rules! reg {
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
            )*
        )*
    };

    (
        $Reg32:ident {
            $(
                $reg16:ident: $Reg16:ident
            ),*
        }
    ) => {
        reg! {
            $Reg32 {
                $(
                    $reg16: $Reg16 {}
                ),*
            }
        }
    }
}

reg! {
    EAX {
        ax: AX {
            ah: AH,
            al: AL
        }
    }
}

reg! {
    ECX {
        cx: CX {
            ch: CH,
            cl: CL
        }
    }
}

reg! {
    EDX {
        dx: DX {
            dh: DH,
            dl: DL
        }
    }
}

reg! {
    EBX {
        bx: BX {
            bh: BH,
            bl: BL
        }
    }
}

reg! {
    ESP {
        sp: SP
    }
}

reg! {
    EBP {
        bp: BP
    }
}

reg! {
    ESI {
        si: SI
    }
}

reg! {
    EDI {
        di: DI
    }
}

#[derive(Debug)]
pub struct Set {
    pub eax: EAX,
    pub ecx: ECX,
    pub edx: EDX,
    pub ebx: EBX,
    pub esp: ESP,
    pub ebp: EBP,
    pub esi: ESI,
    pub edi: EDI,
}

impl Set {
    pub fn new() -> Set {
        Set {
            eax: EAX::new(),
            ecx: ECX::new(),
            edx: EDX::new(),
            ebx: EBX::new(),
            esp: ESP::new(),
            ebp: EBP::new(),
            esi: ESI::new(),
            edi: EDI::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use x86::reg::Set;

    #[test]
    fn split_borrow() {
        let mut set = Set::new();
        let ah = &mut set.eax.ax.ah;
        let al = &mut set.eax.ax.al;
        // must compile
    }
}
