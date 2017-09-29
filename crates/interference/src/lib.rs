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
    tmps: HashMap<Node, NodeIndex>,
}

impl Builder {
    pub fn build_graph(vm: &vm::Program) -> Graph {
        let mut builder = Builder {
            graph: Graph::new_undirected(),
            tmps: HashMap::new(),
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

                }
                Neg(tmp) => {}
                Add(val, tmp) => {}
                Push(val) => {}
                Call(ref label) => {}
            }
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
