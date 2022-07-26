use bitvec::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TreeRoot {
    pub lhs: HuffTree,
    pub rhs: HuffTree,
}

#[derive(Debug)]
pub enum HuffTree {
    Leaf(char),
    NonLeaf(Box<TreeRoot>),
}

pub fn decode(entry: &HuffTree, bytes: &BitVec) -> String {
    match entry {
        // A slightly degenerate case of a huffman tree with a single node.
        HuffTree::Leaf(char) => bytes.iter().map(|_| *char).collect(),
        HuffTree::NonLeaf(entry) => {
            let mut ret = "".to_string();
            let mut tree_pos: &TreeRoot = entry;
            for b in bytes {
                let branch = if *b { &tree_pos.lhs } else { &tree_pos.rhs };
                tree_pos = match branch {
                    HuffTree::NonLeaf(root) => root,
                    HuffTree::Leaf(lval) => {
                        ret.push(*lval);
                        entry
                    }
                };
            }
            ret
        }
    }
}

pub fn encode(htree: &HuffTree, value: &String) -> BitVec {
    let branches = branches(htree);
    encode_aux(value, &branches)
}

fn encode_aux(value: &String, encoder: &HashMap<char, BitVec>) -> BitVec {
    value
        .chars()
        .flat_map(|c| encoder.get(&c).unwrap().clone())
        .collect()
}

// We iterate of a tree building all its branches. The algorithm
// goes through a stack of nodes and direction indicators.
fn branches(entry: &HuffTree) -> HashMap<char, BitVec> {
    enum BranchesInstruction<'a> {
        NextTree(&'a HuffTree),
        Left,
        Right,
    }

    let mut rtn: HashMap<char, BitVec> = HashMap::new();
    let mut current_branch: BitVec = BitVec::new();
    let mut tree_stack: Vec<BranchesInstruction> = vec![BranchesInstruction::NextTree(entry)];
    loop {
        let current_tree = tree_stack.pop();
        match current_tree {
            None => {
                break rtn;
            }
            Some(BranchesInstruction::NextTree(HuffTree::Leaf(lval))) => {
                rtn.insert(*lval, current_branch.clone());
            }
            Some(BranchesInstruction::NextTree(HuffTree::NonLeaf(next))) => {
                let branches = next;
                tree_stack.push(BranchesInstruction::Right);
                tree_stack.push(BranchesInstruction::NextTree(&branches.rhs));
                tree_stack.push(BranchesInstruction::Left);
                tree_stack.push(BranchesInstruction::NextTree(&branches.lhs));
                current_branch.push(true);
            }
            Some(BranchesInstruction::Left) => {
                current_branch.pop();
                current_branch.push(false);
            }
            Some(BranchesInstruction::Right) => {
                current_branch.pop();
            }
        };
    }
}

fn frequencies(message: &String) -> HashMap<char, i32> {
    let mut rtn = HashMap::new();
    for c in message.chars() {
        rtn.entry(c).and_modify(|e| *e += 1).or_insert(0);
    }
    rtn
}

pub fn huff_tree(message: &String) -> HuffTree {
    let mut trees: Vec<(HuffTree, i32)> = frequencies(message)
        .iter()
        .map(|w| (HuffTree::Leaf(*w.0), *w.1))
        .collect();
    if trees.is_empty() {
        return HuffTree::NonLeaf(Box::new(TreeRoot {
            lhs: HuffTree::Leaf('a'),
            rhs: HuffTree::Leaf('b'),
        }));
    }
    while trees.len() > 1 {
        trees.sort_by(|a, b| a.1.cmp(&b.1).reverse());
        let (lhs, lweight) = trees.pop().unwrap();
        let (rhs, rweight) = trees.pop().unwrap();
        let combined = HuffTree::NonLeaf(Box::new(TreeRoot { lhs, rhs }));
        trees.push((combined, lweight + rweight));
    };
    //TODO: gotta do something in the case of a single leaf (with trivial branch)
    trees.pop().unwrap().0
}
