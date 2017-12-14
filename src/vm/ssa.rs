use vm::FuncData;
use vm::Block;

use std::collections::HashMap;
use std::collections::HashSet;

pub fn compute_dominators(func: &FuncData) -> HashMap<Block, HashSet<Block>> {
    let mut d: HashMap<Block, HashSet<Block>> = HashMap::new();
    let mut d_p: HashMap<Block, HashSet<Block>> = HashMap::new();
    let all_blocks: HashSet<Block> = func.blocks.iter().map(|(b, _)| b.clone()).collect();
    let root = func.root().name.clone();
    println!("root: {}", root);
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
        println!("d = {:?}", d);
        println!("dp = {:?}", d_p);
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
