use petgraph::graphmap::UnGraphMap;
use std::collections::HashMap;
use std::collections::HashSet;
use ssa::*;
use reg::Reg;
use stack::Slot;

#[derive(Debug)]
pub struct Graph {
    graph: UnGraphMap<Node, ()>,
    unspillable: HashSet<Value>,
    colors: HashMap<Value, Color>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Node {
    Reg(Reg),
    Value(Value),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Color {
    Reg(Reg),
    Stack(Slot),
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
    Spill(Value),
}


