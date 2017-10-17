#![feature(conservative_impl_trait)]
#[macro_use]
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
    Forced(Register),
    Variable(ir::Tmp),
}

impl From<Register> for Node {
    fn from(r: Register) -> Node {
        Node::Forced(r)
    }
}

impl From<ir::Tmp> for Node {
    fn from(tmp: ir::Tmp) -> Node {
        Node::Variable(tmp)
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

        // add all nodes to the graph, must be done before
        // creating edges (I think? I suppose maybe not,
        // if it's true that a variable must have been
        // referenced. Additionally, it might be totally
        // unnecessary if add_edge will add the node if it
        // doesn't already exist)
        for instr in &vm.stack {
            graph.add_referenced_variables(instr);
        }

        // add edges using liveness sets for each instruction
        let liveness = liveness::compute(vm);
        for l in liveness {
            let instr_k = &vm.stack[l.k];
            let live_after_k = &l.live_after_k;
            graph.add_edges(instr_k, live_after_k);
        }

        graph
    }

    fn new() -> Graph {
        Graph {
            graph: UnGraphMap::new(),
            unspillable: HashSet::new(),
            colors: HashMap::new(),
        }
    }

    ///
    /// This is the algorithm from the course notes:
    ///
    /// ```
    /// instr "mov _, v2"
    /// where
    ///     v2 == stack location
    /// {
    ///     // no edges to add, stack doesn't interfere with anything
    /// }
    ///
    /// instr "mov v1, v2"
    /// where
    ///     v2 != stack location
    /// {
    ///     // v2 should interfere with all values in live set
    ///     // except for 1. itself and 2. v1
    ///     // COMMENT: why not v1?
    ///     // Because it would be ok to allocate them to the
    ///     // same register in that case? I feel like that's
    ///     // only a valid optimization if the assembly is
    ///     // in static single assignment form. For example,
    ///     //     mov 1, t0
    ///     //     mov t0, t1
    ///     //     add 1, t1
    ///     //     print t0 + t1
    ///     // If we don't add an edge between t0 and t1,
    ///     // and we allocate them both to eax,
    ///     // that would become:
    ///     //     mov 1, eax
    ///     //     mov eax, eax
    ///     //     add 1, eax
    ///     //     print eax + eax
    ///     // which would print 4 instead of 3!!
    ///     // So, actually I'm going to implement it with the edge
    ///     // between v1 and v2 for now...
    ///     live_set.filter(v != v2).add_edge()
    /// }
    ///
    /// TODO the arithmetic instructions are all the same as the above mov??
    ///
    /// instr "call label" {
    ///     // Add edge between each caller-save register
    ///     // and virtual location in the live set
    ///     (eax, ecx, edx).for_each(|r| live_set.add_edge(r))
    /// }
    /// ```
    ///
    fn add_edges(&mut self, instr: &vm::Instruction, live_set: &HashSet<liveness::Val>) {
        use vm::Instruction::*;
        use vm::RValue::*;
        use vm::LValue::*;
        use liveness::Val as LiveVal;
        // it's really interesting that this import works,
        // cause we're also importing Register from LValue::*
        // and using it in pattern matching
        use trans::Register;

        match *instr {
            // Stack locations don't interfere with anything,
            // and aren't even in the graph, so there's no edges to add
            // here. And I don't think it matters that the source value
            // is, since reads in general don't affect the graph
            Mov(_, Stack(_)) |
            Neg(Stack(_)) |
            Add(_, Stack(_)) |
            Push(LValue(Stack(_))) |
            Push(Int(_)) => {}
            // Don't really need to look explicitly at rval, I don't think!
            // If it's live after this, it will be in the live set and we'll
            // add an edge to it, if it's not live or it's a constant,
            // then it won't be in the live set!
            Mov(_, Tmp(tmp)) |
            Neg(Tmp(tmp)) |
            Add(_, Tmp(tmp)) |
            Push(LValue(Tmp(tmp))) => {
                let dst = LiveVal::Virtual(tmp);
                self.add_edges_to_all(dst, live_set);
            }
            // This is the same thing as above, just with registers
            // I chose to do it like this to try to take advantadge
            // of the exhaustive variant matching check Rust gives
            Mov(_, Register(r)) |
            Neg(Register(r)) |
            Add(_, Register(r)) |
            Push(LValue(Register(r))) => {
                let dst = LiveVal::Register(r);
                self.add_edges_to_all(dst, live_set);
            }
            Call(_) => {
                let caller_save_registers = &[Register::EAX, Register::ECX, Register::EDX];

                // How to handle the edge from eax to itself?
                // Because the live set for a call always
                // contains eax...I guess we just say don't add it?
                // Also, this still ends up with like eax -- ecx and eax -- edx,
                // so idk I guess just ignore edges between 2 forced
                // nodes?
                for &v in live_set {
                    for &r in caller_save_registers {
                        if LiveVal::Register(r) == v {
                            assert_eq!(r, Register::EAX, "expected eax");
                            continue;
                        }
                        self.add_edge(LiveVal::Register(r), v);
                    }
                }
            }
        }
    }

    fn add_edges_to_all(&mut self, val: liveness::Val, live_set: &HashSet<liveness::Val>) {
        for &v in live_set {
            // filter out self
            if v == val {
                continue;
            }
            self.add_edge(val, v);
        }
    }


    /// This won't hurt anything if the edge already exists. At least,
    /// it shouldn't, I hope... O_O
    fn add_edge<L: Into<Node>, R: Into<Node>>(&mut self, l: L, r: R) {
        let l = l.into();
        let r = r.into();
        assert_ne!(l, r, "trying to add an edge from a node to itself");
        self.graph.add_edge(l, r, ());
    }

