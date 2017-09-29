extern crate liveness;
extern crate python_ir as ir;
extern crate python_vm as vm;
extern crate python_trans as trans;
extern crate petgraph;

use std::collections::HashMap;
use liveness::Liveness;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Forced(trans::Register),
    Variable {
        color: Option<Color>,
        unspillable: bool,
        tmp: ir::Tmp,
    }
}

pub type Color = trans::Register;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Name {
    Tmp(ir::Tmp),
    Register(trans::Register),
}

impl From<liveness::Val> for Node {
    fn from(val: liveness::Val) -> Node {
        match val {
            liveness::Val::Virtual(tmp) => Node::Variable {
                
            }
            liveness::Val::Register(r) => Node::Register(r),
        }
    }
}

pub type Graph = petgraph::Graph<Node, (), petgraph::Undirected>;
pub type NodeIndex = petgraph::graph::NodeIndex;

pub struct Builder {
    graph: Graph,
    nodes: HashMap<Node, NodeIndex>,
}

impl Builder {
    pub fn build_graph(vm: &vm::Program) -> Graph {
        let mut builder = Builder {
            graph: Graph::new_undirected(),
            nodes: HashMap::new(),
        };
        
        builder.create_vertices(vm);
        let liveness = liveness::compute_vm(vm);
        builder.add_edges(vm, &liveness);

        builder.graph
    }

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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
