pub mod huffman;
mod test;

use crate::huffman::{TreeRoot, HuffTree, encode, decode};
fn main(){
    let htree = HuffTree :: NonLeaf(Box::new(TreeRoot {
        lhs: HuffTree::NonLeaf(Box::new(TreeRoot {
            lhs: HuffTree::Leaf('a'),
            rhs: HuffTree::Leaf('z'),
        })),
        rhs: HuffTree::NonLeaf(Box::new(TreeRoot {
            lhs: HuffTree::Leaf('b'),
            rhs: HuffTree::Leaf('c'),
        })),
    }));
    let message = String::from("azbbc");
    assert_eq!(decode(&htree, &encode(&htree, &message)), message);
}
