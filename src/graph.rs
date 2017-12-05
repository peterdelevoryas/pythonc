use vasm;
use vasm::Reg;
use vasm::Lval;
use vasm::Rval;
use vasm::Inst;
use liveness;
use liveness::LiveSet;
use explicate::Var;

use std::collections::HashMap;
use std::collections::HashSet;
use petgraph::graphmap::UnGraphMap;

pub struct Graph {
    /// Undirected graph that only contains
    /// the virtual location name (`ir::Tmp`)
    /// or the un-named, pre-colored forced-register
    /// locations.
    graph: UnGraphMap<Node, ()>,
    /// The set of unspillable virtual locations
    unspillable: HashSet<Var>,
    /// Virtual location colors (registers), if
    /// not allocated yet, then `colors.get(tmp) == None`
    colors: HashMap<Var, Color>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Node {
    Forced(Reg),
    Var(Var),
}

pub type Color = Reg;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Saturation {
    Spillable(usize),
    Unspillable(usize),
    Forced,
}

#[derive(Debug, Copy, Clone)]
pub enum DSaturResult {
    Success,
    Spill(Var),
}

impl From<Reg> for Node {
    fn from(r: Reg) -> Node {
        Node::Forced(r)
    }
}

impl From<Var> for Node {
    fn from(v: Var) -> Node {
        Node::Var(v)
    }
}

impl From<Lval> for Node {
    fn from(l: Lval) -> Node {
        match l {
            Lval::Reg(reg) => Node::Forced(reg),
            Lval::Var(var) => Node::Var(var),
            _ => panic!("cannot convert {:?} into Node", l)
        }
    }
}

impl Graph {
    pub fn build(vasm: &vasm::Function) -> Graph {
        let mut graph = Self::new();

        for inst in &vasm.block.insts {
            graph.add_referenced_variables(inst);
        }

        let (_before, liveness) = liveness::liveness(vasm);
        for live_set in liveness {
            graph.add_edges(live_set);
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

    fn add_referenced_variables(&mut self, inst: &vasm::Inst) {
        use vasm::Inst::*;
        use vasm::Lval::*;
        use vasm::Rval::*;

        match *inst {
            Mov(Var(var), Lval(StackSlot(_))) => {
                self.add_unspillable(var);
            }
            Mov(StackSlot(_), Lval(StackSlot(_))) => {
                panic!("mov stack -> stack encountered in vasm!")
            }
            Mov(lval, rval) | Add(lval, rval) | Cmp(lval, rval)
                | Or(lval, rval) | And(lval, rval) => {
                self.add_lval(lval);
                self.add_rval(rval);
            }
            Neg(lval) | Pop(lval) | CallIndirect(lval)
                | Sete(lval) | Setne(lval)
                | Shr(lval, _) | Shl(lval, _)
                | MovLabel(lval, _) => {
                self.add_lval(lval);
            }
            Push(rval) => {
                self.add_rval(rval);
            }
            Call(_) | Ret => {}
            If(lval, ref then, ref else_) => {
                self.add_lval(lval);
                for inst in &then.insts { self.add_referenced_variables(inst) }
                for inst in &else_.insts { self.add_referenced_variables(inst) }
            }
            While(lval, ref comp, ref body) => {
                self.add_lval(lval);
                for inst in &comp.insts { self.add_referenced_variables(inst) }
                for inst in &body.insts { self.add_referenced_variables(inst) }
            }
            JmpLabel(_) => unimplemented!(),
            JeqLabel(_) => unimplemented!(),
            Sub(_, _) => unimplemented!(),
            Label(_) => unimplemented!(),
        }
    }

    fn add_unspillable(&mut self, var: Var) {
        self.add_node(Node::Var(var));
        self.unspillable.insert(var);
    }

    fn add_spillable(&mut self, var: Var) {
        self.add_node(Node::Var(var));
    }

    fn add_forced(&mut self, reg: Reg) {
        self.add_node(Node::Forced(reg));
    }

    fn add_node(&mut self, node: Node) {
        self.graph.add_node(node);
    }

    fn add_lval(&mut self, lval: Lval) {
        match lval {
            Lval::Var(var) => self.add_spillable(var),
            Lval::Reg(reg) => self.add_forced(reg),
            Lval::StackSlot(_) => {}
            Lval::Param(_) => {}
        }
    }

    fn add_rval(&mut self, rval: Rval) {
        match rval {
            Rval::Imm(_) => {}
            Rval::Lval(lval) => self.add_lval(lval),
        }
    }

    fn add_edges(&mut self, live_set: LiveSet) {
        use self::Inst::*;
        use self::Rval::*;
        use self::Lval::*;

        match live_set {
            LiveSet::Inst {
                inst,
                live_after
            } => {
                match *inst {
                    Mov(StackSlot(_), _)
                        | Neg(StackSlot(_))
                        | Add(StackSlot(_), _)
                        | Push(Lval(StackSlot(_)))
                        | Pop(StackSlot(_))
                        | Sete(StackSlot(_))
                        | Setne(StackSlot(_))
                        | Or(StackSlot(_), _)
                        | And(StackSlot(_), _)
                        | Shr(StackSlot(_), _)
                        | Shl(StackSlot(_), _)
                        | MovLabel(StackSlot(_), _)
                        | Push(Imm(_))
                        | Mov(Param(_), _)
                        | Neg(Param(_))
                        | Add(Param(_), _)
                        | Push(Lval(Param(_)))
                        | Pop(Param(_))
                        | Sete(Param(_))
                        | Setne(Param(_))
                        | Or(Param(_), _)
                        | And(Param(_), _)
                        | Shr(Param(_), _)
                        | Shl(Param(_), _)
                        | MovLabel(Param(_), _)
                        | Cmp(_, _)
                        | Ret
                        => {}
                    Mov(Var(var), _) | Neg(Var(var)) | Add(Var(var), _) | Push(Lval(Var(var)))
                        | Pop(Var(var))
                        | Or(Var(var), _) | And(Var(var), _) | Shr(Var(var), _) | Shl(Var(var), _)
                        | MovLabel(Var(var), _) =>
                    {
                        self.add_interference(Var(var), &live_after);
                    }
                    Sete(Var(var)) | Setne(Var(var)) => {
                        self.add_interference(Var(var), &live_after);
                        let edi_esi = hash_set!(::vasm::Reg::EDI.into(), ::vasm::Reg::ESI.into());
                        self.add_interference(Var(var), &edi_esi);
                    }
                    Mov(Reg(reg), _) | Neg(Reg(reg)) | Add(Reg(reg), _) | Push(Lval(Reg(reg)))
                        | Pop(Reg(reg)) | Sete(Reg(reg)) | Setne(Reg(reg))
                        | Or(Reg(reg), _) | And(Reg(reg), _) | Shr(Reg(reg), _) | Shl(Reg(reg), _)
                        | MovLabel(Reg(reg), _) =>
                    {
                        self.add_interference(Reg(reg), &live_after);
                    }
                    Call(_) | CallIndirect(_) => {
                        let caller_save_registers = &[
                            ::vasm::Reg::EAX,
                            ::vasm::Reg::ECX,
                            ::vasm::Reg::EDX,
                        ];

                        for &live in &live_after {
                            for &r in caller_save_registers {
                                if ::vasm::Lval::Reg(r) == live {
                                    assert_eq!(r, ::vasm::Reg::EAX, "expected eax");
                                    continue
                                }
                                match live {
                                    Param(_) | StackSlot(_) => continue,
                                    _ => {}
                                }
                                self.add_edge(::vasm::Lval::Reg(r), live);
                            }
                        }
                    }
                    If(_, _, _) => panic!("encountered If in wrong place"),
                    While(_cond, ref _comp, ref _body) => panic!("encountered While in wrong place"),
                    JmpLabel(_) => unimplemented!(),
                    JeqLabel(_) => unimplemented!(),
                    Sub(_, _) => unimplemented!(),
                    Label(_) => unimplemented!(),
                }
            }
            LiveSet::If {
                inst,
                then,
                else_,
                ..
            } => {
                match *inst {
                    If(_, _, _) => {}
                    _ => panic!()
                }
                for live_set in then {
                    self.add_edges(live_set);
                }
                for live_set in else_ {
                    self.add_edges(live_set);
                }
            }
            LiveSet::While {
                inst,
                header,
                body,
                ..
            } => {
                match *inst {
                    While(_, _, _) => {}
                    _ => panic!(),
                }
                for live_set in header {
                    self.add_edges(live_set);
                }
                for live_set in body {
                    self.add_edges(live_set);
                }
            }
        }

    }

    fn add_interference(&mut self, lval: Lval, interfering: &HashSet<::vasm::Lval>) {
        match lval {
            Lval::Param(_) | Lval::StackSlot(_) => return,
            _ => {}
        }
        for &interfering in interfering {
            if lval == interfering {
                continue
            }
            if let Lval::Param(_) = interfering {
                continue
            }
            if let Lval::StackSlot(_) = interfering {
                continue
            }
            self.add_edge(lval, interfering);
        }
    }

    fn add_edge<L: Into<Node>, R: Into<Node>>(&mut self, l: L, r: R) {
        let l = l.into();
        let r = r.into();
        assert_ne!(l, r, "trying to add an edge from a node to itself");
        self.graph.add_edge(l, r, ());
    }

    pub fn run_dsatur(&mut self) -> DSaturResult {
        use self::Reg::*;

        let registers = hash_set!(EAX, EBX, ECX, EDX, ESI, EDI);

        loop {
            let uncolored_nodes: Vec<Var> = self.uncolored_nodes().collect();
            let (u, r) = if let Some(&u) = uncolored_nodes.iter().max_by_key(|&var| self.saturation(*var)) {
                let diff: HashSet<Reg> = registers.difference(&self.adjacent_colors(u)).map(|&r| r).collect();
                let r = match diff.iter().next() {
                    Some(&r) => r,
                    None => return DSaturResult::Spill(u)
                };
                (u, r)
            } else {
                break;
            };
            self.write_color(u, r);
        }
        DSaturResult::Success
    }

    fn write_color(&mut self, var: Var, color: Color) {
        assert!(
            !self.colors.contains_key(&var),
            "A color should only be written once"
        );
        self.colors.insert(var, color);
    }

    fn uncolored_nodes<'graph>(&'graph self) -> impl 'graph + Iterator<Item=Var> {
        self.graph
            .nodes()
            .filter_map(|n| {
                match n {
                    Node::Var(var) => Some(var),
                    Node::Forced(_) => None,
                }
            })
            .filter_map(move |var| {
                match self.var_color(var) {
                    None => Some(var),
                    Some(_) => None,
                }
            })
    }

