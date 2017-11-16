use explicate as ex;
use flatten as flat;
use raise;

use explicate::Var;

use std::collections::HashMap;
use std::convert;

#[derive(Debug, Clone, Hash)]
pub enum Instr {
    Mov(Lval, Rval),
    Add(Lval, Rval),
    Push(Rval),
    Pop(Lval),
    Call(Rval),
    If(Rval, Block, Block),
}

#[derive(Debug, Clone, Hash)]
pub struct Block {
    instrs: Vec<Instr>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Rval {
    Lval(Lval),
    Const(i32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Lval {
    Reg(Reg),
    StackSlot(StackSlot),
    Var(Var),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Reg {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StackSlot {
    ebp_offset: i32,
}

pub struct Builder {
    
}
