use std::collections::HashMap;
use std::collections::HashSet;
use vm::Func;
use vm::FuncData;
use vm::Block;
use vm::BlockData;
use vm::Var;
use vm::Lval;
use vm::Rval;
use vm::Inst;
use vm::InstData;
use vm;

pub type Lvals = HashSet<Lval>;

pub struct Liveness<'func_data> {
    func_data: &'func_data FuncData,

    gens: HashMap<Block, Lvals>,
    kills: HashMap<Block, Lvals>,
    in_: HashMap<Block, Lvals>,
    out: HashMap<Block, Lvals>,
}

impl<'func_data> Liveness<'func_data> {
    pub fn new(func_data: &'func_data FuncData) -> Self {
        unimplemented!()
    }
}

pub fn gens_kills(block: &BlockData) -> (Lvals, Lvals) {
    let mut gens = Lvals::new();
    let mut kills = Lvals::new();
    for inst in &block.body {

    }
    unimplemented!()
}

pub trait Uses {
    fn uses(&self) -> Lvals {
        Lvals::new()
    }
}

pub fn uses<U>(val: &U) -> Lvals
where
    U: Uses
{
    val.uses()
}

impl Uses for Inst {
    fn uses(&self) -> Lvals {
        self.data.uses()
    }
}

impl Uses for InstData {
    fn uses(&self) -> Lvals {
        use self::InstData::*;
        match *self {
            Unary { opcode, ref arg } => {
                match opcode {
                    // all of these instructions just
                    // write to the destination, and
                    // have no other interferences
                    vm::Unary::Mov |
                    vm::Unary::Neg |
                    vm::Unary::Not |
                    vm::Unary::Push => arg.uses(),
                    // pop doesn't read from arg,
                    // although we might consider adding
                    // a "use stackslot" or etc????
                    vm::Unary::Pop => Lvals::new(),
                }
            }
            Binary { opcode, ref left, ref right } => {
                // all binary opcodes just read
                // from the arguments, no special
                // writing or interferences
                &left.uses() | &right.uses()
            }
            // XXX
            // Q: Should EAX be added to the read set here?
            // A: Only if EAX is added to the write set in defs. I think.
            CallIndirect { ref target, ref args } => {
                &hash_set!(target.clone()) | &args.iter().map(Uses::uses).fold(Lvals::new(), |acc, set| &acc | &set)
            }
            Call { ref args, .. } => {
                args.iter().map(Uses::uses).fold(Lvals::new(), |acc, set| &acc | &set)
            }
            ShiftLeftThenOr { ref arg, .. } => {
                arg.uses()
            }
            MovFuncLabel { .. } => Lvals::new(),
        }
    }
}

impl Uses for Rval {
    fn uses(&self) -> Lvals {
        match *self {
            Rval::Imm(_) => Lvals::new(),
            Rval::Lval(ref lval) => hash_set!(lval.clone()),
        }
    }
}

pub trait Defs {
    fn defs(&self) -> Lvals {
        Lvals::new()
    }
}

pub fn defs<D>(val: &D) -> Lvals
where
    D: Defs
{
    val.defs()
}

impl Defs for Inst {
    fn defs(&self) -> Lvals {
        let mut defs = hash_set!(self.dst.clone());
        defs.extend(self.data.defs());
        defs
    }
}

impl Defs for InstData {
    fn defs(&self) -> Lvals {
        unimplemented!()
    }
}
