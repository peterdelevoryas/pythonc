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

pub fn dominance_frontiers(all_dominators: &AllDominators) -> DominanceFrontiers {
    let mut dfp = DominanceFrontiers::new();
    let mut df = DominanceFrontiers::new();
    let mut df_local = DominanceFrontiers::new();
    let mut df_up = DominanceFrontiers::new();
    unimplemented!()
}

pub fn convert_to_ssa(func: FuncData) -> FuncData {
    unimplemented!()
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
