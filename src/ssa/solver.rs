use petgraph::graphmap::UnGraphMap;
use std::collections::HashMap;
use std::collections::HashSet;
use ssa::*;
use reg::Reg;
use stack::Slot;
use reg::Reg::*;
use std::fmt;
use std::ops::Deref;

#[derive(Debug)]
pub struct Graph {
    graph: UnGraphMap<Node, ()>,
    unspillable: HashSet<Node>,
    pub colors: HashMap<Node, Color>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Node {
    Reg(Reg),
    Value(Value),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Color {
    Reg(Reg),
    Stack(Slot),
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
    Spill(Value),
}

pub struct Coloring {
    pub next_spill: usize,
    pub colors: HashMap<Node, Color>,
}

impl Coloring {
    pub fn color(&self, value: Value) -> Color {
        match self.colors.get(&Node::Value(value)) {
            Some(&color) => color,
            None => panic!("no color for {}", value),
        }
    }
}

impl Graph {
    fn new() -> Graph {
        let graph = UnGraphMap::new();
        let unspillable = set!(EAX, EBX, ECX, EDX, ESI, EDI)
            .into_iter().map(|reg| Node::Reg(reg)).collect();
        let colors = set!(EAX, EBX, ECX, EDX, ESI, EDI)
            .into_iter().map(|reg| (Node::Reg(reg), Color::Reg(reg))).collect();
        Graph { graph, unspillable, colors }
    }

    pub fn build(function: &FunctionData, colors: &HashMap<Node, Color>) -> Graph {
        let mut graph = Graph::new();

        for &reg in &[EAX, EBX, ECX, EDX, ESI, EDI] {
            graph.add_reg(reg);
        }

        for (&node, &color) in colors {
            graph.write_color(node, color);
        }

        for (_, block) in &function.blocks {
            for &value in &block.body {
                graph.add_spillable(value);
            }
            for &value in &block.body {
                use ssa::Unary::Mov;
                use ssa::Binary::Sete;
                use ssa::Binary::Setne;
                use ssa::Expr::Unary;
                use ssa::Expr::Binary;

                match function.values[value] {
                    Binary { opcode: Sete, .. } |
                    Binary { opcode: Setne, .. } => {
                        // sete and setne need to use an 8-bit register,
                        // esi and edi don't have lower registers, so can't use them!
                        graph.add_interference(Node::Value(value), Node::Reg(ESI));
                        graph.add_interference(Node::Value(value), Node::Reg(EDI));
                    }
                    _ => {}
                }
            }
        }

        let livesets = ::ssa::LiveSets::new(function);

        // See this set of slides for info on algorithm! (Page 6/10)
        // https://www2.cs.arizona.edu/~collberg/Teaching/553/2011/Handouts/Handout-23.pdf
        for (b, block) in &function.blocks {
            let mut live = livesets.out[&b].clone();
            for &value in block.body.iter().rev() {
                let expr = &function.values[value];
                let expr_defs = &::ssa::liveness::defs(expr) | &hash_set!(value.into());
                for &d in &expr_defs {
                    for l in &live | &expr_defs {
                        graph.add_interference(l.into(), d.into());
                    }
                    //for u in inst.uses() {
                        //graph.add_interference(u, d.clone());
                    //}
                }
                let uses = ::ssa::liveness::uses(b, expr);
                live = &uses | &(&live - &expr_defs);
            }
        }

        graph
    }

    fn add_interference(&mut self, left: Node, right: Node) {
        self.add_edge(left, right);
    }

    fn add_edge(&mut self, l: Node, r: Node) {
        if l != r {
            self.graph.add_edge(l, r, ());
        }
    }

    fn add_spillable(&mut self, value: Value) {
        self.add_node(Node::Value(value));
    }

    fn add_unspillable(&mut self, value: Value) {
        self.add_node(Node::Value(value));
        self.unspillable.insert(Node::Value(value));
    }

    fn add_reg(&mut self, reg: Reg) {
        self.add_node(Node::Reg(reg));
    }

    fn add_node(&mut self, node: Node) {
        self.graph.add_node(node);
    }

    fn write_color(&mut self, node: Node, color: Color) {
        assert!(
            !self.colors.contains_key(&node),
            "Writing color for node that already has a color"
        );
        self.colors.insert(node, color);
    }

    pub fn run_dsatur(&mut self, coloring: &mut Coloring) -> DSaturResult {
        let reg_pool = set!(EAX, EBX, ECX, EDX, ESI, EDI);

        loop {
            let uncolored_nodes: Vec<Node> = self.uncolored_nodes().collect();
            let (u, r) = if let Some(u) = uncolored_nodes.into_iter().max_by_key(|&node| self.saturation(node)) {
                let free_regs = &reg_pool - &self.adjacent_registers(u);
                let r = match free_regs.iter().next() {
                    Some(&r) => Color::Reg(r),
                    None => {
                        let u = match u {
                            Node::Value(value) => value,
                            _ => panic!()
                        };
                        println!("spilling {}", u);
                        return DSaturResult::Spill(u)
                    }
                };
                (u, r)
            } else {
                break;
            };
            self.write_color(u, r);
        }

        DSaturResult::Success
    }

    pub fn uncolored_nodes<'graph>(&'graph self) -> impl 'graph + Iterator<Item=Node> {
        self.graph
            .nodes()
            .filter_map(move |node| match self.node_color(node) {
                None => Some(node),
                Some(_) => None,
            })
    }

