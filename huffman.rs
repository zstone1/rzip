use std::collections::HashMap;
fn main() {
    let htree = TreeRoot { 
        lhs : HuffTree::NonLeaf(Box::new(TreeRoot {
            lhs : HuffTree::Leaf('a'),
            rhs : HuffTree::Leaf('z'),
        })),        
        rhs : HuffTree::NonLeaf(Box::new(TreeRoot {
            lhs : HuffTree::Leaf('b'),
            rhs : HuffTree::Leaf('c'),
        }))
    };
    let decoded = decode(&htree, &vec![true, false, true, true, false, false]);
    println!("Hello, htree {:?}", htree);
    println!("Hello, decoded {:?}", decoded);
    println!("branches {:?}", branches(&HuffTree::NonLeaf(Box::new(htree))));


}

fn decode(entry : &TreeRoot<char>, bytes : &Vec<bool>) -> String {
  let mut ret = "".to_string();
  let mut tree_pos : &TreeRoot<char> = entry;
  for b in bytes {
      let branch = if *b {&tree_pos.lhs} else {&tree_pos.rhs};
      let next = match branch {
          HuffTree :: NonLeaf(root) => root,
          HuffTree :: Leaf(lval) => {
              ret.push(*lval);
              entry
          }
      };
      tree_pos = next;
  }
  ret
}

enum BranchesInstruction<'a> {
    NextTree(&'a HuffTree<char>),
    Left,
    Right,
}

fn branches(entry : &HuffTree<char>) -> HashMap<char, Vec<bool>> {
  let mut rtn : HashMap<char, Vec<bool>> = HashMap::new();
  let mut current_branch : Vec<bool> = Vec::new();
  let mut tree_stack : Vec<BranchesInstruction> = vec![BranchesInstruction::NextTree(entry)];
  loop {
      let current_tree = tree_stack.pop();
      match current_tree {
          None => { 
              break;
          },
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
          },
          Some(BranchesInstruction::Right) => {
              current_branch.pop();
          }
      };
  }
  rtn
}

#[derive(Debug)]
struct TreeRoot<T> {lhs: HuffTree<T>, rhs: HuffTree<T>}

#[derive(Debug)]
enum HuffTree<T> {
    Leaf(T),
    NonLeaf(Box<TreeRoot<T>>),
}
