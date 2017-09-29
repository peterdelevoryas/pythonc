extern crate liveness;
extern crate python_ir as ir;
extern crate python_vm as vm;
extern crate python_trans as trans;
extern crate petgraph;

pub enum Node {
    Tmp(ir::Tmp),
    Register(trans::Register),
}

pub type Graph = petgraph::Graph<Node, (), petgraph::Undirected>;

pub fn build_graph(vm: &vm::Program) -> Graph {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
