extern crate liveness;
extern crate python_ir as ir;
extern crate python_vm as vm;
extern crate python_trans as trans;
extern crate petgraph;

use std::collections::HashMap;
use std::collections::HashSet;
use liveness::Liveness;
use trans::Register;
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Node {
    Forced(trans::Register),
    Variable(ir::Tmp),
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

        let eax = Node::Forced(Register::EAX);
        let ecx = Node::Forced(Register::ECX);

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

    /// Adds all variables referenced in the
    /// instruction to the internal graph.
    /// If the variable already exists in the graph,
    /// it is not modified.
    fn add_referenced_variables(&mut self, instr: &vm::Instr) {
        use vm::Instr::*;
        match *instr {
            Mov(rval, lval) => {
                unimplemented!()
            }
            Neg(lval) => {
                unimplemented!()
            }
            Add(rval, lval) => {
                unimplemented!()
            }
            Push(rval) => {
                unimplemented!()
            }
            Call(ref label) => {
                unimplemented!()
            }
        }
    }

    fn write_color(&mut self, tmp: ir::Tmp, color: Color) {
        assert!(
            !self.colors.contains_key(&tmp),
            "Did you mean to overwrite the previous color for {}?",
            tmp
        );
        self.colors.insert(tmp, color);
    }

    fn add_unspillable(&mut self, tmp: ir::Tmp) {
        self.add_node(Node::Variable(tmp));
        self.unspillable.insert(tmp);
    }

    fn add_spillable(&mut self, tmp: ir::Tmp) {
        self.add_node(Node::Variable(tmp));
    }

    fn add_forced(&mut self, register: Register) {
        self.add_node(Node::Forced(register));
    }

    fn add_node(&mut self, node: Node) {
        self.graph.add_node(node);
    }

    fn add_edge(&mut self, l: Node, r: Node) -> Option<()> {
        self.graph.add_edge(l, r, ())
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