    /// Adds all variables referenced in the
    /// instruction to the internal graph.
    /// If the variable already exists in the graph,
    /// it is not modified. If an LValue
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
    fn add_referenced_variables(&mut self, instr: &vm::Instruction) {
        use vm::Instruction::*;
        use vm::RValue::*;
        use vm::LValue::*;
        match *instr {
            // If moving from a stack location to any destination,
            // add the destination as an unspillable variable
            // I don't think it's necessary to match Tmp, Stack and
            // make tmp unspillable in that case, since this case is
            // handled: as long as 1 of the 2 is handled, it's ok?
            // Not entirely sure!! If this is not correct, Add
            // probably also needs to be changed
            Mov(LValue(Stack(_)), Tmp(tmp)) => {
                self.add_unspillable(tmp);
            }
            // I don't think this should be possible?? If it occurs,
            // panic so that we can debug it
            Mov(LValue(Stack(_)), Stack(_)) => panic!("mov stack, stack encountered in virtual asm!"),
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

    /// This just exists so that we don't have to write
    /// RValue::Int(i) => {} everywhere there's an rval.
    /// This uses add_lval internally to handle lval's,
    /// see that function's documentation.
    fn add_rval(&mut self, rval: vm::RValue) {
        use vm::RValue::*;
        match rval {
            Int(_) => {}
            LValue(lval) => self.add_lval(lval),
        }
    }

    /// This does not consider context at all,
    /// so it always adds tmp's as spillable.
    fn add_lval(&mut self, lval: vm::LValue) {
        use vm::LValue::*;
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

    pub fn run_dsatur(&mut self) -> DSaturResult {
        use Register::*;

        let registers = set!(EAX, EBX, ECX, EDX, ESI, EDI);

        // I think with non-lexical lifetimes, this can be a while let Some(u)
        loop {
            let uncolored_nodes: Vec<ir::Tmp> = self.uncolored_nodes().collect();
            //let (u, r) = if let Some(u) = self.uncolored_nodes().max_by_key(|&tmp| self.saturation(tmp)) {
            let (u, r) = if let Some(&u) = uncolored_nodes.iter().max_by_key(
                |&tmp| self.saturation(*tmp),
            )
            {
                let diff: HashSet<Register> = registers
                    .difference(&self.adjacent_colors(u))
                    .map(|&r| r)
                    .collect();
                let r = match diff.iter().next() {
                    //let r = match self.adjacent_colors(u).difference(&registers).next() {
                    Some(&r) => r,
                    None => return DSaturResult::Spill(u),
                };
                (u, r)
            } else {
                // no more uncolored nodes
                break;
            };
            self.write_color(u, r);
        }
        DSaturResult::Success
    }

    fn write_color(&mut self, tmp: ir::Tmp, color: Color) {
        assert!(
            !self.colors.contains_key(&tmp),
            "A color should only be written once"
        );
        self.colors.insert(tmp, color);
    }

    fn uncolored_nodes<'graph>(&'graph self) -> impl 'graph + Iterator<Item = ir::Tmp> {
        self.graph
            .nodes()
            .filter_map(|n| match n {
                Node::Variable(tmp) => Some(tmp),
                Node::Forced(_) => None,
            })
            .filter_map(move |tmp| match self.tmp_color(tmp) {
                None => Some(tmp),
                Some(_) => None,
            })
    }

    fn saturation<N: Into<Node>>(&self, node: N) -> Saturation {
        let node = node.into();
        match node {
            Node::Variable(tmp) => {
                let unspillable = self.unspillable.contains(&tmp);
                let count = self.count_adjacent_colored(node);
                if unspillable {
                    Saturation::Unspillable(count)
                } else {
                    Saturation::Spillable(count)
                }
            }
            Node::Forced(_) => Saturation::Forced,
        }
    }

    fn adjacent_colors<N: Into<Node>>(&self, node: N) -> HashSet<Color> {
        let node = node.into();
        self.graph
            .neighbors(node)
            .filter_map(|n| self.node_color(n))
            .collect()
    }

    fn count_adjacent_colored<N: Into<Node>>(&self, node: N) -> usize {
        let node = node.into();
        self.graph
            .neighbors(node)
            .map(|n| if self.node_color(n).is_some() { 1 } else { 0 })
            .sum()
    }

    fn tmp_color(&self, tmp: ir::Tmp) -> Option<Color> {
        self.colors.get(&tmp).map(|&c| c)
    }

    fn node_color<N: Into<Node>>(&self, node: N) -> Option<Color> {
        let node = node.into();
        match node {
            Node::Variable(tmp) => self.tmp_color(tmp),
            Node::Forced(r) => Some(r),
        }
    }

    pub fn assign_homes(&self, mut vm: vm::Program) -> vm::Program {
        for instr in vm.stack.iter_mut() {
            let tmps = instr.tmps();
            for &tmp in &tmps {
                let color = self.tmp_color(tmp).expect("tmp is not colored");
                //println!("{} := {}", tmp, trans::Att(&color));
                instr.replace_with(tmp, vm::LValue::Register(color));
            }
        }
        vm
    }
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
    Spill(ir::Tmp),
}

#[cfg(test)]
mod tests {
    use super::Saturation;

    #[test]
    fn saturation_ordering() {
        let forced = Saturation::Forced;
        let unspillable_1 = Saturation::Unspillable(1);
        let unspillable_2 = Saturation::Unspillable(2);
        let spillable_1 = Saturation::Spillable(1);
        let spillable_2 = Saturation::Spillable(2);

        assert!(forced > unspillable_1);
        assert!(unspillable_2 > unspillable_1);
        assert!(unspillable_1 > spillable_2);
        assert!(spillable_2 > spillable_1);
    }
}
