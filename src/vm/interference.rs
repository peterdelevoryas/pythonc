use petgraph::graphmap::UnGraphMap;
use std::collections::HashMap;
use std::collections::HashSet;
use vm::Reg;
use vm::Var;
use vm::FuncData;
use vm::BlockData;
use vm::Visit;
use vm::StackSlot;
use vm;

pub struct Graph {
    graph: UnGraphMap<Node, ()>,
    unspillable: HashSet<Var>,
    colors: HashMap<Var, Color>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Node {
    Reg(Reg),
    Var(Var),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Color {
    Reg(Reg),
    StackSlot(StackSlot),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Saturation {
    Spillable(usize),
    Unspillable(usize),
    Forced,
}

#[derive(Debug, Copy, Clone)]
pub enum DSaturResult {
    Success,
    Spill(Var),
}

impl Graph {
    fn new() -> Graph {
        let mut graph = UnGraphMap::new();
        let unspillable = HashSet::new();
        let colors = HashMap::new();
        Graph { graph, unspillable, colors }
    }

    pub fn build(func: &FuncData) -> Graph {
        let mut graph = Graph::new();

        for (_, block) in &func.blocks {
            let vars = referenced_vars(block);
            for var in vars {

            }
        }

        unimplemented!()
    }

    fn add_node(&mut self, node: Node) {
        self.graph.add_node(node);
    }
}

fn referenced_vars(block: &BlockData) -> HashSet<Var> {
    struct ReferencedVars {
        vars: HashSet<Var>
    }

    impl Visit for ReferencedVars {
        fn visit_inst(&mut self, i: &vm::Inst) {
            use vm::InstData::*;
            self.vars = &self.vars | &lval(&i.dst);
            self.vars = &self.vars | &inst(&i.data);
        }

        fn visit_term(&mut self, term: &vm::Term) {
            use vm::Term::*;
            match *term {
                Return { ref var } => {
                    if let Some(var) = *var {
                        self.vars = &self.vars | &hash_set!(var);
                    }
                }
                Goto { .. } => {}
                Switch { cond, .. } => {
                    self.vars = &self.vars | &hash_set!(cond);
                }
            }
        }
    }

    fn rval(rval: &vm::Rval) -> HashSet<Var> {
        match *rval {
            vm::Rval::Imm(_) => HashSet::new(),
            vm::Rval::Lval(ref v) => lval(v),
        }
    }

    fn lval(lval: &vm::Lval) -> HashSet<Var> {
        match *lval {
            vm::Lval::Var(var) => hash_set!(var),
            vm::Lval::Reg(_) => HashSet::new(),
            vm::Lval::StackSlot(_) => HashSet::new(),
        }
    }

    fn inst(inst: &vm::InstData) -> HashSet<Var> {
        unimplemented!()
    }

    let mut referenced = ReferencedVars { vars: HashSet::new() };
    referenced.visit_block(block);

    referenced.vars
}
