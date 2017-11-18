use vasm;
use vasm::Reg;
use vasm::Lval;
use vasm::Rval;
use liveness;
use liveness::LiveSet;
use explicate::Var;

use petgraph;
use std::collections::HashMap;
use std::collections::HashSet;
use petgraph::graphmap::UnGraphMap;

pub struct Graph {
    /// Undirected graph that only contains
    /// the virtual location name (`ir::Tmp`)
    /// or the un-named, pre-colored forced-register
    /// locations.
    graph: UnGraphMap<Node, ()>,
    /// The set of unspillable virtual locations
    unspillable: HashSet<Var>,
    /// Virtual location colors (registers), if
    /// not allocated yet, then `colors.get(tmp) == None`
    colors: HashMap<Var, Color>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Node {
    Forced(Reg),
    Var(Var),
}

pub type Color = Reg;

impl From<Reg> for Node {
    fn from(r: Reg) -> Node {
        Node::Forced(r)
    }
}

impl From<Var> for Node {
    fn from(v: Var) -> Node {
        Node::Var(v)
    }
}

impl Graph {
    pub fn build(vasm: &vasm::Function) -> Graph {
        let mut graph = Self::new();

        for inst in &vasm.block.insts {
            graph.add_referenced_variables(inst);
        }

        let (_before, liveness) = liveness::liveness(vasm);
        for live_set in liveness {
            graph.add_edges(live_set);
        }

        graph
    }

    fn new() -> Graph {
        Graph {
            graph: UnGraphMap::new(),
            unspillable: HashSet::new(),
            colors: HashMap::new(),
        }
    }

    fn add_edges(&mut self, live_set: LiveSet) {
        unimplemented!()
    }

    fn add_referenced_variables(&mut self, inst: &vasm::Inst) {
        use vasm::Inst::*;
        use vasm::Lval::*;
        use vasm::Rval::*;

        match *inst {
            Mov(Var(var), Lval(StackSlot(_))) => {
                self.add_unspillable(var);
            }
            Mov(StackSlot(_), Lval(StackSlot(_))) => {
                panic!("mov stack -> stack encountered in vasm!")
            }
            Mov(lval, rval) => {
                self.add_lval(lval);
                self.add_rval(rval);
            }
            Neg(lval) => self.add_lval(lval),
            _ => unimplemented!()
        }
    }

    fn add_unspillable(&mut self, var: Var) {
        unimplemented!()
    }

    fn add_spillable(&mut self, var: Var) {
        unimplemented!()
    }

    fn add_forced(&mut self, reg: Reg) {
        unimplemented!()
    }

    fn add_lval(&mut self, lval: Lval) {
        match lval {
            Lval::Var(var) => self.add_spillable(var),
            Lval::Reg(reg) => self.add_forced(reg),
            Lval::StackSlot(_) => {}
            Lval::Param(_) => {}
        }
    }

    fn add_rval(&mut self, rval: Rval) {
        match rval {
            Rval::Imm(_) => {}
            Rval::Lval(lval) => self.add_lval(lval),
        }
    }
}