    fn saturation(&self, node: Node) -> Saturation {
        let unspillable = self.unspillable.contains(&node);
        let count = self.count_adjacent_colored(node);
        if unspillable {
            Saturation::Unspillable(count)
        } else {
            Saturation::Spillable(count)
        }
    }

    fn adjacent_registers(&self, node: Node) -> HashSet<Reg> {
        self.graph
            .neighbors(node)
            .filter_map(|n| self.node_color(n))
            .filter_map(|c| match c {
                Color::Reg(r) => Some(r),
                Color::Stack(_) => None,
            })
            .collect()
    }

    fn count_adjacent_colored(&self, node: Node) -> usize {
        self.graph
            .neighbors(node)
            .map(|n| if self.node_color(n).is_some() { 1 } else { 0 })
            .sum()
    }

    pub fn node_color(&self, node: Node) -> Option<Color> {
        self.colors.get(&node).map(|&c| c)
    }

    /*
    pub fn assign_homes(&self, func: &mut FunctionData) {
        use vm::InstData::*;
        use vm::Term::*;
        for (_, block) in &mut func.blocks {
            for inst in &mut block.body {
                self.color_lval(&mut inst.dst);
                match inst.data {
                    Unary { ref mut arg, .. } => self.color_rval(arg),
                    Binary { ref mut left, ref mut right, .. } => {
                        self.color_rval(left);
                        self.color_rval(right);
                    }
                    CallIndirect { ref mut target, ref mut args } => {
                        self.color_lval(target);
                        for arg in args.iter_mut() {
                            self.color_rval(arg);
                        }
                    }
                    Call { ref mut args, .. } => {
                        for arg in args.iter_mut() {
                            self.color_rval(arg);
                        }
                    }
                    ShiftLeftThenOr { ref mut arg, .. } => {
                        self.color_rval(arg);
                    }
                    MovFuncLabel { .. } => {}
                    Phi { ref mut lvals } => {
                        for v in lvals.iter_mut() {
                            self.color_lval(v);
                        }
                    }
                }
            }
            match block.term {
                Return { ref mut rval } => {
                    if let Some(ref mut rval) = *rval {
                        self.color_rval(rval);
                    }
                }
                Goto { .. } => {}
                Switch { ref mut cond, .. } => {
                    self.color_rval(cond);
                }
            }
        }
    }
    */
}

impl Deref for Graph {
    type Target = UnGraphMap<Node, ()>;
    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

use ssa::liveness::LiveVal;

impl From<::ssa::liveness::LiveVal> for Node {
    fn from(val: ::ssa::liveness::LiveVal) -> Self {
        match val {
            LiveVal::Reg(reg) => Node::Reg(reg),
            LiveVal::Value(value) => Node::Value(value),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Value(value) => write!(f, "{}", value),
            Node::Reg(reg) => write!(f, "{}", reg),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::Reg(r) => write!(f, "{}", r),
            Color::Stack(slot) => write!(f, "{}", slot),
        }
    }
}
