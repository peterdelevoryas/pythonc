use ssa::*;
use reg::*;
use stack::Slot;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LiveVal {
    Reg(Reg),
    Stack(Slot),
    Value(Value),
}
pub type LiveSet = HashSet<LiveVal>;

pub struct LiveSets {
    pub gens: HashMap<Block, LiveSet>,
    pub kills: HashMap<Block, LiveSet>,
    pub in_: HashMap<Block, LiveSet>,
    pub out: HashMap<Block, LiveSet>,
}

impl LiveSets {
    pub fn new(function: &FunctionData) -> Self {
        let (gens, kills) = {
            let (mut gens, mut kills) = (map!(), map!());
            for (block, block_data) in &function.blocks {
                let (g, k) = gens_kills(function, block_data);
                gens.insert(block, g);
                kills.insert(block, k);
            }
            (gens, kills)
        };

        let mut in_: HashMap<Block, LiveSet> = HashMap::new();
        let mut out: HashMap<Block, LiveSet> = HashMap::new();
        for (b, _) in &function.blocks {
            in_.insert(b, set!());
            out.insert(b, set!());
        }

        loop {
            let mut change_made = false;
            for n in function.reverse_order() {
                let in_n = &gens[&n] | &(&out[&n] - &kills[&n]);
                if in_n != in_[&n] {
                    change_made |= true;
                    in_.insert(n, in_n);
                }

                let mut out_n = LiveSet::new();
                for s in function.block(n).successors() {
                    out_n.extend(in_[&s].clone());
                }
                if out_n != out[&n] {
                    change_made |= true;
                    out.insert(n, out_n);
                }
            }
            
            if !change_made {
                break;
            }
        }

        LiveSets { gens, kills, in_, out }
    }
}

pub fn gens_kills(function: &FunctionData,
                  block: &BlockData) -> (LiveSet, LiveSet)
{
    let mut gens = LiveSet::new();
    let mut kills = LiveSet::new();
    for &value in &block.body {
        let uses = uses(&function.values[value]);
        let defs = &defs(&function.values[value]) | &set!(value.into());
        for &used in &uses {
            if !kills.contains(&used) {
                gens.insert(used.into());
            }
        }
        for &def in &defs  {
            kills.insert(def.into());
        }
    }

    (gens, kills)
}

impl From<Value> for LiveVal {
    fn from(value: Value) -> Self {
        LiveVal::Value(value)
    }
}

pub fn uses(expr: &Expr) -> LiveSet {
    match *expr {
        Expr::Unary { arg, .. } => set!(arg.into()),
        Expr::Binary { left, right, .. } => set!(left.into(), right.into()),
        Expr::Call { ref args, .. } => args.iter().map(|&value| value.into()).collect(),
        Expr::ShiftLeftThenOr { arg, .. } => set!(arg.into()),
        Expr::Phi(ref phi) => phi.args.iter().map(|&value| value.into()).collect(),
        Expr::LoadParam { .. } |
        Expr::Undef |
        Expr::Const(_) |
        Expr::Function(_) => set!(),
    }
}

pub fn defs(expr: &Expr) -> LiveSet {
    match *expr {
        Expr::Binary { opcode: Binary::Sete, .. } |
        Expr::Binary { opcode: Binary::Setne, .. } => set!(),

        Expr::Unary { .. } |
        Expr::Binary { .. } |
        Expr::Phi(_) |
        Expr::LoadParam { .. } |
        Expr::Undef |
        Expr::Const(_) |
        Expr::ShiftLeftThenOr { .. } |
        Expr::Function(_) => set!(),

        Expr::Call { .. } => {
            ::reg::caller_save().into_iter().map(LiveVal::Reg).collect()
        }
    }
}
