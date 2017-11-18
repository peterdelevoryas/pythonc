use vasm::Reg;
use vasm::Lval;
use vasm::Inst;
use vasm::Function;
use vasm::Block;
use explicate::Var;
use std::collections::HashSet;
use std::fmt;

/// Computes live sets for each instruction
/// in a function
pub fn liveness(block: &Block) -> Vec<LiveSet> {
    let mut live_sets: Vec<LiveSet> = Vec::new();
    let mut live_after: HashSet<Lval> = HashSet::new();
    let mut live_before: HashSet<Lval>;

    for inst in block.insts.iter().rev() {
        use self::Inst::*;

        if let If(cond, ref then, ref else_) = *inst {
            unimplemented!("if statement liveness unimplemented")
        }
        let w = inst.write_set();
        let r = inst.read_set();

        live_before = (&live_after - &w)
            .union(&r)
            .map(|&lval| lval)
            .collect();

        live_sets.push(LiveSet {
            inst: inst,
            live_after: live_after.clone(),
        });

        live_after = live_before;
    }

    live_sets.reverse();
    live_sets
}

#[derive(Debug, Clone)]
pub struct LiveSet<'inst> {
    pub inst: &'inst Inst,
    pub live_after: HashSet<Lval>,
}

impl<'inst> fmt::Display for LiveSet<'inst> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{inst} // liveset: (", inst=self.inst)?;
        let live_after: Vec<Lval> = self.live_after.iter().map(|&lval| lval).collect();
        if !live_after.is_empty() {
            write!(f, "{}", live_after[0])?;
            for lval in &live_after[1..] {
                write!(f, ", {}", lval)?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}
