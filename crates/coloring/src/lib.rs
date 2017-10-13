extern crate python_vm as vm;
extern crate python_ir as ir;
extern crate python_trans as trans;
extern crate liveness;
extern crate interference;
extern crate petgraph;

use trans::Register;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    Register(Register),
    Stack(usize),
    ,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Colorable {
    Unspillable(ir::Tmp),
    Regular(ir::Tmp),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Fixed {
    Stack(ir::Tmp),
    Register(Register),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Colorable(Colorable),
    Fixed(Fixed),
}

pub struct Builder {
}

impl Builder {
    pub fn new(graph: interference::Graph) -> Builder {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
