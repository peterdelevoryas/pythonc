extern crate liveness;
extern crate python_ir as ir;
extern crate python_vm as vm;
extern crate python_trans as trans;
extern crate petgraph;

use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Tmp(ir::Tmp),
    Register(trans::Register),
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

        unimplemented!()
    }

    fn create_vertices(&mut self, vm: &vm::Program) {
        use vm::Instr::*;
        // create tmp nodes and register nodes
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
