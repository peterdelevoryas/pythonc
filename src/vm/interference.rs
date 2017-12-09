use petgraph::graphmap::UnGraphMap;
use std::collections::HashMap;
use std::collections::HashSet;
use vm::Reg;
use vm::Var;
use vm::FuncData;
use vm::Visit;
use vm;

pub struct Graph {
    graph: UnGraphMap<Node, ()>,
    unspillable: HashSet<Var>,
    colors: HashMap<Var, Color>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Node {
    Forced(Reg),
    Var(Var),
}

pub type Color = Reg;

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
    pub fn build(func: &FuncData) -> Graph {
        let mut builder = Builder::new();

        builder.visit_func(func);

        Graph {
            graph: builder.graph,
            unspillable: builder.unspillable,
            colors: builder.colors,
        }
    }
}

struct Builder {
    graph: UnGraphMap<Node, ()>,
    unspillable: HashSet<Var>,
    colors: HashMap<Var, Color>,
}

impl Builder {
    fn new() -> Self {
        Self {
            graph: UnGraphMap::new(),
            unspillable: HashSet::new(),
            colors: HashMap::new(),
        }
    }
}

impl Visit for Builder {
}
