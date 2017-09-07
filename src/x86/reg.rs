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
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $Reg32 {
            $(
                pub $reg16: $Reg16,
            ),*
        }

        $(
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            pub struct $Reg16 {
                $(
                    pub $reg8: $Reg8
                ),*
            }
        )*

        $($(
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            pub struct $Reg8;
        )*)*
    };

    (
        $Reg32:ident {
            $(
                $reg16:ident: $Reg16:ident
            ),*
        }
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $Reg32 {
            $(
                pub $reg16: $Reg16,
            ),*
        }

        $(
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            pub struct $Reg16;
        )*
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
    eax: EAX,
    ecx: ECX,
    edx: EDX,
    ebx: EBX,
    esp: ESP,
    ebp: EBP,
    esi: ESI,
    edi: EDI,
}
