use vasm;
use vasm::Reg;
use vasm::Lval;
use vasm::Rval;
use vasm::Inst;
use liveness;
use liveness::LiveSet;
use explicate::Var;

use petgraph;
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
            If(rval, ref then, ref else_) => {
                self.add_rval(rval);
                for inst in &then.insts { self.add_referenced_variables(inst) }
                for inst in &else_.insts { self.add_referenced_variables(inst) }
            }
            While(rval, ref body) => {
                self.add_rval(rval);
                for inst in &body.insts { self.add_referenced_variables(inst) }
            }
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
                        | Pop(Var(var)) | Sete(Var(var)) | Setne(Var(var))
                        | Or(Var(var), _) | And(Var(var), _) | Shr(Var(var), _) | Shl(Var(var), _)
                        | MovLabel(Var(var), _) =>
                    {
                        self.add_interference(Var(var), &live_after);
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
                                self.add_edge(::vasm::Lval::Reg(r), live);
                            }
                        }
                    }
                    If(_, _, _) => panic!("encountered If in wrong place"),
                    While(cond, ref body) => panic!("encountered While in wrong place"),
                }
            }
            LiveSet::If {
                inst,
                then_before,
                else_before,
                live_after,
                then,
                else_,
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
        }

    }

    fn add_interference(&mut self, lval: Lval, interfering: &HashSet<::vasm::Lval>) {
        for &interfering in interfering {
            if lval == interfering {
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

}
