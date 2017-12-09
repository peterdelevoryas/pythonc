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

#[derive(Debug)]
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
        let graph = UnGraphMap::new();
        let unspillable = HashSet::new();
        let colors = HashMap::new();
        Graph { graph, unspillable, colors }
    }

    pub fn build(func: &FuncData) -> Graph {
        let mut graph = Graph::new();

        use vm::Reg::*;
        for &reg in &[EAX, EBX, ECX, EDX, ESI, EDI] {
            graph.add_reg(reg);
        }

        for (_, block) in &func.blocks {
            let vars = referenced_vars(block);
            for var in vars {
                if let Some(index) = func.args.iter().position(|&arg| arg == var) {
                    // Yikes, stack slot should be constructed way before this point
                    let stack_slot = StackSlot::Param { index };
                    graph.add_param(var, stack_slot);
                }
                graph.add_spillable(var);
            }
        }

        graph
    }

    fn add_spillable(&mut self, var: Var) {
        self.add_node(Node::Var(var));
    }

    fn add_unspillable(&mut self, var: Var) {
        self.add_node(Node::Var(var));
        self.unspillable.insert(var);
    }

    fn add_reg(&mut self, reg: Reg) {
        self.add_node(Node::Reg(reg));
    }

    fn add_param(&mut self, param: Var, stack_slot: StackSlot) {
        self.add_node(Node::Var(param));

    }

    fn add_node(&mut self, node: Node) {
        self.graph.add_node(node);
    }

    fn write_color(&mut self, var: Var, color: Color) {
        assert!(
            !self.colors.contains_key(&var),
            "Writing color for node that already has a color"
        );
        self.colors.insert(var, color);
    }
}

fn referenced_vars(block: &BlockData) -> HashSet<Var> {
    struct ReferencedVars {
        vars: HashSet<Var>
    }

    impl Visit for ReferencedVars {
        fn visit_inst(&mut self, i: &vm::Inst) {
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
        use vm::InstData::*;
        match *inst {
            Unary { ref arg, .. } => rval(arg),
            Binary { ref left, ref right, .. } => &rval(left) | &rval(right),
            CallIndirect { ref target, ref args } => {
                let mut vars = lval(target);
                for arg in args {
                    vars.extend(rval(arg));
                }
                vars
            }
            Call { ref args, .. } => {
                let mut vars = HashSet::new();
                for arg in args {
                    vars.extend(rval(arg));
                }
                vars
            }
            ShiftLeftThenOr { ref arg, .. } => rval(arg),
            MovFuncLabel { .. } => HashSet::new(),
        }
    }

    let mut referenced = ReferencedVars { vars: HashSet::new() };
    referenced.visit_block(block);

    referenced.vars
}