    fn saturation<N: Into<Node>>(&self, node: N) -> Saturation {
        let node = node.into();
        match node {
            Node::Var(var) => {
                let unspillable = self.unspillable.contains(&var);
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
            .filter_map(|n| {
                self.node_color(n)
            })
            .collect()
    }

    fn count_adjacent_colored<N: Into<Node>>(&self, node: N) -> usize {
        let node = node.into();
        self.graph
            .neighbors(node)
            .map(|n| if self.node_color(n).is_some() { 1 } else { 0 })
            .sum()
    }

    fn var_color(&self, var: Var) -> Option<Color> {
        self.colors.get(&var).map(|&c| c)
    }

    fn node_color<N: Into<Node>>(&self, node: N) -> Option<Color> {
        let node = node.into();
        match node {
            Node::Var(var) => self.var_color(var),
            Node::Forced(r) => Some(r)
        }
    }

    pub fn assign_homes(&self, mut function: vasm::Function) -> vasm::Function {
        use vasm::TransformBlock;
        let mut assign_homes = AssignHomes { graph: self };
        function.block = assign_homes.block(function.block);
        function
    }
}

struct AssignHomes<'graph> {
    graph: &'graph Graph,
}

// assigns homes to registers based on coloring
impl<'graph> ::vasm::TransformBlock for AssignHomes<'graph> {
    fn lval(&mut self, lval: Lval) -> Lval {
        if let Lval::Var(var) = lval {
            let color = self.graph.var_color(var).expect("var is not colored");
            return color.into()
        }

        lval
    }
}
