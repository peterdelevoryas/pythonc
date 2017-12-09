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

pub struct Liveness {
    pub gens: HashMap<Block, Lvals>,
    pub kills: HashMap<Block, Lvals>,
    pub in_: HashMap<Block, Lvals>,
    pub out: HashMap<Block, Lvals>,
}

impl Liveness {
    pub fn new(func_data: &FuncData) -> Self {
        let (gens, kills) = {
            let (mut gens, mut kills) = (HashMap::new(), HashMap::new());
            for (block, block_data) in &func_data.blocks {
                let (g, k) = gens_kills(block_data);
                gens.insert(block.clone(), g);
                kills.insert(block.clone(), k);
            }
            (gens, kills)
        };

        let mut in_p: HashMap<Block, Lvals> = HashMap::new();
        let mut out_p: HashMap<Block, Lvals> = HashMap::new();
        let mut in_: HashMap<Block, Lvals> = func_data.blocks
            .iter()
            .map(|(b, _)| (b.clone(), Lvals::new()))
            .collect();
        let mut out: HashMap<Block, Lvals> = func_data.blocks
            .iter()
            .map(|(b, _)| (b.clone(), Lvals::new()))
            .collect();

        loop {
            for (n, block) in &func_data.blocks {
                in_p.insert(n.clone(), in_[&n].clone());
                out_p.insert(n.clone(), out[&n].clone());
                let in_n = &gens[&n] | &(&out[&n] - &kills[&n]);
                in_.insert(n.clone(), in_n);
                let mut out_n = Lvals::new();
                for s in block.successors() {
                    out_n.extend(in_[&s].clone());
                }
                out.insert(n.clone(), out_n);
            }

            let done = func_data.blocks.iter().all(|(b, _)| {
                in_p[&b] == in_[&b] && out_p[&b] == out[&b]
            });
            if done {
                break;
            }
        }

        Liveness { gens, kills, in_, out }
    }
}

pub fn gens_kills(block: &BlockData) -> (Lvals, Lvals) {
    let mut gens = Lvals::new();
    let mut kills = Lvals::new();
    for inst in &block.body {
        let uses = inst.uses();
        let defs = inst.defs();
        for used in &uses {
            if !kills.contains(&used) {
                gens.insert(used.clone());
            }
        }
        for defined in &defs {
            kills.insert(defined.clone());
        }
    }

    (gens, kills)
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
        use self::InstData::*;
        match *self {
            CallIndirect { .. } |
            Call { .. } => {
                vm::reg::caller_save().into_iter().map(Lval::Reg).collect()
            }
            Unary { .. } |
            Binary { .. } |
            ShiftLeftThenOr { .. } |
            MovFuncLabel { .. } => Lvals::new(),
        }
    }
}
