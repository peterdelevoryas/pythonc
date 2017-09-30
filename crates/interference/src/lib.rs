extern crate liveness;
extern crate python_ir as ir;
extern crate python_vm as vm;
extern crate python_trans as trans;
extern crate petgraph;

use std::collections::HashMap;
use std::collections::HashSet;
use liveness::Liveness;
use petgraph::graphmap::UnGraphMap;

// TODO I feel like Node and NodeVariant could be refactored
// into a single enum, but I'm not sure how


#[derive(Debug)]
pub struct Graph {
    /// Undirected graph that only contains
    /// the virtual location name (`ir::Tmp`)
    /// or the un-named, pre-colored forced-register
    /// locations.
    graph: UnGraphMap<Node, ()>,
    /// The set of unspillable virtual locations
    unspillable: HashSet<ir::Tmp>,
    /// Virtual location colors (registers), if
    /// not allocated yet, then `colors.get(tmp) == None`
    colors: HashMap<ir::Tmp, Color>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Forced(trans::Register),
    Variable(ir::Tmp),
}

// Node also needs an impl of PartialOrd and Ord for ordering
// an edge (a, b), required by UnGraphMap<Node>.
// There is no ordering that makes sense, so this always returns "eq"
use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::cmp::Ordering;
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
    }
}

/// 
/// `Color` is just the register that
/// a virtual location has been allocated.
/// Some virtual locations will be assigned stack
/// locations, however these are removed from the graph,
/// so they have no `Color` value.
///
pub type Color = trans::Register;

impl From<liveness::Val> for Node {
    fn from(val: liveness::Val) -> Node {
        match val {
            liveness::Val::Virtual(tmp) => Node::Variable(tmp),
            liveness::Val::Register(r) => Node::Forced(r),
        }
    }
}

impl Graph {
    pub fn build(vm: &vm::Program) -> Graph {
        let mut graph = Self::new();

        //graph.create_vertices(vm);
        //let liveness = liveness::compute(vm);
        //graph.add_edges(vm, &liveness);

        graph
    }

    fn new() -> Graph {
        Graph {
            graph: UnGraphMap::new(),
            unspillable: HashSet::new(),
            colors: HashMap::new(),
        }
    }

    /*
    fn add_edges(&mut self, vm: &vm::Program, liveness: &[Liveness]) {
        use vm::Instr::*;
        use liveness::Val::*;

        for liveness in liveness {
            let instr = &vm.stack[liveness.k];
            match *instr {
                // generate spillable data
                // register | tmp
                Mov(val, tmp) => {
                    let val = match val {
                        vm::Val::Int(i) => None,
                        vm::Val::Virtual(t) => Some(Virtual(t)),
                        vm::Val::Register(r) => Some(Register(r)),
                    };
                    for &v in &liveness.live_after_k {
                        // if source in live set, then skip
                        if val.is_some() && val.unwrap() == v {
                            continue
                        }
                        self.add_edge(Node::Tmp(tmp), v.into());
                    }
                }
                Neg(tmp) | Add(_, tmp) => {
                    for &v in &liveness.live_after_k {
                        self.add_edge(Node::Tmp(tmp), v.into());
                    }
                }
                Call(ref label) => {
                    for &v in &liveness.live_after_k {
                        self.add_edge(Node::Register(trans::Register::EAX), v.into());
                        self.add_edge(Node::Register(trans::Register::ECX), v.into());
                        self.add_edge(Node::Register(trans::Register::EDX), v.into());
                    }
                }
                _ => {}
            }
        }
    }

    fn add_edge(&mut self, l: Node, r: Node) {
        assert!(self.nodes.contains_key(&l));
        assert!(self.nodes.contains_key(&r));
        if l != r {
            let li = self.nodes[&l];
            let ri = self.nodes[&r];
            self.graph.add_edge(li, ri, ());
        }
    }

    fn create_vertices(&mut self, vm: &vm::Program) {
        use vm::Instr::*;

        self.add_node(Node::Register(trans::Register::EAX));
        self.add_node(Node::Register(trans::Register::ECX));
        self.add_node(Node::Register(trans::Register::EDX));

        for instr in &vm.stack {
            match *instr {
                Mov(val, tmp) => {
                    self.add_val(val);
                    self.add_node(Node::Tmp(tmp));
                }
                Neg(tmp) => self.add_node(Node::Tmp(tmp)),
                Add(val, tmp) => {
                    self.add_val(val);
                    self.add_node(Node::Tmp(tmp));
                }
                Push(val) => self.add_val(val),
                Call(ref label) => {}
            }
        }
    }

    fn add_val(&mut self, val: vm::Val) {
        use vm::Val::*;
        match val {
            Virtual(tmp) => self.add_node(Node::Tmp(tmp)),
            Register(r) => self.add_node(Node::Register(r)),
            _ => {}
        }
    }

    fn add_node(&mut self, node: Node) {
        if !self.nodes.contains_key(&node) {
            let index = self.graph.add_node(node);
            self.nodes.insert(node, index);
        }
    }
    */
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
