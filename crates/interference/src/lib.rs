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
    /// it is not modified. If an LVal
    /// is a stack location, it is not added
    /// to the graph.
    ///
    /// If the variables referenced in the instruction
    /// have already been added to the graph, then
    /// they won't be affected, unless they need to
    /// be added as unspillable, in which case
    /// they are added to the unspillable set. For example,
    ///
    /// ```
    /// 0. mov stack_0, tmp_7
    /// 1. mov tmp_7, stack_1
    /// ```
    /// If you do `add_referenced_variables(0)`,
    /// `tmp_7` will be added to the graph and to the unspillable
    /// set. `add_referenced_variables(1)` will then try to
    /// add `tmp_7` as a spillable variable, but this will
    /// not remove it from the unspillable set, so everything
    /// is ok.
    ///
    fn add_referenced_variables(&mut self, instr: &vm::Instr) {
        use vm::Instr::*;
        use vm::RVal::*;
        use vm::LVal::*;
        match *instr {
            // If moving from a stack location to any destination,
            // add the destination as an unspillable variable
            // I don't think it's necessary to match Tmp, Stack and
            // make tmp unspillable in that case, since this case is
            // handled: as long as 1 of the 2 is handled, it's ok?
            // Not entirely sure!! If this is not correct, Add
            // probably also needs to be changed
            Mov(LVal(Stack(_)), Tmp(tmp)) => {
                self.add_unspillable(tmp);
            }
            // I don't think this should be possible?? If it occurs,
            // panic so that we can debug it
            Mov(LVal(Stack(_)), Stack(_)) => {
                panic!("mov stack, stack encountered in virtual asm!")
            }
            // add_lval and add_rval don't consider context, so they
            // only add tmp's as spillable (forced registers don't
            // change depending on context)
            Mov(rval, lval) => {
                self.add_rval(rval);
                self.add_lval(lval);
            }
            // For Neg, Add, and Push, pretty sure we
            // can
            Neg(lval) => self.add_lval(lval),
            Add(rval, lval) => self.add_rval(rval),
            Push(rval) => self.add_rval(rval),
            // Nothing gets referenced in a Call right now
            Call(ref label) => {}
        }
    }

    fn write_color(&mut self, tmp: ir::Tmp, color: Color) {
        assert!(
            !self.colors.contains_key(&tmp),
            "A color should only be written once"
        );
        self.colors.insert(tmp, color);
    }

    /// This just exists so that we don't have to write
    /// RVal::Int(i) => {} everywhere there's an rval.
    /// This uses add_lval internally to handle lval's,
    /// see that function's documentation.
    fn add_rval(&mut self, rval: vm::RVal) {
        use vm::RVal::*;
        match rval {
            Int(_) => {},
            LVal(lval) => self.add_lval(lval),
        }
    }

    /// This does not consider context at all,
    /// so it always adds tmp's as spillable.
    fn add_lval(&mut self, lval: vm::LVal) {
        use vm::LVal::*;
        match lval {
            Tmp(tmp) => self.add_spillable(tmp),
            Register(r) => self.add_forced(r),
            // We don't add stack locations to the graph
            Stack(_) => {}
        }
    }

    /// If `tmp` already exists in the graph,
    /// but is not unspillable, then this will
    /// also make it unspillable.
    fn add_unspillable(&mut self, tmp: ir::Tmp) {
        self.add_node(Node::Variable(tmp));
        self.unspillable.insert(tmp);
    }

    /// This will only add the tmp to the graph.
    /// If the tmp is already in the graph,
    /// and unspillable, then this will not make
    /// it spillable
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
