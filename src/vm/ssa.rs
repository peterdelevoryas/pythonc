use vm::FuncData;
use vm::Block;

use std::collections::HashMap;
use std::collections::HashSet;

pub type Dominators = HashSet<Block>;
pub type AllDominators = HashMap<Block, Dominators>;
pub type DominanceFrontier = HashSet<Block>;
pub type DominanceFrontiers = HashMap<Block, DominanceFrontier>;

pub fn compute_dominators(func: &FuncData) -> HashMap<Block, HashSet<Block>> {
    let mut d: HashMap<Block, HashSet<Block>> = HashMap::new();
    let mut d_p: HashMap<Block, HashSet<Block>> = HashMap::new();
    let all_blocks: HashSet<Block> = func.blocks.iter().map(|(b, _)| b.clone()).collect();
    let root = func.root().name.clone();
    for (block, _) in &func.blocks {
        if *block == root {
            d.insert(root.clone(), hash_set!(root.clone()));
            d_p.insert(root.clone(), hash_set!(root.clone()));
            continue;
        }
        d.insert(block.clone(), all_blocks.clone());
        d_p.insert(block.clone(), all_blocks.clone());
    }

    loop {
        for (n, data) in &func.blocks {
            if n == &root {
                continue
            }
            d_p.insert(n.clone(), d[&n].clone());

            // D[n] = {n} | (D[pred[n]] intersection)
            let pred: Vec<Block> = data.pred.iter().map(|b| b.clone()).collect();
            assert!(!pred.is_empty());
            let mut intersect = d[&pred[0]].clone();
            for p in &pred[1..] {
                intersect = &intersect & &d[p];
            }
            d.insert(n.clone(), &hash_set!(n.clone()) | &intersect);
        }
        let done = func.blocks.iter().all(|(b, _)| {
            d_p[&b] == d[&b]
        });
        if done {
            break;
        }
    }

    d
}

pub fn idom(all_dominators: &AllDominators, n: Block) -> Block {
    let n_dominators = &all_dominators[&n];
    for n_dominator in n_dominators {
        if *n_dominator == n {
            continue;
        }
        // if n_dominator is a dominator of any of the other n_dominators, then not idom
        if n_dominators.iter().any(|d| *d != n && d != n_dominator && all_dominators[d].contains(n_dominator)) {
            continue
        }
        return n_dominator.clone()
    }
    panic!("Could not find idom for block {}!", n)
}

pub fn children(n: Block, all: &AllDominators, func: &FuncData) -> HashSet<Block> {
    let mut children = HashSet::new();
    for (b, _) in &func.blocks {
        if *b == func.root().name {
            continue;
        }
        if idom(all, b.clone()) == n {
            children.insert(b.clone());
        }
    }
    children
}

/// call this initially on the root of the cfg
pub fn compute_dominance_frontier(n: Block, all: &AllDominators, func: &FuncData, df: &mut DominanceFrontiers) {
    let mut s = DominanceFrontier::new();
    for y in func.block(&n).successors() {
        if idom(all, y.clone()) != n {
            s = &s | &hash_set!(y.clone());
        }
    }
    for c in children(n.clone(), all, func) {
        compute_dominance_frontier(c.clone(), all, func, df);
        for w in &df[&c] {
            // XXX AAAAAAAAHHHHHHHHHHHHHHHHHHHHHHHH Is this because we care
            // that it doesn't strictly dominate????????????????? Adding
            // w == n makes the example from the textbook add the second
            // block to the dominance frontier of itself, so I think so!!
            if !all[&w].contains(&n) || *w == n {
                s = &s | &hash_set!(w.clone());
            }
        }
    }
    df.insert(n.clone(), s);
}

pub fn place_phi(func: &mut FuncData) {
    use vm::liveness::Lvals;
    use vm::liveness::Defs;
    use vm::Lval;
    use vm::Var;

    let ds = compute_dominators(func);
    let mut df = DominanceFrontiers::new();
    compute_dominance_frontier(func.root().name.clone(), &ds, func, &mut df);
    // apparently the first block must contain def of every var???!!?!?
    let all_vars: HashSet<Var> = func.blocks.iter()
        .flat_map(|(_, data)| {
            ::vm::interference::referenced_vars(data).into_iter()
        })
        .collect();

    let a_orig: HashMap<Block, HashSet<Var>> = func.blocks.iter()
        .map(|(n, b)| {
            if *n == func.root().name {
                (n.clone(), all_vars.clone())
            } else {
                let var_defs: HashSet<Var> = b.defs().into_iter().filter_map(|lval| {
                    match lval { Lval::Var(v) => Some(v), _ => None }
                }).collect();
                (n.clone(), var_defs)
            }
        })
        .collect();
    let mut defsites: HashMap<Var, HashSet<Block>> = HashMap::new();
    for (n, _) in &mut func.blocks {
        for &a in &a_orig[n] {
            if let Some(set) = defsites.get_mut(&a) {
                set.insert(n.clone());
            }
            defsites.insert(a, hash_set!(n.clone()));
        }
    }

    // blocks that var a needs to have phi function in
    let mut a_phi: HashMap<Var, HashSet<Block>> = HashMap::new();
    for (a, _) in &defsites {
        a_phi.insert(*a, HashSet::new());
    }
    for (a, worklist) in defsites {
        let mut worklist: Vec<Block> = worklist.into_iter().collect();
        while !worklist.is_empty() {
            let n = worklist.pop().unwrap();
            for y in &df[&n] {
                if !a_phi[&a].contains(y) {
                    // insert phi node
                    let n_pred = func.block(y).pred.len();
                    let phi = ::vm::Inst {
                        dst: Lval::Var(a),
                        data: ::vm::InstData::Phi {
                            lvals: ::std::iter::repeat(Lval::Var(a)).take(n_pred).collect(),
                        },
                    };
                    func.block_mut(y).body.insert(0, phi);
                    a_phi.get_mut(&a).unwrap().insert(y.clone());
                    if !a_orig[&y].contains(&a) {
                        worklist.push(y.clone());
                    }
                }
            }
        }
    }
}

pub fn convert_to_ssa(mut func: FuncData) -> FuncData {
    place_phi(&mut func);
    func
}

#[cfg(test)]
mod tests {
    use vm::Block;
    use vm::BlockData;

    /*
    #[test]
    fn dominators() {
        let b0 = Block::new("b0".into(), 0);
        let b1 = Block::new("b1".into(), 1);
        let b2 = Block::new("b2".into(), 2);

        let b0_data = BlockData {
            name: b0.clone(),
            body: vec![],
            term: Term::Goto { block: b1.clone() },
        };

        let b1_data = BlockData {
            name: b1.clone(),
            body: vec![],
            term: Term::Goto
        };
    }
    */
}
