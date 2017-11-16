use explicate as ex;
use flatten as flat;
use raise;

use explicate::Var;

use std::collections::HashMap;
use std::convert;

pub enum VirtInstr {
    MOV(Src, Dst),
    ADD(Src, Dst),
    PUSH(Src),
    POP(Dst),
    CALL(Src),
    VirtualIf(Src, Vec<VirtInstr>, Vec<VirtInstr>),
    CMP(Src, Src),
    JNZ(Src),
    JZ(Src),
    SETE(Dst),
    SETNE(Dst),
}

pub enum Src {
    Dst(Dst),
    Const(i32),
}

pub enum Dst {
    Reg(Register),
    Stack(i32),
    Tmp(Var),
}

pub enum Register {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
}

